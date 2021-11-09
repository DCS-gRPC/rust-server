use super::MissionRpc;
use stubs::world::v0::world_service_server::WorldService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl WorldService for MissionRpc {
    async fn get_airbases(
        &self,
        request: Request<world::v0::GetAirbasesRequest>,
    ) -> Result<Response<world::v0::GetAirbasesResponse>, Status> {
        let res: world::v0::GetAirbasesResponse = self.request("getAirbases", request).await?;
        Ok(Response::new(res))
    }

    async fn get_mark_panels(
        &self,
        request: Request<world::v0::GetMarkPanelsRequest>,
    ) -> Result<Response<world::v0::GetMarkPanelsResponse>, Status> {
        let res: world::v0::GetMarkPanelsResponse = self.request("getMarkPanels", request).await?;
        Ok(Response::new(res))
    }
}
