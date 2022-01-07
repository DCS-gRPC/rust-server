use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;

use crate::rpc::{HookRpc, MissionRpc};
use crate::services::DcsServices;
use crate::shutdown::{Shutdown, ShutdownHandle};
use crate::stats::Stats;
use dcs_module_ipc::IPC;
use futures_util::FutureExt;
use serde::{Deserialize, Serialize};
use stubs::mission::v0::StreamEventsResponse;
use tokio::runtime::Runtime;
use tokio::sync::oneshot::{self, Receiver};
use tokio::time::sleep;
use tonic::transport;

pub struct Server {
    runtime: Runtime,
    shutdown: Shutdown,
    after_shutdown: Option<oneshot::Sender<()>>,
    state: ServerState,
}

#[derive(Clone)]
struct ServerState {
    addr: SocketAddr,
    eval_enabled: bool,
    ipc_mission: IPC<StreamEventsResponse>,
    ipc_hook: IPC<()>,
    stats: Stats,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub write_dir: String,
    pub dll_path: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub eval_enabled: bool,
}

impl Server {
    pub fn new(config: &Config) -> Result<Self, StartError> {
        let ipc_mission = IPC::default();
        let ipc_hook = IPC::default();
        let runtime = Runtime::new()?;
        let shutdown = Shutdown::new();
        Ok(Self {
            runtime,
            after_shutdown: None,
            state: ServerState {
                addr: format!("{}:{}", config.host, config.port).parse()?,
                eval_enabled: config.eval_enabled,
                ipc_mission,
                ipc_hook,
                stats: Stats::new(shutdown.handle()),
            },
            shutdown,
        })
    }

    pub fn run_in_background(&mut self) {
        if self.after_shutdown.is_some() {
            // already running
            return;
        }

        let (tx, rx) = oneshot::channel();
        self.after_shutdown = Some(tx);

        self.runtime.spawn(crate::server::run(
            self.state.clone(),
            self.shutdown.handle(),
            rx,
        ));

        self.runtime
            .spawn(self.state.stats.clone().run_in_background());
    }

    pub fn stop_blocking(mut self) {
        // graceful shutdown
        self.runtime.block_on(self.shutdown.shutdown());
        if let Some(after_shutdown) = self.after_shutdown.take() {
            let _ = after_shutdown.send(());
        }

        // shutdown the async runtime, again give everything another 5 secs before forecefully
        // killing everything
        self.runtime.shutdown_timeout(Duration::from_secs(5));
    }

    pub fn ipc_mission(&self) -> &IPC<StreamEventsResponse> {
        &self.state.ipc_mission
    }

    pub fn ipc_hook(&self) -> &IPC<()> {
        &self.state.ipc_hook
    }

    pub fn stats(&self) -> &Stats {
        &self.state.stats
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }
}

async fn run(
    state: ServerState,
    shutdown_signal: ShutdownHandle,
    mut after_shutdown: Receiver<()>,
) {
    loop {
        match try_run(state.clone(), shutdown_signal.clone(), &mut after_shutdown).await {
            Ok(_) => break,
            Err(err) => {
                log::error!("{}", err);
                log::info!("Restarting gIPC Server in 10 seconds ...");
                sleep(Duration::from_secs(10)).await;
            }
        }
    }
}

async fn try_run(
    state: ServerState,
    shutdown_signal: ShutdownHandle,
    after_shutdown: &mut Receiver<()>,
) -> Result<(), transport::Error> {
    log::info!("Staring gRPC Server (on {}) ...", state.addr);

    let ServerState {
        addr,
        eval_enabled,
        ipc_mission,
        ipc_hook,
        stats,
    } = state;

    let mut mission_rpc = MissionRpc::new(ipc_mission, stats.clone(), shutdown_signal.clone());
    let mut hook_rpc = HookRpc::new(ipc_hook, stats, shutdown_signal.clone());

    if eval_enabled {
        mission_rpc.enable_eval();
        hook_rpc.enable_eval();
    }

    transport::Server::builder()
        .add_service(DcsServices::new(mission_rpc, hook_rpc))
        .serve_with_shutdown(addr, after_shutdown.map(|_| ()))
        .await?;

    log::info!("Server stopped ...");

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    AddrParse(#[from] std::net::AddrParseError),
}

fn default_host() -> String {
    String::from("127.0.0.1")
}

fn default_port() -> u16 {
    50051
}

impl<'lua> mlua::FromLua<'lua> for Config {
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        use mlua::LuaSerdeExt;
        let config: Config = lua.from_value(lua_value)?;
        Ok(config)
    }
}
