#![allow(dead_code)]
#![recursion_limit = "256"]

mod fps;
#[cfg(feature = "hot-reload")]
mod hot_reload;
pub mod rpc;
mod server;
mod shutdown;
mod stats;
mod stream;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Instant;

use mlua::{prelude::*, LuaSerdeExt};
use mlua::{Function, Value};
use once_cell::sync::Lazy;
use server::{Config, Server};
use stubs::mission::v0::StreamEventsResponse;
use thiserror::Error;

static INITIALIZED: AtomicBool = AtomicBool::new(false);
static SERVER: Lazy<RwLock<Option<Server>>> = Lazy::new(|| RwLock::new(None));

pub fn init(config: &Config) {
    if INITIALIZED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .unwrap_or(true)
    {
        return;
    }

    // init logging
    use log::LevelFilter;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Logger, Root};
    use log4rs::encode::pattern::PatternEncoder;

    let mut log_file = PathBuf::from(&config.write_dir);
    log_file.push("Logs/gRPC.log");

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S%.3f)} {l:<7} {t}: {m}{n}",
        )))
        .append(false)
        .build(log_file)
        .unwrap();

    let level = if config.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    let log_config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(requests)))
        .logger(Logger::builder().build("dcs_grpc", level))
        .logger(Logger::builder().build("tokio", level))
        .logger(Logger::builder().build("tonic", level))
        .logger(Logger::builder().build("dcs_module_ipc", level))
        .build(Root::builder().appender("file").build(LevelFilter::Off))
        .unwrap();

    log4rs::init_config(log_config).unwrap();
}

#[no_mangle]
pub fn start(_: &Lua, config: Config) -> LuaResult<()> {
    {
        if SERVER.read().unwrap().is_some() {
            return Ok(());
        }
    }

    init(&config);

    log::debug!("Config: {:#?}", config);
    log::info!("Starting ...");

    let mut server =
        Server::new(&config).map_err(|err| mlua::Error::ExternalError(Arc::new(err)))?;
    server.run_in_background();
    *(SERVER.write().unwrap()) = Some(server);

    log::info!("Started");

    Ok(())
}

#[no_mangle]
pub fn stop(_: &Lua, _: ()) -> LuaResult<()> {
    log::info!("Stopping ...");

    if let Some(server) = SERVER.write().unwrap().take() {
        server.stop_blocking();
    }

    log::info!("Stopped");

    Ok(())
}

#[no_mangle]
pub fn next(lua: &Lua, (env, callback): (i32, Function)) -> LuaResult<bool> {
    let start = Instant::now();

    if let Some(server) = &*SERVER.read().unwrap() {
        let _guard = server.stats().track_block_time(start);

        let next = match env {
            1 => server.ipc_mission().try_next(),
            2 => server.ipc_hook().try_next(),
            _ => return Ok(false),
        };

        if let Some(mut next) = next {
            server.stats().track_call();

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
    let start = Instant::now();

    let event: StreamEventsResponse = match lua.from_value(event) {
        Ok(event) => event,
        Err(err) => {
            log::error!("failed to deserialize event: {}", err);
            // In certain cases DCS crashes when we return an error back to Lua here (see
            // https://github.com/DCS-gRPC/rust-server/issues/19), which we are working around
            // by intercepting and logging the error instead.
            return Ok(());
        }
    };

    if let Some(server) = &*SERVER.read().unwrap() {
        let _guard = server.stats().track_block_time(start);
        server.stats().track_event();

        log::debug!("Received event: {:#?}", event);
        server.block_on(server.ipc_mission().event(event));
    }

    Ok(())
}

// This method is called on each simulation frame, so make sure to do as few as possible (avoid
// even getting a lock on [SERVER]).
#[no_mangle]
pub fn simulation_frame(_lua: &Lua, time: f64) -> LuaResult<()> {
    crate::fps::frame(time);

    Ok(())
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

#[no_mangle]
pub fn log_info(_: &Lua, err: String) -> LuaResult<()> {
    log::info!("{}", err);
    Ok(())
}

#[no_mangle]
pub fn log_debug(_: &Lua, err: String) -> LuaResult<()> {
    log::debug!("{}", err);
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
pub fn dcs_grpc_hot_reload(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", lua.create_function(hot_reload::start)?)?;
    exports.set("stop", lua.create_function(hot_reload::stop)?)?;
    exports.set("next", lua.create_function(hot_reload::next)?)?;
    exports.set("event", lua.create_function(hot_reload::event)?)?;
    exports.set(
        "simulationFrame",
        lua.create_function(hot_reload::simulation_frame)?,
    )?;
    exports.set("logError", lua.create_function(hot_reload::log_error)?)?;
    exports.set("logWarning", lua.create_function(hot_reload::log_warning)?)?;
    exports.set("logInfo", lua.create_function(hot_reload::log_info)?)?;
    exports.set("logDebug", lua.create_function(hot_reload::log_debug)?)?;
    Ok(exports)
}

#[cfg(not(feature = "hot-reload"))]
#[mlua::lua_module]
pub fn dcs_grpc(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", lua.create_function(start)?)?;
    exports.set("stop", lua.create_function(stop)?)?;
    exports.set("next", lua.create_function(next)?)?;
    exports.set("event", lua.create_function(event)?)?;
    exports.set("simulationFrame", lua.create_function(simulation_frame)?)?;
    exports.set("logError", lua.create_function(log_error)?)?;
    exports.set("logWarning", lua.create_function(log_warning)?)?;
    exports.set("logInfo", lua.create_function(log_info)?)?;
    exports.set("logDebug", lua.create_function(log_debug)?)?;
    Ok(exports)
}

fn pretty_print_value(val: Value, indent: usize) -> LuaResult<String> {
    use std::fmt::Write;

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
                let _ = writeln!(
                    s,
                    "{}{} = {},",
                    "  ".repeat(indent + 1),
                    pretty_print_value(key, indent + 1)?,
                    pretty_print_value(value, indent + 1)?
                );
            }
            let _ = write!(s, "{}}}", "  ".repeat(indent));
            s
        }
        Value::Function(_) => "[function]".to_string(),
        Value::Thread(_) => String::new(),
        Value::UserData(_) => String::new(),
        Value::Error(err) => err.to_string(),
    })
}
