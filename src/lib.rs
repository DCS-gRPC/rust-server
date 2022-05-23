#![allow(dead_code)]
#![recursion_limit = "256"]

#[cfg(feature = "hot-reload")]
mod hot_reload;
pub mod rpc;
mod server;
mod shutdown;
mod stats;
mod stream;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
use std::time::Instant;

use mlua::{prelude::*, LuaSerdeExt};
use mlua::{Function, Value};
use once_cell::sync::Lazy;
use server::{Config, Server};
use stubs::mission::v0::StreamEventsResponse;

static INITIALIZED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
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
pub fn start(_: &Lua, config: Config) {
    {
        // do nothing if already started
        if SERVER.read().unwrap().is_some() {
            return;
        }
    }

    init(&config);

    log::debug!("Config: {:#?}", config);
    log::info!("Starting ...");

    match Server::new(&config) {
        Ok(mut server) => {
            server.run_in_background();
            *(SERVER.write().unwrap()) = Some(server);
            log::info!("Started");
        }
        Err(err) => {
            log::error!("Failed to start server: {}", err)
        }
    }
}

#[no_mangle]
pub fn stop(_: &Lua, _: ()) {
    log::info!("Stopping ...");

    if let Some(server) = SERVER.write().unwrap().take() {
        server.stop_blocking();
    }

    log::info!("Stopped");
}

#[no_mangle]
pub fn next(lua: &Lua, (env, callback): (i32, Function)) -> bool {
    let start = Instant::now();

    if let Some(server) = &*SERVER.read().unwrap() {
        let _guard = server.stats().track_block_time(start);

        let next = match env {
            1 => server.ipc_mission().try_next(),
            2 => server.ipc_hook().try_next(),
            _ => return false,
        };

        if let Some(mut next) = next {
            let _call = server.stats().track_call();

            let method = next.method().to_string();
            let params = match next.params(lua) {
                Ok(params) => params,
                Err(err) => {
                    log::error!("Failed serialize request params to Lua: {}", err);
                    return true;
                }
            };

            if let Some(params) = &params {
                log::debug!(
                    "Sending request `{}`: {}",
                    method,
                    pretty_print_value(params.clone(), 0)
                );
            } else {
                log::debug!("Sending request `{}`", method,);
            }

            let result: LuaTable = match callback.call((method.as_str(), params)) {
                Ok(result) => result,
                Err(err) => {
                    log::error!("Failed to call next callback: {}", err);
                    return true;
                }
            };
            let error: Option<LuaTable> = result.get("error").unwrap_or_default();

            if let Some(error) = error {
                let message: String = error
                    .get("message")
                    .unwrap_or_else(|err| format!("<failed to read `error.message`: {}>", err));
                let kind: Option<String> = error.get("type").unwrap_or_default();

                next.error(message, kind);
                return true;
            }

            let res: Value<'_> = match result.get("result") {
                Ok(res) => res,
                Err(err) => {
                    log::error!("Failed to read `result` from Lua response: {}", err);
                    return true;
                }
            };
            log::debug!("Receiving: {}", pretty_print_value(res.clone(), 0));

            if let Err(err) = next.success(lua, &res) {
                log::error!("failed to pretty print result: {}", err)
            }

            return true;
        }
    }

    false
}

#[no_mangle]
pub fn event(lua: &Lua, event: Value) {
    let start = Instant::now();

    let event: StreamEventsResponse = match lua.from_value(event.clone()) {
        Ok(event) => event,
        Err(err) => {
            log::error!(
                "failed to deserialize event: {}\n{}",
                err,
                pretty_print_value(event, 0)
            );
            return;
        }
    };

    if let Some(server) = &*SERVER.read().unwrap() {
        let _guard = server.stats().track_block_time(start);
        server.stats().track_event();

        log::debug!("Received event: {:#?}", event);
        server.block_on(server.ipc_mission().event(event));
    }
}

#[no_mangle]
pub fn log_error(_: &Lua, err: String) {
    log::error!("{}", err);
}

#[no_mangle]
pub fn log_warning(_: &Lua, err: String) {
    log::warn!("{}", err);
}

#[no_mangle]
pub fn log_info(_: &Lua, err: String) {
    log::info!("{}", err);
}

#[no_mangle]
pub fn log_debug(_: &Lua, err: String) {
    log::debug!("{}", err);
}

#[cfg(feature = "hot-reload")]
#[mlua::lua_module]
pub fn dcs_grpc_hot_reload(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", infallible(lua, hot_reload::start)?)?;
    exports.set("stop", infallible(lua, hot_reload::stop)?)?;
    exports.set("next", infallible(lua, hot_reload::next)?)?;
    exports.set("event", infallible(lua, hot_reload::event)?)?;
    exports.set("logError", infallible(lua, hot_reload::log_error)?)?;
    exports.set("logWarning", infallible(lua, hot_reload::log_warning)?)?;
    exports.set("logInfo", infallible(lua, hot_reload::log_info)?)?;
    exports.set("logDebug", infallible(lua, hot_reload::log_debug)?)?;
    Ok(exports)
}

#[cfg(not(feature = "hot-reload"))]
#[mlua::lua_module]
pub fn dcs_grpc(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", infallible(lua, start)?)?;
    exports.set("stop", infallible(lua, stop)?)?;
    exports.set("next", infallible(lua, next)?)?;
    exports.set("event", infallible(lua, event)?)?;
    exports.set("logError", infallible(lua, log_error)?)?;
    exports.set("logWarning", infallible(lua, log_warning)?)?;
    exports.set("logInfo", infallible(lua, log_info)?)?;
    exports.set("logDebug", infallible(lua, log_debug)?)?;
    Ok(exports)
}

// The combination of DCS and `mlua::Error` is unfortunately not working well together.
// `mlua::Error`s returned to DCS crash DCS in certain cases. We were unable to figure out the cause
// and are also unable to reproduce the issue outside of DCS, which is why we simply avoid returning
// errors (and directly log them instead). The following function is used to make sure that we don't
// accidentally return errors to DCS.
fn infallible<'lua, 'callback, A, R, F>(lua: &'lua Lua, func: F) -> LuaResult<Function<'lua>>
where
    'lua: 'callback,
    A: FromLuaMulti<'callback>,
    R: ToLuaMulti<'callback>,
    F: 'static + Send + Fn(&'callback Lua, A) -> R,
{
    lua.create_function(move |lua, arg| Ok(func(lua, arg)))
}

fn pretty_print_value(val: Value, indent: usize) -> String {
    match val {
        Value::Nil => "nil".to_string(),
        Value::Boolean(v) => v.to_string(),
        Value::LightUserData(_) => String::new(),
        Value::Integer(v) => v.to_string(),
        Value::Number(v) => v.to_string(),
        Value::String(v) => format!(
            "\"{}\"",
            v.to_str().unwrap_or("<failed to convert to string>")
        ),
        Value::Table(t) => {
            let mut s = "{\n".to_string();
            let mut pairs = t.pairs::<Value, Value>();
            while let Some(Ok((key, value))) = pairs.next() {
                s += &format!(
                    "{}{} = {},\n",
                    "  ".repeat(indent + 1),
                    pretty_print_value(key, indent + 1),
                    pretty_print_value(value, indent + 1)
                );
            }
            s += &format!("{}}}", "  ".repeat(indent));
            s
        }
        Value::Function(_) => "[function]".to_string(),
        Value::Thread(_) => String::new(),
        Value::UserData(_) => String::new(),
        Value::Error(err) => err.to_string(),
    }
}
