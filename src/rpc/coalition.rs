use super::MissionRpc;
use stubs::coalition::coalition_service_server::CoalitionService;
use stubs::*;
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

    async fn get_main_reference_point(
        &self,
        request: Request<coalition::GetMainReferencePointRequest>,
    ) -> Result<Response<coalition::GetMainReferencePointResponse>, Status> {
        let res: coalition::GetMainReferencePointResponse =
            self.request("getMainReferencePoint", request).await?;
        Ok(Response::new(res))
    }
}
