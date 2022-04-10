use std::convert::Infallible;
use std::ffi::c_void;
use std::fs;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::os::raw::c_char;
use std::path::Path;
use std::sync::Arc;

use crate::rpc::{HookRpc, MissionRpc};
use crate::services::DcsServices;
use bytes::{Buf, BufMut};
use futures_util::TryFutureExt;
use http_body::Body;
use libloading::{Library, Symbol};
use tonic::body::BoxBody;
use tonic::codegen::http::{self, StatusCode};
use tonic::codegen::{BoxFuture, Never, Service};
use tonic::transport::{self, NamedService};
use tonic::{Code, Status};

pub fn load(dir: impl AsRef<Path>) -> Vec<Plugin> {
    let mut plugins = Vec::new();
    let mut next_port = 50500;
    let dir = match fs::read_dir(dir.as_ref()) {
        Ok(dir) => dir,
        Err(err) => {
            log::error!("Failed to read plugins directory: {}", err);
            return plugins;
        }
    };

    for entry in dir {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(err) => {
                log::warn!("Error reading plugin directory entry: {}", err);
                continue;
            }
        };

        if !path.is_file() {
            continue;
        }

        if path.extension() != Some("dll".as_ref()) {
            continue;
        }

        match unsafe { Plugin::load(&path) } {
            Ok(mut plugin) => {
                plugin.port = next_port;
                next_port += 1;
                plugins.push(plugin)
            }
            Err(err) => {
                log::warn!(
                    "Error loading plugin `{}`: {}",
                    path.as_os_str().to_string_lossy(),
                    err
                );
            }
        }
    }

    plugins
}

#[derive(Clone)]
pub struct Plugin {
    inner: Arc<PluginInner>,
    port: u16,
}

struct PluginInner {
    name: String,
    lib: Library,
}

impl Plugin {
    unsafe fn load(path: &Path) -> Result<Self, libloading::Error> {
        let lib = Library::new(path)?;

        let api_version: Symbol<unsafe extern "C" fn() -> i32> = lib.get(b"api_version")?;
        let api_version = api_version().to_be_bytes();
        let major_version = i16::from_be_bytes(api_version[..2].try_into().unwrap());
        let minor_version = i16::from_be_bytes(api_version[2..].try_into().unwrap());

        let mut name_ptr = MaybeUninit::uninit();
        let name: Symbol<unsafe extern "C" fn(name: *mut *const u8) -> usize> = lib.get(b"name")?;
        let len = name(name_ptr.as_mut_ptr());
        let bytes = std::slice::from_raw_parts(name_ptr.assume_init(), len);
        let name = String::from_utf8_lossy(bytes).to_string();

        log::info!(
            "Loaded Plugin `{}` (API version {}.{})",
            name,
            major_version,
            minor_version
        );

        // TODO: validate version against `env!("CARGO_PKG_VERSION")`

        Ok(Plugin {
            inner: Arc::new(PluginInner { name, lib }),
            port: 0,
        })
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn start(&self) {
        unsafe {
            let start: Symbol<
                unsafe extern "C" fn(
                    request: unsafe extern "C" fn(
                        method_ptr: *const c_char,
                        method_len: usize,
                        request_ptr: *const u8,
                        request_len: usize,
                        response_ptr: *mut *mut u8,
                    ) -> usize,
                ),
            > = match self.inner.lib.get(b"start") {
                Ok(s) => s,
                Err(err) => {
                    log::error!("Error starting plugin `{}`: {}", self.inner.name, err);
                    return;
                }
            };

            if let Err(err) = std::panic::catch_unwind(|| {
                start(request);
            }) {
                if let Ok(err) = err.downcast::<String>() {
                    log::error!("starting plugin `{}` panicked: {:?}", self.name(), err);
                } else {
                    log::error!("starting plugin `{}` panicked", self.name());
                }
            }
        }
    }

    pub fn stop(&self) {
        unsafe {
            let stop: Symbol<unsafe extern "C" fn() -> *const c_void> =
                match self.inner.lib.get(b"stop") {
                    Ok(s) => s,
                    Err(err) => {
                        log::error!("Error stopping plugin `{}`: {}", self.inner.name, err);
                        return;
                    }
                };

            if let Err(err) = std::panic::catch_unwind(|| {
                stop();
            }) {
                if let Ok(err) = err.downcast::<String>() {
                    log::error!("stopping plugin `{}` panicked: {:?}", self.name(), err);
                } else {
                    log::error!("stopping plugin `{}` panicked", self.name());
                }
            }
        }
    }
}

impl<B> Service<http::Request<B>> for Plugin
where
    B: Body + Send + 'static,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + Send + 'static,
{
    type Response = http::Response<BoxBody>;
    type Error = Never;
    type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

    #[inline]
    fn poll_ready(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        let plugin = self.clone();
        let fut = async move {
            // let inner = inner.inner;
            // let method = GetMarkPanelsSvc(inner);
            let mut grpc = tonic::server::Grpc::new(RawCodec);
            let res = grpc.unary(plugin, req).await;
            Ok(res)
        };
        Box::pin(fut)
    }
}

impl tonic::server::UnaryService<Vec<u8>> for Plugin {
    type Response = Vec<u8>;
    type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
    fn call(&mut self, request: tonic::Request<Vec<u8>>) -> Self::Future {
        let plugin = self.clone();
        let fut = async move {
            let data =
                tokio::task::spawn_blocking(move || unsafe { asd(plugin, request.into_inner()) })
                    .await
                    .map_err(|err| {
                        Status::new(Code::Unknown, format!("failed to spawn blocking: {}", err))
                    })??;

            Ok(tonic::Response::new(data))
        };
        Box::pin(fut)
    }
}

unsafe fn asd(plugin: Plugin, request: Vec<u8>) -> Result<Vec<u8>, Status> {
    log::info!("request: {:?}", request);

    let call: Symbol<
        unsafe extern "C" fn(
            method_ptr: *const c_char,
            method_len: usize,
            request_ptr: *const u8,
            request_len: usize,
            response_ptr: *mut *mut u8,
        ) -> usize,
    > = plugin.inner.lib.get(b"call").map_err(|err| {
        Status::internal(format!(
            "failed to get `call` function from `{}` plugin: {}",
            plugin.name(),
            err
        ))
    })?;

    let method = "example.greeter.v0.GreeterService/Greet";

    log::info!("CALL");
    let data = std::panic::catch_unwind(|| {
        let mut response = MaybeUninit::uninit();
        let len = call(
            method.as_ptr() as *const c_char,
            method.len(),
            request.as_ptr(),
            request.len(),
            response.as_mut_ptr(),
        );
        if len == 0 {
            unimplemented!("no response")
        } else {
            let response = response.assume_init();
            let data = Box::from_raw(std::slice::from_raw_parts_mut(response, len));
            let data = Vec::from(data);
            data
        }
    })
    .map_err(|err| {
        if let Ok(err) = err.downcast::<String>() {
            Status::internal(format!(
                "calling plugin `{}` panicked: {:?}",
                plugin.name(),
                err
            ))
        } else {
            Status::internal(format!("calling plugin `{}` panicked", plugin.name()))
        }
    })?;
    log::info!("CALLED");
    Ok(data)
}

#[derive(Default, Clone)]
struct RawCodec;

impl tonic::codec::Codec for RawCodec {
    type Encode = Vec<u8>;
    type Decode = Vec<u8>;

