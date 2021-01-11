#![allow(dead_code)]
#![recursion_limit = "256"]

mod rpc;
mod server;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

use dcs_module_ipc::IPC;
use mlua::prelude::*;
use mlua::{Function, Value};
use once_cell::sync::Lazy;
use thiserror::Error;
use tokio::sync::oneshot;

static INITIALIZED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static SERVER: Lazy<RwLock<Option<Server>>> = Lazy::new(|| RwLock::new(None));

struct Server {
    ipc: IPC<usize>,
    shutdown_signal: oneshot::Sender<()>,
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
        .unwrap()
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

fn start(lua: &Lua, (): ()) -> LuaResult<()> {
    {
        if SERVER.read().unwrap().is_some() {
            return Ok(());
        }
    }

    let _write_dir = init(lua)?;

    log::info!("Starting ...");

    let ipc = IPC::new();
    let (tx, rx) = oneshot::channel();

    let mut server = SERVER.write().unwrap();
    *server = Some(Server {
        shutdown_signal: tx,
        ipc: ipc.clone(),
    });

    // Spawn an executor thread that waits for the shutdown signal
    thread::spawn(|| crate::server::run(ipc, rx));

    log::info!("Started ...");

    Ok(())
}

fn stop(_: &Lua, _: ()) -> LuaResult<()> {
    log::info!("Stopping ...");

    if let Some(Server {
        shutdown_signal, ..
    }) = SERVER.write().unwrap().take()
    {
        let _ = shutdown_signal.send(());
    }

    Ok(())
}

fn next(lua: &Lua, callback: Function) -> LuaResult<bool> {
    if let Some(Server { ref ipc, .. }) = *SERVER.read().unwrap() {
        if let Some(mut next) = ipc.try_next() {
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
            let error: Option<String> = result.get("error")?;

            if let Some(error) = error {
                next.error(error);
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

#[mlua::lua_module]
pub fn dcs_grpc_server(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("start", lua.create_function(start)?)?;
    exports.set("stop", lua.create_function(stop)?)?;
    exports.set("next", lua.create_function(next)?)?;
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
