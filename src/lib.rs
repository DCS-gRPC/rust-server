#![allow(dead_code)]
#![recursion_limit = "256"]

mod chat;
#[cfg(feature = "hot-reload")]
mod hot_reload;
pub mod rpc;
mod server;
mod shutdown;
mod stream;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::shutdown::Shutdown;
use chat::Chat;
use dcs_module_ipc::IPC;
use mlua::{prelude::*, LuaSerdeExt};
use mlua::{Function, Value};
use once_cell::sync::Lazy;
use rpc::dcs::Event;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;

static INITIALIZED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static SERVER: Lazy<RwLock<Option<Server>>> = Lazy::new(|| RwLock::new(None));

struct Server {
    ipc_mission: IPC<Event>,
    ipc_hook: IPC<()>,
    chat: Chat,
    runtime: Runtime,
    shutdown: Shutdown,
    after_shutdown: oneshot::Sender<()>,
}

pub fn init(lua: &Lua) -> LuaResult<String> {
    // get lfs.writedir()
    let write_dir: String = {
        let globals = lua.globals();
        let lfs: LuaTable = globals.get("lfs")?;
        lfs.call_method("writedir", ())?
    };

    if INITIALIZED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .unwrap_or(true)
    {
        return Ok(write_dir);
    }

    // init logging
    use log::LevelFilter;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};

    let log_file = write_dir.clone() + "Logs/gRPC.log";

    let requests = FileAppender::builder()
        .append(false)
        .build(log_file)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(requests)))
        .logger(Logger::builder().build("dcs_grpc_server", LevelFilter::Debug))
        .logger(Logger::builder().build("tokio", LevelFilter::Debug))
        .logger(Logger::builder().build("tonic", LevelFilter::Debug))
        .build(Root::builder().appender("file").build(LevelFilter::Off))
        .unwrap();

    log4rs::init_config(config).unwrap();

    Ok(write_dir)
}

#[no_mangle]
pub fn start(lua: &Lua, is_mission_env: bool) -> LuaResult<()> {
    {
        if !is_mission_env || SERVER.read().unwrap().is_some() {
            return Ok(());
        }
    }

    let _write_dir = init(lua)?;

    log::info!("Starting ...");

    let ipc_mission = IPC::new();
    let ipc_hook = IPC::new();
    let chat = Chat::new();
    let (tx, rx) = oneshot::channel();
    let shutdown = Shutdown::new();

    // Spawn an executor thread that waits for the shutdown signal
    let runtime = Runtime::new()?;
    runtime.spawn(crate::server::run(
        ipc_mission.clone(),
        ipc_hook.clone(),
        chat.clone(),
        shutdown.handle(),
        rx,
    ));

    let mut server = SERVER.write().unwrap();
    *server = Some(Server {
        ipc_mission,
        ipc_hook,
        chat,
        runtime,
        shutdown,
        after_shutdown: tx,
    });

    log::info!("Started");

    Ok(())
}

#[no_mangle]
pub fn stop(_: &Lua, _: ()) -> LuaResult<()> {
    log::info!("Stopping ...");

    if let Some(Server {
        runtime,
        shutdown,
        after_shutdown,
        ..
    }) = SERVER.write().unwrap().take()
    {
        // graceful shutdown
        runtime.block_on(shutdown.shutdown());
        let _ = after_shutdown.send(());

        // shutdown the async runtime, again give everything another 5 secs before forecefully
        // killing everything
        runtime.shutdown_timeout(Duration::from_secs(5));
    }

    log::info!("Stopped");

    Ok(())
}

#[no_mangle]
pub fn next(lua: &Lua, (env, callback): (i32, Function)) -> LuaResult<bool> {
    if let Some(Server {
        ref ipc_mission,
        ref ipc_hook,
        ..
    }) = *SERVER.read().unwrap()
    {
        let next = match env {
            1 => ipc_mission.try_next(),
            2 => ipc_hook.try_next(),
            _ => return Ok(false),
        };

        if let Some(mut next) = next {
            let method = next.method().to_string();
            let params = next
                .params(lua)
                .map_err(|err| mlua::Error::ExternalError(Arc::new(Error::SerializeParams(err))))?;

            if let Some(params) = &params {
                log::debug!(
                    "Sending request `{}`: {}",
                    method,
                    pretty_print_value(params.clone(), 0)?
                );
            } else {
                log::debug!("Sending request `{}`", method,);
            }

            let result: LuaTable = callback.call((method.as_str(), params))?;
            let error: Option<LuaTable> = result.get("error")?;

            if let Some(error) = error {
                let message: String = error.get("message")?;
                let kind: Option<String> = error.get("type")?;

                next.error(message, kind);
                return Ok(true);
            }

            let res: Value<'_> = result.get("result")?;
            log::debug!("Receiving: {}", pretty_print_value(res.clone(), 0)?);

            next.success(lua, &res).map_err(|err| {
                mlua::Error::ExternalError(Arc::new(Error::DeserializeResult {
                    err,
                    method,
                    result: pretty_print_value(res, 0)
                        .unwrap_or_else(|err| format!("failed to pretty print result: {}", err)),
                }))
            })?;

            return Ok(true);
        }
    }

    Ok(false)
}

