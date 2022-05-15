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
        let res = self.request("addGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn get_static_objects(
        &self,
        request: Request<coalition::v0::GetStaticObjectsRequest>,
    ) -> Result<Response<coalition::v0::GetStaticObjectsResponse>, Status> {
        let res = self.request("getStaticObjects", request).await?;
        Ok(Response::new(res))
    }

    async fn add_static_object(
        &self,
        request: Request<coalition::v0::AddStaticObjectRequest>,
    ) -> Result<Response<coalition::v0::AddStaticObjectResponse>, Status> {
        let res = self.request("addStaticObject", request).await?;
        Ok(Response::new(res))
    }

    async fn add_linked_static(
        &self,
        request: Request<coalition::v0::AddLinkedStaticRequest>,
    ) -> Result<Response<coalition::v0::AddLinkedStaticResponse>, Status> {
        let res = self.request("addLinkedStatic", request).await?;
        Ok(Response::new(res))
    }

    async fn get_groups(
        &self,
        request: Request<coalition::v0::GetGroupsRequest>,
    ) -> Result<Response<coalition::v0::GetGroupsResponse>, Status> {
        let res = self.request("getGroups", request).await?;
        Ok(Response::new(res))
    }

    async fn get_bullseye(
        &self,
        request: Request<coalition::v0::GetBullseyeRequest>,
    ) -> Result<Response<coalition::v0::GetBullseyeResponse>, Status> {
        let res = self.request("getBullseye", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_units(
        &self,
        request: Request<coalition::v0::GetPlayerUnitsRequest>,
    ) -> Result<Response<coalition::v0::GetPlayerUnitsResponse>, Status> {
        let res = self.request("getPlayerUnits", request).await?;
        Ok(Response::new(res))
    }
}
