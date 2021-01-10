use dcs::mission_server::{Mission, MissionServer};
use dcs::{OutTextRequest, OutTextResponse};
use dcs_module_rpc::RPC;
use tonic::transport::server::Router;
use tonic::transport::{self, Server};
use tonic::{Request, Response, Status};

pub mod dcs {
    tonic::include_proto!("dcs");
}

pub struct Service {
    rpc: RPC<usize>,
}

impl Service {
    pub fn builder(
        rpc: RPC<usize>,
    ) -> Router<MissionServer<Service>, transport::server::Unimplemented> {
        Server::builder().add_service(MissionServer::new(Service { rpc }))
    }
}

#[tonic::async_trait]
impl Mission for Service {
    async fn out_text(
        &self,
        request: Request<OutTextRequest>,
    ) -> Result<Response<OutTextResponse>, Status> {
        self.rpc
            .notification("outText", Some(request.into_inner()))
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(OutTextResponse { success: true }))
    }
}
