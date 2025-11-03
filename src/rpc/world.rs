use stubs::world::v0::world_service_server::WorldService;
use stubs::*;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl WorldService for MissionRpc {
    async fn get_airbases(
        &self,
        request: Request<world::v0::GetAirbasesRequest>,
    ) -> Result<Response<world::v0::GetAirbasesResponse>, Status> {
        let res = self.request("getAirbases", request).await?;
        Ok(Response::new(res))
    }

    async fn get_mark_panels(
        &self,
        request: Request<world::v0::GetMarkPanelsRequest>,
    ) -> Result<Response<world::v0::GetMarkPanelsResponse>, Status> {
        let res = self.request("getMarkPanels", request).await?;
        Ok(Response::new(res))
    }

    async fn get_theatre(
        &self,
        request: Request<world::v0::GetTheatreRequest>,
    ) -> Result<Response<world::v0::GetTheatreResponse>, Status> {
        let res = self.request("getTheatre", request).await?;
        Ok(Response::new(res))
    }

    async fn search_objects(
        &self,
        request: Request<world::v0::SearchObjectsRequest>,
    ) -> Result<Response<world::v0::SearchObjectsResponse>, Status> {
        let res = self.request("searchObjects", request).await?;
        Ok(Response::new(res))
    }
}
