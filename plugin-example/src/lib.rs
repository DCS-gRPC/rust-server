mod ffi;

pub mod greeter {
    pub mod v0 {
        tonic::include_proto!("example.greeter.v0");
    }
}

use std::str::FromStr;

use greeter::v0::greeter_service_server::GreeterService;
use greeter::v0::greeter_service_server::GreeterServiceServer;
use greeter::v0::GreetRequest;
use greeter::v0::GreetResponse;
use prost::Message;
use stubs::trigger::v0::trigger_service_client::TriggerServiceClient;
use stubs::trigger::v0::OutTextRequest;
use tokio::runtime::Runtime;
use tokio::sync::oneshot::Receiver;
use tokio::sync::Mutex;
use tonic::transport;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::Response;
use tonic::{Request, Status};

pub struct Plugin {
    runtime: Runtime,
    // shutdown: oneshoot::Sender<()>,
    trigger: Mutex<TriggerServiceClient<Channel>>,
}

impl Plugin {
    pub fn new(port: u16) -> Self {
        let runtime = Runtime::new().unwrap();
        let _guard = runtime.enter();

        let url = format!("http://127.0.0.1:{}", port);
        let channel = Endpoint::from_str(&url)
            .unwrap()
            .keep_alive_while_idle(true)
            .connect_lazy();
        let trigger = TriggerServiceClient::new(channel);

        Self {
            runtime,
            trigger: Mutex::new(trigger),
        }
    }
}

// TODO: re-use runtime between calls
fn handle_call(plugin: &Plugin, method: &str, request: &[u8]) -> Vec<u8> {
    match method {
        "example.greeter.v0.GreeterService/Greet" => {
            let request = GreetRequest::decode(request).unwrap();
            let mut response = Vec::new();
            plugin
                .runtime
                .block_on(plugin.greet(Request::new(request)))
                .unwrap()
                .into_inner()
                .encode(&mut response)
                .unwrap();
            response
        }
        _ => unimplemented!("method {}", method),
    }
}

#[tonic::async_trait]
impl GreeterService for Plugin {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
        // TODO: ffi instead of http calls for this communication direction?
        let GreetRequest { name } = request.into_inner();
        let mut trigger = self.trigger.lock().await;
        trigger
            .out_text(OutTextRequest {
                text: format!("Hi, {}!", name),
                display_time: 10,
                clear_view: false,
            })
            .await?;

        Ok(Response::new(GreetResponse { success: true }))
    }
}
