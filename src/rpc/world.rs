use super::MissionRpc;
use stubs::world::world_service_server::WorldService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl WorldService for MissionRpc {
    async fn get_airbases(
        &self,
        request: Request<world::GetAirbasesRequest>,
    ) -> Result<Response<world::GetAirbasesResponse>, Status> {
        let res: world::GetAirbasesResponse = self.request("getAirbases", request).await?;
        Ok(Response::new(res))
    }

    async fn get_mark_panels(
        &self,
        request: Request<world::GetMarkPanelsRequest>,
    ) -> Result<Response<world::GetMarkPanelsResponse>, Status> {
        let res: world::GetMarkPanelsResponse = self.request("getMarkPanels", request).await?;
        Ok(Response::new(res))
    }
}
