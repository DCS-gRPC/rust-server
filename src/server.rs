use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;

use crate::chat::Chat;
use crate::rpc::{dcs, HookRpc, MissionRpc};
use crate::shutdown::{Shutdown, ShutdownHandle};
use crate::stats::Stats;
use dcs::atmosphere::atmosphere_service_server::AtmosphereServiceServer;
use dcs::coalitions_server::CoalitionsServer;
use dcs::controllers_server::ControllersServer;
use dcs::custom_server::CustomServer;
use dcs::hook_server::HookServer;
use dcs::mission_server::MissionServer;
use dcs::timer_server::TimerServer;
use dcs::trigger::trigger_service_server::TriggerServiceServer;
use dcs::units_server::UnitsServer;
use dcs::world::world_service_server::WorldServiceServer;
use dcs::*;
use dcs_module_ipc::IPC;
use futures_util::FutureExt;
use serde::{Deserialize, Serialize};
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
    config: Config,
    ipc_mission: IPC<Event>,
    ipc_hook: IPC<()>,
    chat: Chat,
    stats: Stats,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub debug: bool,
    pub eval_enabled: bool,
}

impl Server {
    pub fn new(config: Config) -> Result<Self, StartError> {
        let ipc_mission = IPC::new();
        let ipc_hook = IPC::new();
        let runtime = Runtime::new()?;
        let shutdown = Shutdown::new();
        Ok(Self {
            runtime,
            after_shutdown: None,
            state: ServerState {
                addr: format!("{}:{}", config.host, config.port).parse()?,
                config,
                ipc_mission,
                ipc_hook,
                chat: Chat::default(),
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

    pub fn handle_chat_message(&self, player_id: u32, message: String, all: bool) {
        self.state.chat.handle_message(player_id, message, all);
    }

    pub fn ipc_mission(&self) -> &IPC<Event> {
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
    log::info!("Staring gRPC Server ...");

    let ServerState {
        addr,
        config,
        ipc_mission,
        ipc_hook,
        chat,
        stats,
    } = state;

    let mut mission_rpc = MissionRpc::new(ipc_mission, stats.clone(), shutdown_signal.clone());
    let mut hook_rpc = HookRpc::new(ipc_hook, chat, stats, shutdown_signal.clone());

    if config.eval_enabled {
        mission_rpc.enable_eval();
        hook_rpc.enable_eval();
    }

    transport::Server::builder()
        .add_service(AtmosphereServiceServer::new(mission_rpc.clone()))
        .add_service(CoalitionsServer::new(mission_rpc.clone()))
        .add_service(ControllersServer::new(mission_rpc.clone()))
        .add_service(CustomServer::new(mission_rpc.clone()))
        .add_service(HookServer::new(hook_rpc))
        .add_service(MissionServer::new(mission_rpc.clone()))
        .add_service(TimerServer::new(mission_rpc.clone()))
        .add_service(TriggerServiceServer::new(mission_rpc.clone()))
        .add_service(UnitsServer::new(mission_rpc.clone()))
        .add_service(WorldServiceServer::new(mission_rpc))
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
