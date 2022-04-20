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
        let res = self.request("sendChatTo", request).await?;
        Ok(Response::new(res))
    }

    async fn send_chat(
        &self,
        request: Request<net::v0::SendChatRequest>,
    ) -> Result<Response<net::v0::SendChatResponse>, Status> {
        let res = self.request("sendChat", request).await?;
        Ok(Response::new(res))
    }

    async fn get_players(
        &self,
        request: Request<net::v0::GetPlayersRequest>,
    ) -> Result<Response<net::v0::GetPlayersResponse>, Status> {
        let res = self.request("getPlayers", request).await?;
        Ok(Response::new(res))
    }

    async fn force_player_slot(
        &self,
        request: Request<net::v0::ForcePlayerSlotRequest>,
    ) -> Result<Response<net::v0::ForcePlayerSlotResponse>, Status> {
        let res = self.request("forcePlayerSlot", request).await?;
        Ok(Response::new(res))
    }

    async fn kick_player(
        &self,
        request: Request<net::v0::KickPlayerRequest>,
    ) -> Result<Response<net::v0::KickPlayerResponse>, Status> {
        let res = self.request("kickPlayer", request).await?;
        Ok(Response::new(res))
    }
}
