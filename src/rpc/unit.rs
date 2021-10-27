use super::dcs::unit::unit_service_server::UnitService;
use super::dcs::*;
use super::MissionRpc;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UnitService for MissionRpc {
    async fn get_radar(
        &self,
        request: Request<unit::GetRadarRequest>,
    ) -> Result<Response<unit::GetRadarResponse>, Status> {
        let res: unit::GetRadarResponse = self.request("getRadar", request).await?;
        Ok(Response::new(res))
    }

    async fn get_position(
        &self,
        request: Request<unit::GetUnitPositionRequest>,
    ) -> Result<Response<unit::GetUnitPositionResponse>, Status> {
        let res: unit::GetUnitPositionResponse = self.request("getUnitPosition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_name(
        &self,
        request: Request<unit::GetUnitPlayerNameRequest>,
    ) -> Result<Response<unit::GetUnitPlayerNameResponse>, Status> {
        let res: unit::GetUnitPlayerNameResponse =
            self.request("getUnitPlayerName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_unit_descriptor(
        &self,
        request: Request<unit::GetUnitDescriptorRequest>,
    ) -> Result<Response<unit::GetUnitDescriptorResponse>, Status> {
        let res: unit::GetUnitDescriptorResponse =
            self.request("getUnitDescriptor", request).await?;
        Ok(Response::new(res))
    }

    async fn set_emission(
        &self,
        request: Request<unit::SetEmissionRequest>,
    ) -> Result<Response<unit::SetEmissionResponse>, Status> {
        self.notification("setEmission", request).await?;
        Ok(Response::new(unit::SetEmissionResponse {}))
    }
}
