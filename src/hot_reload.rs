///! This module is a wrapper around all exposed Lua methods which are forwarded to a dynamically
///! loaded dcs_grpc_server.dll. Upon calling the `stop()` method, the library is unloaded, and re-
///! loaded during the next `start()` call.
use std::sync::{Arc, RwLock};

use libloading::{Library, Symbol};
use mlua::prelude::*;
use mlua::{Function, Value};
use once_cell::sync::Lazy;

static LIBRARY: Lazy<RwLock<Option<Library>>> = Lazy::new(|| RwLock::new(None));

pub fn start(lua: &Lua, is_mission_env: bool) -> LuaResult<()> {
    let write_dir = super::init(lua)?;
    let lib_path = write_dir.clone() + "Mods/Tech/DCS-gRPC/dcs_grpc_server.dll";

    let new_lib = unsafe { Library::new(lib_path) }.map_err(|err| {
        log::error!("Load: {}", err);
        mlua::Error::ExternalError(Arc::new(err))
    })?;
    let mut lib = LIBRARY.write().unwrap();
    let lib = lib.get_or_insert(new_lib);

    let f: Symbol<fn(lua: &Lua, is_mission_env: bool) -> LuaResult<()>> = unsafe {
        lib.get(b"start")
            .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
    };
    let result = f(lua, is_mission_env);

    result
}

pub fn stop(lua: &Lua, arg: ()) -> LuaResult<()> {
    if let Some(lib) = LIBRARY.write().unwrap().take() {
        let f: Symbol<fn(lua: &Lua, arg: ()) -> LuaResult<()>> = unsafe {
            lib.get(b"stop")
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
        };
        f(lua, arg)
    } else {
        Ok(())
    }
}

pub fn next(lua: &Lua, arg: (i32, Function)) -> LuaResult<bool> {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        let f: Symbol<fn(lua: &Lua, arg: (i32, Function)) -> LuaResult<bool>> = unsafe {
            lib.get(b"next")
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
        };
        f(lua, arg)
    } else {
        Ok(false)
    }
}

pub fn event(lua: &Lua, event: Value) -> LuaResult<()> {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        let f: Symbol<fn(lua: &Lua, event: Value) -> LuaResult<()>> = unsafe {
            lib.get(b"event")
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
        };
        f(lua, event)
    } else {
        Ok(())
    }
}

pub fn on_chat_message(
    lua: &Lua,
    (player_id, message, all): (u32, String, bool),
) -> LuaResult<String> {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        let f: Symbol<fn(lua: &Lua, arg: (u32, String, bool)) -> LuaResult<String>> = unsafe {
            lib.get(b"on_chat_message")
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
        };
        f(lua, (player_id, message, all))
    } else {
        Ok(message)
    }
}

pub fn log_error(lua: &Lua, err: String) -> LuaResult<()> {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        let f: Symbol<fn(lua: &Lua, err: String) -> LuaResult<()>> = unsafe {
            lib.get(b"log_error")
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
        };
        f(lua, err)
    } else {
        Ok(())
    }
}

pub fn log_warning(lua: &Lua, err: String) -> LuaResult<()> {
    if let Some(ref lib) = *LIBRARY.read().unwrap() {
        let f: Symbol<fn(lua: &Lua, err: String) -> LuaResult<()>> = unsafe {
            lib.get(b"log_error")
                .map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?
        };
        f(lua, err)
    } else {
        Ok(())
    }
}
