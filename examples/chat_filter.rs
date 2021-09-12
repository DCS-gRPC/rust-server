use std::time::Duration;

use backoff::ExponentialBackoff;
use dcs_grpc_server::rpc::dcs::hook::{FilterChatMessageRequest, StreamChatRequest};
use dcs_grpc_server::rpc::dcs::hook_client::HookClient;
use futures_util::future::{select, FutureExt};
use futures_util::stream::StreamExt;
use log::LevelFilter;
use tonic::{transport, Request, Status};

async fn run() -> Result<(), Error> {
    let addr = "http://127.0.0.1:50051";
    log::debug!("Connecting to gRPC server at {}", addr);

    let endpoint = transport::Endpoint::from_static(addr).keep_alive_while_idle(true);
    let mut client = HookClient::connect(endpoint).await?;
    let mut chat_messages = client
        .stream_chat(Request::new(StreamChatRequest {}))
        .await?
        .into_inner();

    while let Some(msg) = chat_messages.next().await {
        let msg = msg?;
        client
            .filter_chat_message(Request::new(FilterChatMessageRequest {
                uuid: msg.uuid,
                message: msg.message.to_uppercase(),
            }))
            .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_module("chat_filter", LevelFilter::Debug)
        .init();

    let backoff = ExponentialBackoff {
        // never wait longer than 30s for a retry
        max_interval: Duration::from_secs(30),
        // never stop trying
        max_elapsed_time: None,
        ..Default::default()
    };

    select(
        Box::pin(backoff::future::retry_notify(
            backoff,
            || async { run().await.map_err(backoff::Error::Transient) },
            |err, backoff: Duration| {
                log::error!("Error: {}", err);
                log::info!("Retrying after error in {:.2}s", backoff.as_secs_f64());
            },
        )),
        Box::pin(tokio::signal::ctrl_c().map(|_| ())),
    )
    .await;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Grpc(#[from] Status),
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),
    #[error("event stream ended")]
    End,
}
