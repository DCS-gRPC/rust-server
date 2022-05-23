///! This module is a wrapper around all exposed Lua methods which are forwarded to a dynamically
///! loaded dcs_grpc.dll. Upon calling the `stop()` method, the library is unloaded, and re-
///! loaded during the next `start()` call.
use std::path::PathBuf;
use std::sync::RwLock;

use crate::Config;
use libloading::{Library, Symbol};
use mlua::prelude::*;
use mlua::{Function, Value};
use once_cell::sync::Lazy;

static LIBRARY: Lazy<RwLock<Option<Library>>> = Lazy::new(|| RwLock::new(None));

pub fn start(lua: &Lua, config: Config) {
    let lib_path = {
        let mut lib_path = PathBuf::from(&config.dll_path);
        lib_path.push("dcs_grpc.dll");
        lib_path
    };

    let new_lib = match unsafe { Library::new(lib_path) } {
        Ok(new_lib) => new_lib,
        Err(err) => {
            log::error!("Failed to load `dcs_grpc.dll`: {}", err);
            return;
        }
    };
    let mut lib = LIBRARY.write().unwrap();
    let lib = lib.get_or_insert(new_lib);

    match unsafe { lib.get::<Symbol<fn(lua: &Lua, config: Config)>>(b"start") } {
        Ok(f) => f(lua, config),
        Err(err) => {
            log::error!("Failed to get `start` method: {}", err);
        }
    }
}

pub fn stop(lua: &Lua, arg: ()) {
    if let Some(lib) = LIBRARY.write().unwrap().take() {
        match unsafe { lib.get::<Symbol<fn(lua: &Lua, arg: ())>>(b"stop") } {
            Ok(f) => f(lua, arg),
            Err(err) => {
                log::error!("Failed to get `stop` method: {}", err);
            }
        }
    }
}

pub fn next(lua: &Lua, arg: (i32, Function)) -> bool {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        match unsafe { lib.get::<Symbol<fn(lua: &Lua, arg: (i32, Function)) -> bool>>(b"next") } {
            Ok(f) => f(lua, arg),
            Err(err) => {
                log::error!("Failed to get `next` method: {}", err);
                return false;
            }
        }
    } else {
        false
    }
}

pub fn event(lua: &Lua, event: Value) {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        match unsafe { lib.get::<Symbol<fn(lua: &Lua, event: Value)>>(b"event") } {
            Ok(f) => f(lua, event),
            Err(err) => {
                log::error!("Failed to get `event` method: {}", err);
                return;
            }
        }
    }
}

pub fn log_error(lua: &Lua, err: String) {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        match unsafe { lib.get::<Symbol<fn(lua: &Lua, err: String)>>(b"log_error") } {
            Ok(f) => f(lua, err),
            Err(err) => {
                log::error!("Failed to get `log_error` method: {}", err);
                return;
            }
        }
    }
}

pub fn log_warning(lua: &Lua, msg: String) {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        match unsafe { lib.get::<Symbol<fn(lua: &Lua, err: String)>>(b"log_warning") } {
            Ok(f) => f(lua, msg),
            Err(err) => {
                log::error!("Failed to get `log_warning` method: {}", err);
                return;
            }
        }
    }
}

pub fn log_info(lua: &Lua, msg: String) {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        match unsafe { lib.get::<Symbol<fn(lua: &Lua, msg: String)>>(b"log_info") } {
            Ok(f) => f(lua, msg),
            Err(err) => {
                log::error!("Failed to get `log_info` method: {}", err);
                return;
            }
        }
    }
}

pub fn log_debug(lua: &Lua, msg: String) {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        match unsafe { lib.get::<Symbol<fn(lua: &Lua, msg: String)>>(b"log_debug") } {
            Ok(f) => f(lua, msg),
            Err(err) => {
                log::error!("Failed to get `log_debug` method: {}", err);
                return;
            }
        }
    }
}
