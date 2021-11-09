use std::io::{self, BufRead};

use clap::Parser;
use serde_json::Value;
use stubs::custom::v0::custom_service_client::CustomServiceClient;
use stubs::hook::v0::hook_service_client::HookServiceClient;
use stubs::{custom, hook};
use tonic::{transport, Code, Status};

#[derive(Parser)]
#[clap(name = "repl")]
struct Opts {
    #[clap(short, long, possible_values = ["mission", "hook"], default_value = "mission")]
    env: String,
}

enum Client<T> {
    Mission(CustomServiceClient<T>),
    Hook(HookServiceClient<T>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let endpoint =
        transport::Endpoint::from_static("http://127.0.0.1:50051").keep_alive_while_idle(true);
    let mut client = match opts.env.as_str() {
        "mission" => Client::Mission(CustomServiceClient::connect(endpoint).await?),
        "hook" => Client::Hook(HookServiceClient::connect(endpoint).await?),
        _ => unreachable!("invalid --env value"),
    };

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        if let Some(line) = lines.next() {
            let lua = line?;
            let result = match &mut client {
                Client::Mission(client) => client
                    .eval(custom::v0::EvalRequest { lua })
                    .await
                    .map(|res| res.into_inner().json),
                Client::Hook(client) => client
                    .eval(hook::v0::EvalRequest { lua })
                    .await
                    .map(|res| res.into_inner().json),
            };

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

fn handle_respone(json: Result<String, Status>) -> Result<Value, Error> {
    let json = json?;
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
