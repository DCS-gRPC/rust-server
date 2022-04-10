use std::fs;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use http_body::Body;
use hyper::{Client, StatusCode, Uri};
use tokio::process::{Child, Command};
use tokio::sync::Mutex;
use tonic::body::BoxBody;
use tonic::codegen::http::{self};
use tonic::codegen::{Never, Service};
use tonic::{transport, Status};

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
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                log::warn!("Error reading plugin directory entry: {}", err);
                continue;
            }
        };

        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        let exec = path.join("plugin.exe");
        if !exec.is_file() {
            log::warn!("Plugin at `{}` not found", exec.to_string_lossy());
            continue;
        }

        log::info!("Loaded Plugin `{}`", name,);

        plugins.push(Plugin(Arc::new(PluginInner {
            name,
            exec,
            port: next_port,
            child: Mutex::new(None),
        })));
        next_port += 1;
    }

    plugins
}

#[derive(Clone)]
pub struct Plugin(Arc<PluginInner>);

pub struct PluginInner {
    name: String,
    exec: PathBuf,
    port: u16,
    child: Mutex<Option<Child>>,
}

impl Plugin {
    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn port(&self) -> u16 {
        self.0.port
    }

    pub async fn start(&self, server_port: u16) {
        let mut child = self.0.child.lock().await;
        if child.is_some() {
            return;
        }

        let mut command = Command::new(&self.0.exec);
        command.env("SERVER_PORT", server_port.to_string());
        command.env("PLUGIN_PORT", self.0.port.to_string());

        // don't start a console window for the process
        command.creation_flags(winapi::um::winbase::CREATE_NO_WINDOW);

        *child = match command.spawn() {
            Ok(child) => Some(child),
            Err(err) => {
                log::error!("failed to start plugin `{}`: {}", self.0.name, err);
                return;
            }
        };
    }

    pub async fn stop(&self) {
        let mut child = self.0.child.lock().await;
        if let Some(mut child) = child.take() {
            if let Err(err) = child.kill().await {
                log::error!("error killing plugin `{}`: {}", self.0.name, err);
            }
        }
    }
}

impl Service<http::Request<transport::Body>> for Plugin {
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

    fn call(&mut self, req: http::Request<transport::Body>) -> Self::Future {
        let (mut parts, body) = req.into_parts();

        // Point request to plugin server
        parts.uri = Uri::builder()
            .scheme("http")
            .authority(format!("localhost:{}", self.port()))
            .path_and_query(
                parts
                    .uri
                    .path_and_query()
                    .cloned()
                    .unwrap_or_else(|| http::uri::PathAndQuery::from_static("/")),
            )
            .build()
            .unwrap();

        // TODO: re-use client
        let client: Client<_, transport::Body> = Client::builder().http2_only(true).build_http();
        let req = hyper::Request::from_parts(parts, body);
        Box::pin(async move {
            let res = match client.request(req).await {
                Ok(res) => res,
                Err(err) => {
                    log::error!("error forwarding request to plugin: {}", err);
                    return Ok(hyper::Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(
                            transport::Body::empty()
                                .map_err(|_| unreachable!())
                                .boxed_unsync(),
                        )
                        .unwrap());
                }
            };
            let (parts, body) = res.into_parts();
            let body = body
                .map_err(|err| {
                    Status::internal(format!("error reading response body from plugin: {}", err))
                })
                .boxed_unsync();
            Ok(hyper::Response::from_parts(parts, body))
        })
    }
}

// impl tonic::server::UnaryService<Vec<u8>> for Plugin {
//     type Response = Vec<u8>;
//     type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
//     fn call(&mut self, request: tonic::Request<Vec<u8>>) -> Self::Future {
//         let plugin = self.clone();
//         let fut = async move {
//             let data =
//                 tokio::task::spawn_blocking(move || unsafe { asd(plugin, request.into_inner()) })
//                     .await
//                     .map_err(|err| {
//                         Status::new(Code::Unknown, format!("failed to spawn blocking: {}", err))
//                     })??;

//             Ok(tonic::Response::new(data))
//         };
//         Box::pin(fut)
//     }
// }

// unsafe fn asd(plugin: Plugin, request: Vec<u8>) -> Result<Vec<u8>, Status> {
//     log::info!("request: {:?}", request);

//     let call: Symbol<
//         unsafe extern "C" fn(
//             method_ptr: *const c_char,
//             method_len: usize,
//             request_ptr: *const u8,
//             request_len: usize,
//             response_ptr: *mut *mut u8,
//         ) -> usize,
//     > = plugin.inner.lib.get(b"call").map_err(|err| {
//         Status::internal(format!(
//             "failed to get `call` function from `{}` plugin: {}",
//             plugin.name(),
//             err
//         ))
//     })?;

