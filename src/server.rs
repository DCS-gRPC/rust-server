use std::time::Duration;

use crate::service::Service;
use dcs_module_rpc::RPC;
use futures::FutureExt;
use tokio::sync::oneshot::Receiver;
use tokio::time::delay_for;
use tonic::transport;

#[tokio::main]
pub async fn run(rpc: RPC<usize>, mut shutdown_signal: Receiver<()>) {
    loop {
        match try_run(rpc.clone(), &mut shutdown_signal).await {
            Ok(_) => break,
            Err(err) => {
                log::error!("{}", err);
                log::info!("Restarting gRPC Server in 10 seconds ...");
                delay_for(Duration::from_secs(10)).await;
            }
        }
    }
}

async fn try_run(
    rpc: RPC<usize>,
    shutdown_signal: &mut Receiver<()>,
) -> Result<(), transport::Error> {
    log::info!("Staring gRPC Server ...");

    let addr = "[::1]:50051".parse().unwrap();
    Service::builder(rpc)
        .serve_with_shutdown(addr, shutdown_signal.map(|_| ()))
        .await?;

    log::info!("Server stopped ...");

    Ok(())
}
