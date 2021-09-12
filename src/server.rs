use std::time::Duration;

use crate::chat::Chat;
use crate::rpc::{dcs, HookRpc, MissionRpc};
use crate::shutdown::ShutdownHandle;
use dcs::atmosphere_server::AtmosphereServer;
use dcs::coalitions_server::CoalitionsServer;
use dcs::controllers_server::ControllersServer;
use dcs::custom_server::CustomServer;
use dcs::hook_server::HookServer;
use dcs::mission_server::MissionServer;
use dcs::timer_server::TimerServer;
use dcs::triggers_server::TriggersServer;
use dcs::units_server::UnitsServer;
use dcs::world_server::WorldServer;
use dcs::*;
use dcs_module_ipc::IPC;
use futures_util::FutureExt;
use tokio::sync::oneshot::Receiver;
use tokio::time::sleep;
use tonic::transport::{self, Server};

pub async fn run(
    ipc_mission: IPC<Event>,
    ipc_hook: IPC<()>,
    chat: Chat,
    shutdown_signal: ShutdownHandle,
    mut after_shutdown: Receiver<()>,
) {
    loop {
        match try_run(
            ipc_mission.clone(),
            ipc_hook.clone(),
            chat.clone(),
            shutdown_signal.clone(),
            &mut after_shutdown,
        )
        .await
        {
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
    ipc_mission: IPC<Event>,
    ipc_hook: IPC<()>,
    chat: Chat,
    shutdown_signal: ShutdownHandle,
    after_shutdown: &mut Receiver<()>,
) -> Result<(), transport::Error> {
    log::info!("Staring gRPC Server ...");

    let addr = "0.0.0.0:50051".parse().unwrap();
    let mission_rpc = MissionRpc::new(ipc_mission, shutdown_signal.clone());
    let hook_rpc = HookRpc::new(ipc_hook, chat, shutdown_signal.clone());
    Server::builder()
        .add_service(AtmosphereServer::new(mission_rpc.clone()))
        .add_service(CoalitionsServer::new(mission_rpc.clone()))
        .add_service(ControllersServer::new(mission_rpc.clone()))
        .add_service(CustomServer::new(mission_rpc.clone()))
        .add_service(HookServer::new(hook_rpc))
        .add_service(MissionServer::new(mission_rpc.clone()))
        .add_service(TimerServer::new(mission_rpc.clone()))
        .add_service(TriggersServer::new(mission_rpc.clone()))
        .add_service(UnitsServer::new(mission_rpc.clone()))
        .add_service(WorldServer::new(mission_rpc))
        .serve_with_shutdown(addr, after_shutdown.map(|_| ()))
        .await?;

    log::info!("Server stopped ...");

    Ok(())
}
