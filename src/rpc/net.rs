use super::MissionRpc;
use stubs::net::v0::net_service_server::NetService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl NetService for MissionRpc {
    async fn send_chat_to(
        &self,
        request: Request<net::v0::SendChatToRequest>,
    ) -> Result<Response<net::v0::SendChatToResponse>, Status> {
        self.notification("sendChatTo", request).await?;
        Ok(Response::new(net::v0::SendChatToResponse {}))
    }

    async fn send_chat(
        &self,
        request: Request<net::v0::SendChatRequest>,
    ) -> Result<Response<net::v0::SendChatResponse>, Status> {
        self.notification("sendChat", request).await?;
        Ok(Response::new(net::v0::SendChatResponse {}))
    }

    // getPlayerInfo
    async fn get_player_info(
        &self,
        request: Request<net::v0::GetPlayerInfoRequest>,
    ) -> Result<Response<net::v0::GetPlayerInfoResponse>, Status> {
        let res: net::v0::GetPlayerInfoResponse = self.request("getPlayerInfo", request).await?;
        Ok(Response::new(res))
    }
}
