use super::dcs::coalition::coalition_service_server::CoalitionService;
use super::dcs::*;
use super::MissionRpc;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl CoalitionService for MissionRpc {
    async fn get_players(
        &self,
        request: Request<coalition::GetPlayersRequest>,
    ) -> Result<Response<coalition::GetPlayersResponse>, Status> {
        let res: coalition::GetPlayersResponse = self.request("getPlayers", request).await?;
        Ok(Response::new(res))
    }

    async fn get_groups(
        &self,
        request: Request<coalition::GetGroupsRequest>,
    ) -> Result<Response<coalition::GetGroupsResponse>, Status> {
        let res: coalition::GetGroupsResponse = self.request("getGroups", request).await?;
        Ok(Response::new(res))
    }
}