#[no_mangle]
pub fn event(lua: &Lua, event: Value) -> LuaResult<()> {
    let event: Event = match lua.from_value(event) {
        Ok(event) => event,
        Err(err) => {
            log::error!("failed to deserialize event: {}", err);
            // In certain cases DCS crashes when we return an error back to Lua here (see
            // https://github.com/DCS-gRPC/rust-server/issues/19), which we are working around
            // by intercepting and logging the error instead.
            return Ok(());
        }
    };

    if let Some(Server {
        ref ipc_mission,
        ref runtime,
        ..
    }) = *SERVER.read().unwrap()
    {
        log::debug!("Received event: {:#?}", event);
        runtime.block_on(ipc_mission.event(event));
    }

    Ok(())
}

#[no_mangle]
pub fn on_chat_message(
    _: &Lua,
    (player_id, message, all): (u32, String, bool),
) -> LuaResult<String> {
    log::debug!("on_chat_message {} {} {}", player_id, message, all);

    if let Some(Server {
        ref chat,
        ref runtime,
        ..
    }) = *SERVER.read().unwrap()
    {
        Ok(runtime.block_on(chat.handle_message(player_id, message, all)))
    } else {
        Ok(message)
    }
}

#[no_mangle]
pub fn log_error(_: &Lua, err: String) -> LuaResult<()> {
    log::error!("{}", err);
    Ok(())
}

#[no_mangle]
pub fn log_warning(_: &Lua, err: String) -> LuaResult<()> {
    log::warn!("{}", err);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize params: {0}")]
    DeserializeParams(#[source] mlua::Error),
    #[error("Failed to deserialize result for method {method}: {err}\n{result}")]
    DeserializeResult {
        #[source]
        err: mlua::Error,
        method: String,
        result: String,
    },
    #[error("Failed to serialize params: {0}")]
    SerializeParams(#[source] mlua::Error),
}

#[cfg(feature = "hot-reload")]
#[mlua::lua_module]
pub fn dcs_grpc_server_hot_reload(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", lua.create_function(hot_reload::start)?)?;
    exports.set("stop", lua.create_function(hot_reload::stop)?)?;
    exports.set("next", lua.create_function(hot_reload::next)?)?;
    exports.set("event", lua.create_function(hot_reload::event)?)?;
    exports.set(
        "on_chat_message",
        lua.create_function(hot_reload::on_chat_message)?,
    )?;
    exports.set("log_error", lua.create_function(hot_reload::log_error)?)?;
    exports.set("log_warning", lua.create_function(hot_reload::log_warning)?)?;
    Ok(exports)
}

#[cfg(not(feature = "hot-reload"))]
#[mlua::lua_module]
pub fn dcs_grpc_server(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", lua.create_function(start)?)?;
    exports.set("stop", lua.create_function(stop)?)?;
    exports.set("next", lua.create_function(next)?)?;
    exports.set("event", lua.create_function(event)?)?;
    exports.set("on_chat_message", lua.create_function(on_chat_message)?)?;
    exports.set("log_error", lua.create_function(log_error)?)?;
    exports.set("log_warning", lua.create_function(log_warning)?)?;
    Ok(exports)
}

fn pretty_print_value(val: Value, indent: usize) -> LuaResult<String> {
    Ok(match val {
        Value::Nil => "nil".to_string(),
        Value::Boolean(v) => v.to_string(),
        Value::LightUserData(_) => String::new(),
        Value::Integer(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => format!("\"{}\"", v.to_str()?),
        Value::Table(t) => {
            let mut s = "{\n".to_string();
            for pair in t.pairs::<Value, Value>() {
                let (key, value) = pair?;
                s += &format!(
                    "{}{} = {},\n",
                    "  ".repeat(indent + 1),
                    pretty_print_value(key, indent + 1)?,
                    pretty_print_value(value, indent + 1)?
                );
            }
            s += &format!("{}}}", "  ".repeat(indent));
            s
        }
        Value::Function(_) => "[function]".to_string(),
        Value::Thread(_) => String::new(),
        Value::UserData(_) => String::new(),
        Value::Error(err) => err.to_string(),
    })
}
