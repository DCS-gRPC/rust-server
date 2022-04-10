use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::sync::RwLock;

use once_cell::sync::Lazy;
use tonic::transport::NamedService;

use crate::greeter::v0::greeter_service_server::GreeterServiceServer;
use crate::Plugin;

// extern "C" {
//     fn log_error(err: *const c_char);
// }

/// Get the name of the service the plugin exposes. A pointer to the name is written into the given
/// `name`. The caller must not deallocate the name. The length of the name is returned.
#[no_mangle]
pub unsafe extern "C" fn name(name: *mut *const u8) -> usize {
    *name = GreeterServiceServer::<super::Plugin>::NAME.as_ptr();
    GreeterServiceServer::<super::Plugin>::NAME.len()
}

/// Returns the DCS-gRPC version the plugin is compatible with. The most significant 16 bits are the
/// major version number. The least significant 16 bits are the minor version number.
#[no_mangle]
pub extern "C" fn api_version() -> i32 {
    let mut b = [0u8; 4];
    b[..2].copy_from_slice(0i16.to_be_bytes().as_ref()); // major version
    b[2..].copy_from_slice(5i16.to_be_bytes().as_ref()); // minor version
    i32::from_be_bytes(b)
}

static PLUGIN: Lazy<RwLock<Option<Plugin>>> = Lazy::new(|| RwLock::new(None));

type RequestFn = unsafe extern "C" fn(
    method_ptr: *const c_char,
    method_len: usize,
    request_ptr: *const u8,
    request_len: usize,
    response_ptr: *mut *mut u8,
) -> usize;

#[no_mangle]
pub extern "C" fn start(request_fn: RequestFn) {
    let mut shutdown = PLUGIN.write().unwrap();
    if shutdown.is_some() {
        // already started
        return;
    }

    *shutdown = Some(Plugin::new(request_fn));
}

#[no_mangle]
pub extern "C" fn stop() {
    PLUGIN.write().unwrap().take();
}

#[no_mangle]
pub unsafe extern "C" fn call(
    method_ptr: *const c_char,
    method_len: usize,
    request_ptr: *const u8,
    request_len: usize,
    response_ptr: *mut *mut u8,
) -> usize {
    let plugin = PLUGIN.read().unwrap();
    let plugin = match &*plugin {
        Some(plugin) => plugin,
        None => return 0,
    };

    let method = std::slice::from_raw_parts(method_ptr as *const u8, method_len);
    let method = std::str::from_utf8(method).unwrap_or_default();

    let request = std::slice::from_raw_parts(request_ptr, request_len);
    let response = super::handle_call(plugin, method, request);
    let mut response = ManuallyDrop::new(response.into_boxed_slice());
    *(response_ptr.as_mut().unwrap()) = response.as_mut_ptr();
    response.len()
}

struct Client(RequestFn);

impl tonic::client::GrpcService<tonic::body::BoxBody> for Client {
    type ResponseBody = tonic::body::BoxBody;
    type Error = std::convert::Infallible;
    type Future =
        std::future::Ready<Result<tonic::codegen::http::Response<Self::ResponseBody>, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(
        &mut self,
        request: tonic::codegen::http::Request<tonic::body::BoxBody>,
    ) -> Self::Future {
        use futures_util::stream::TryStreamExt;

        let fut = async move {
            let (parts, body) = request.into_parts();
            // let bytes = hyper::body::to_bytes(body).await?;
            let entire_body = body
                .try_fold(Vec::new(), |mut data, chunk| async move {
                    data.extend_from_slice(&chunk);
                    Ok(data)
                })
                .await;

            todo!()
        };

        todo!()
    }
}
