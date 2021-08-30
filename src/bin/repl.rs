use std::io::{self, BufRead};

use dcs_grpc_server::rpc::dcs::custom_client::CustomClient;
use dcs_grpc_server::rpc::dcs::{EvalRequest, EvalResponse};
use serde_json::Value;
use tonic::{transport, Code, Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint =
        transport::Endpoint::from_static("http://127.0.0.1:50051").keep_alive_while_idle(true);
    let mut client = CustomClient::connect(endpoint).await?;

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        if let Some(line) = lines.next() {
            let req = Request::new(EvalRequest { lua: line? });
            let result = client.eval(req).await;

            let json: Value = match handle_respone(result) {
                Ok(json) => json,
                Err(Error::Grpc(err)) if err.code() == Code::Unavailable => {
                    return Err(err.into());
                }
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };

            if let Some(s) = json.as_str() {
                println!("= {}", s);
            } else {
                let json = serde_json::to_string_pretty(&json)?;
                println!("= {}", json);
            }
        }
    }
}

fn handle_respone(res: Result<Response<EvalResponse>, Status>) -> Result<Value, Error> {
    let json = res?.into_inner().json;
    let json: Value = serde_json::from_str(&json)?;
    Ok(json)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Grpc(#[from] Status),
    #[error("failed to decode JSON result")]
    Json(#[from] serde_json::Error),
}
