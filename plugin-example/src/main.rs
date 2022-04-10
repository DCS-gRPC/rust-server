pub mod greeter {
    pub mod v0 {
        tonic::include_proto!("example.greeter.v0");
    }
}

use std::env;
use std::str::FromStr;

use greeter::v0::greeter_service_server::GreeterService;
use greeter::v0::greeter_service_server::GreeterServiceServer;
use greeter::v0::GreetRequest;
use greeter::v0::GreetResponse;
use stubs::trigger::v0::trigger_service_client::TriggerServiceClient;
use stubs::trigger::v0::OutTextRequest;
use tokio::sync::Mutex;
use tonic::transport;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::Response;
use tonic::{Request, Status};

#[tokio::main]
async fn main() {
    let server_port = env::var("SERVER_PORT")
        .expect("SERVER_PORT not defined")
        .parse::<u16>()
        .expect("SERVER_PORT as u16");
    let plugin_port = env::var("PLUGIN_PORT")
        .expect("PLUGIN_PORT not defined")
        .parse::<u16>()
        .expect("PLUGIN_PORT as u16");

    let url = format!("http://127.0.0.1:{}", server_port);
    let channel = Endpoint::from_str(&url)
        .unwrap()
        .keep_alive_while_idle(true)
        .connect_lazy();
    let trigger = TriggerServiceClient::new(channel);

    transport::Server::builder()
        .add_service(GreeterServiceServer::new(Plugin {
            trigger: Mutex::new(trigger),
        }))
        .serve(([127, 0, 0, 1], plugin_port).into())
        .await
        .expect("run plugin gRPC server");
}

pub struct Plugin {
    trigger: Mutex<TriggerServiceClient<Channel>>,
}

#[tonic::async_trait]
impl GreeterService for Plugin {
    async fn greet(
        &self,
        request: Request<GreetRequest>,
    ) -> Result<Response<GreetResponse>, Status> {
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