//     let method = "example.greeter.v0.GreeterService/Greet";

//     log::info!("CALL");
//     let data = std::panic::catch_unwind(|| {
//         let mut response = MaybeUninit::uninit();
//         let len = call(
//             method.as_ptr() as *const c_char,
//             method.len(),
//             request.as_ptr(),
//             request.len(),
//             response.as_mut_ptr(),
//         );
//         if len == 0 {
//             unimplemented!("no response")
//         } else {
//             let response = response.assume_init();
//             let data = Box::from_raw(std::slice::from_raw_parts_mut(response, len));
//             let data = Vec::from(data);
//             data
//         }
//     })
//     .map_err(|err| {
//         if let Ok(err) = err.downcast::<String>() {
//             Status::internal(format!(
//                 "calling plugin `{}` panicked: {:?}",
//                 plugin.name(),
//                 err
//             ))
//         } else {
//             Status::internal(format!("calling plugin `{}` panicked", plugin.name()))
//         }
//     })?;
//     log::info!("CALLED");
//     Ok(data)
// }

// #[derive(Default, Clone)]
// struct RawCodec;

// impl tonic::codec::Codec for RawCodec {
//     type Encode = Vec<u8>;
//     type Decode = Vec<u8>;

//     type Encoder = RawCodec;
//     type Decoder = RawCodec;

//     fn encoder(&mut self) -> Self::Encoder {
//         self.clone()
//     }

//     fn decoder(&mut self) -> Self::Decoder {
//         self.clone()
//     }
// }

// impl tonic::codec::Encoder for RawCodec {
//     type Item = Vec<u8>;
//     type Error = Status;

//     fn encode(
//         &mut self,
//         item: Self::Item,
//         buf: &mut tonic::codec::EncodeBuf<'_>,
//     ) -> Result<(), Self::Error> {
//         buf.put(&*item);
//         Ok(())
//     }
// }

// impl tonic::codec::Decoder for RawCodec {
//     type Item = Vec<u8>;
//     type Error = Status;

//     fn decode(
//         &mut self,
//         buf: &mut tonic::codec::DecodeBuf<'_>,
//     ) -> Result<Option<Self::Item>, Self::Error> {
//         let mut data = vec![0; buf.remaining()];
//         buf.copy_to_slice(&mut data);
//         Ok(Some(data))
//     }
// }

// #[derive(Debug, thiserror::Error)]
// pub enum PluginError {
//     #[error(transparent)]
//     Io(#[from] std::io::Error),
//     #[error(transparent)]
//     Lib(#[from] libloading::Error),
// }

// unsafe extern "C" fn request(
//     method_ptr: *const c_char,
//     method_len: usize,
//     request_ptr: *const u8,
//     request_len: usize,
//     response_ptr: *mut *mut u8,
// ) -> usize {
//     let server = crate::SERVER.read().unwrap();
//     let server = if let Some(server) = &*server {
//         server
//     } else {
//         return 0;
//     };

//     let mut mission_rpc = MissionRpc::new(
//         server.ipc_mission().clone(),
//         server.stats().clone(),
//         server.shutdown_handle(),
//     );
//     let mut hook_rpc = HookRpc::new(
//         server.ipc_hook().clone(),
//         server.stats().clone(),
//         server.shutdown_handle(),
//     );
//     let services = DcsServices::new(mission_rpc, hook_rpc, Arc::new(Vec::new()));

//     let method = std::slice::from_raw_parts(method_ptr as *const u8, method_len);
//     let method = std::str::from_utf8(method).unwrap_or_default();

//     let request = std::slice::from_raw_parts(request_ptr, request_len);

//     let response = server.block_on(asdasdasd(services, method, request));

//     let mut response = ManuallyDrop::new(response.into_boxed_slice());
//     *(response_ptr.as_mut().unwrap()) = response.as_mut_ptr();
//     response.len()
// }

// async fn asdasdasd(
//     services: crate::services::DcsServices,
//     method: &str,
//     request: &[u8],
// ) -> Vec<u8> {
//     // let request = http::Request::builder().uri(format!("http://localhost/{}", method))
//     // .body(body)

//     let mut client = tonic::client::Grpc::new(services);
//     let path = http::uri::PathAndQuery::from_maybe_shared(method.to_string()).unwrap();
//     let response = client
//         // TODO: get rid of to_vec()
//         .unary(tonic::Request::new(request.to_vec()), path, RawCodec)
//         .await
//         .unwrap()
//         .into_inner();

//     response
// }

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
