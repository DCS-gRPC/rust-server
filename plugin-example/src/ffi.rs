use std::ffi::CString;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use tokio::sync::oneshot;
use tonic::transport::NamedService;

use crate::greeter::v0::greeter_service_server::GreeterServiceServer;

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

static SHUTDOWN: Lazy<Mutex<Option<oneshot::Sender<()>>>> = Lazy::new(|| Mutex::new(None));

#[no_mangle]
pub extern "C" fn start(dcs_grpc_port: u16, plugin_port: u16) {
    let mut shutdown = SHUTDOWN.lock().unwrap();
    if shutdown.is_some() {
        // already started
        return;
    }

    let (tx, rx) = oneshot::channel();
    std::thread::spawn(move || {
        if let Err(err) = super::start(dcs_grpc_port, plugin_port, rx) {
            if let Ok(s) = CString::new(err.to_string()) {
                // unsafe {
                //     log_error(s.as_ptr());
                // }
                panic!("{}", err);
            }
        }
    });

    *shutdown = Some(tx);
    drop(shutdown);
}

#[no_mangle]
pub extern "C" fn stop() {
    if let Some(shutdown) = SHUTDOWN.lock().unwrap().take() {
        shutdown.send(()).ok();
    }
}
