use super::MissionRpc;
use stubs::coalition::v0::coalition_service_server::CoalitionService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl CoalitionService for MissionRpc {
    async fn add_group(
        &self,
        request: Request<coalition::v0::AddGroupRequest>,
    ) -> Result<Response<coalition::v0::AddGroupResponse>, Status> {
        let res: coalition::v0::AddGroupResponse = self.request("addGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn get_groups(
        &self,
        request: Request<coalition::v0::GetGroupsRequest>,
    ) -> Result<Response<coalition::v0::GetGroupsResponse>, Status> {
        let res: coalition::v0::GetGroupsResponse = self.request("getGroups", request).await?;
        Ok(Response::new(res))
    }

    async fn get_bullseye(
        &self,
        request: Request<coalition::v0::GetBullseyeRequest>,
    ) -> Result<Response<coalition::v0::GetBullseyeResponse>, Status> {
        let res: coalition::v0::GetBullseyeResponse = self.request("getBullseye", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_units(
        &self,
        request: Request<coalition::v0::GetPlayerUnitsRequest>,
    ) -> Result<Response<coalition::v0::GetPlayerUnitsResponse>, Status> {
        let res: coalition::v0::GetPlayerUnitsResponse =
            self.request("getPlayers", request).await?;
        Ok(Response::new(res))
    }
}
