use std::fs;
use std::mem::MaybeUninit;
use std::path::Path;

use libloading::{Library, Symbol};

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

pub struct Plugin {
    name: String,
    lib: Library,
    port: u16,
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

        Ok(Plugin { name, lib, port: 0 })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn start(&self, dcs_grpc_port: u16) {
        unsafe {
            let start: Symbol<unsafe extern "C" fn(dcs_grpc_port: u16, plugin_port: u16)> =
                match self.lib.get(b"start") {
                    Ok(s) => s,
                    Err(err) => {
                        log::error!("Error starting plugin `{}`: {}", self.name, err);
                        return;
                    }
                };
            start(dcs_grpc_port, self.port);
        }
    }

    pub fn stop(&self) {
        unsafe {
            let stop: Symbol<unsafe extern "C" fn()> = match self.lib.get(b"stop") {
                Ok(s) => s,
                Err(err) => {
                    log::error!("Error stopping plugin `{}`: {}", self.name, err);
                    return;
                }
            };
            stop();
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Lib(#[from] libloading::Error),
}
