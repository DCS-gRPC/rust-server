use std::time::Duration;

use crate::rpc::dcs::Event;
use crate::rpc::RPC;
use dcs_module_ipc::IPC;
use futures::FutureExt;
use tokio::sync::oneshot::Receiver;
use tokio::time::sleep;
use tonic::transport;

pub async fn run(ipc: IPC<Event>, mut shutdown_signal: Receiver<()>) {
    loop {
        match try_run(ipc.clone(), &mut shutdown_signal).await {
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
    shutdown_signal: &mut Receiver<()>,
) -> Result<(), transport::Error> {
    log::info!("Staring gRPC Server ...");

    let addr = "[::1]:50051".parse().unwrap();
    RPC::builder(ipc)
        .serve_with_shutdown(addr, shutdown_signal.map(|_| ()))
        .await?;

    log::info!("Server stopped ...");

    Ok(())
}
