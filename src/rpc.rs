use dcs::mission_server::{Mission, MissionServer};
use dcs::*;
use dcs_module_ipc::IPC;
use tonic::transport::server::Router;
use tonic::transport::{self, Server};
use tonic::{Request, Response, Status};

pub mod dcs {
    tonic::include_proto!("dcs");
}

pub struct RPC {
    ipc: IPC<usize>,
}

impl RPC {
    pub fn builder(
        ipc: IPC<usize>,
    ) -> Router<MissionServer<RPC>, transport::server::Unimplemented> {
        Server::builder().add_service(MissionServer::new(RPC { ipc }))
    }
}

#[tonic::async_trait]
impl Mission for RPC {
    async fn out_text(
        &self,
        request: Request<OutTextRequest>,
    ) -> Result<Response<OutTextResponse>, Status> {
        self.ipc
            .notification("outText", Some(request.into_inner()))
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(OutTextResponse { success: true }))
    }

    async fn get_user_flag(
        &self,
        request: Request<GetUserFlagRequest>,
    ) -> Result<Response<GetUserFlagResponse>, Status> {
        let res: GetUserFlagResponse = self
            .ipc
            .request("getUserFlag", Some(request.into_inner()))
            .await
            .map_err(|err| Status::internal(err.to_string()))?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<SetUserFlagRequest>,
    ) -> Result<Response<SetUserFlagResponse>, Status> {
        self.ipc
            .notification("setUserFlag", Some(request.into_inner()))
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(SetUserFlagResponse { success: true }))
    }
}
