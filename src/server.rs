use std::time::Duration;

use crate::rpc::dcs::mission_server::MissionServer;
use crate::rpc::dcs::Event;
use crate::rpc::RPC;
use crate::shutdown::ShutdownHandle;
use dcs_module_ipc::IPC;
use futures_util::FutureExt;
use tokio::sync::oneshot::Receiver;
use tokio::time::sleep;
use tonic::transport;

pub async fn run(
    ipc: IPC<Event>,
    shutdown_signal: ShutdownHandle,
    mut after_shutdown: Receiver<()>,
) {
    loop {
        match try_run(ipc.clone(), shutdown_signal.clone(), &mut after_shutdown).await {
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
    ipc: IPC<Event>,
    shutdown_signal: ShutdownHandle,
    after_shutdown: &mut Receiver<()>,
) -> Result<(), transport::Error> {
    log::info!("Staring gRPC Server ...");

    let addr = "0.0.0.0:50051".parse().unwrap();
    let rpc = RPC::new(ipc, shutdown_signal.clone());
    transport::Server::builder()
        .add_service(MissionServer::new(rpc))
        .serve_with_shutdown(addr, after_shutdown.map(|_| ()))
        .await?;

    log::info!("Server stopped ...");

    Ok(())
}
