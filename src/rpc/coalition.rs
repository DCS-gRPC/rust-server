use super::MissionRpc;
use stubs::coalition::v0::coalition_service_server::CoalitionService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl CoalitionService for MissionRpc {
    async fn get_groups(
        &self,
        request: Request<coalition::v0::GetGroupsRequest>,
    ) -> Result<Response<coalition::v0::GetGroupsResponse>, Status> {
        let res: coalition::v0::GetGroupsResponse = self.request("getGroups", request).await?;
        Ok(Response::new(res))
    }

    async fn get_main_reference_point(
        &self,
        request: Request<coalition::v0::GetMainReferencePointRequest>,
    ) -> Result<Response<coalition::v0::GetMainReferencePointResponse>, Status> {
        let res: coalition::v0::GetMainReferencePointResponse =
            self.request("getMainReferencePoint", request).await?;
        Ok(Response::new(res))
    }

    async fn get_players(
        &self,
        request: Request<coalition::v0::GetPlayersRequest>,
    ) -> Result<Response<coalition::v0::GetPlayersResponse>, Status> {
        let res: coalition::v0::GetPlayersResponse = self.request("getPlayers", request).await?;
        Ok(Response::new(res))
    }
}