    type Encoder = RawCodec;
    type Decoder = RawCodec;

    fn encoder(&mut self) -> Self::Encoder {
        self.clone()
    }

    fn decoder(&mut self) -> Self::Decoder {
        self.clone()
    }
}

impl tonic::codec::Encoder for RawCodec {
    type Item = Vec<u8>;
    type Error = Status;

    fn encode(
        &mut self,
        item: Self::Item,
        buf: &mut tonic::codec::EncodeBuf<'_>,
    ) -> Result<(), Self::Error> {
        buf.put(&*item);
        Ok(())
    }
}

impl tonic::codec::Decoder for RawCodec {
    type Item = Vec<u8>;
    type Error = Status;

    fn decode(
        &mut self,
        buf: &mut tonic::codec::DecodeBuf<'_>,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let mut data = vec![0; buf.remaining()];
        buf.copy_to_slice(&mut data);
        Ok(Some(data))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Lib(#[from] libloading::Error),
}

unsafe extern "C" fn request(
    method_ptr: *const c_char,
    method_len: usize,
    request_ptr: *const u8,
    request_len: usize,
    response_ptr: *mut *mut u8,
) -> usize {
    let server = crate::SERVER.read().unwrap();
    let server = if let Some(server) = &*server {
        server
    } else {
        return 0;
    };

    let mut mission_rpc = MissionRpc::new(
        server.ipc_mission().clone(),
        server.stats().clone(),
        server.shutdown_handle(),
    );
    let mut hook_rpc = HookRpc::new(
        server.ipc_hook().clone(),
        server.stats().clone(),
        server.shutdown_handle(),
    );
    let services = DcsServices::new(mission_rpc, hook_rpc, Arc::new(Vec::new()));

    let method = std::slice::from_raw_parts(method_ptr as *const u8, method_len);
    let method = std::str::from_utf8(method).unwrap_or_default();

    let request = std::slice::from_raw_parts(request_ptr, request_len);

    let response = server.block_on(asdasdasd(services, method, request));

    let mut response = ManuallyDrop::new(response.into_boxed_slice());
    *(response_ptr.as_mut().unwrap()) = response.as_mut_ptr();
    response.len()
}

async fn asdasdasd(
    services: crate::services::DcsServices,
    method: &str,
    request: &[u8],
) -> Vec<u8> {
    // let request = http::Request::builder().uri(format!("http://localhost/{}", method))
    // .body(body)

    let mut client = tonic::client::Grpc::new(services);
    let path = http::uri::PathAndQuery::from_maybe_shared(method.to_string()).unwrap();
    let response = client
        // TODO: get rid of to_vec()
        .unary(tonic::Request::new(request.to_vec()), path, RawCodec)
        .await
        .unwrap()
        .into_inner();

    response
}

// struct Bla;
// impl<B> tonic::client::GrpcService<B> for crate::services::DcsServices
// where
//     B: Body + Send + 'static,
//     B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>> + Send + 'static,
// {
//     type ResponseBody = tonic::body::BoxBody;
//     type Error = Infallible;
//     type Future = std::future::Ready<Result<http::Response<Self::ResponseBody>, Self::Error>>;

//     fn poll_ready(
//         &mut self,
//         _cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         std::task::Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, request: http::Request<B>) -> Self::Future {
//         todo!()
//     }
// }
