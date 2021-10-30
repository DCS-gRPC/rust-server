use super::MissionRpc;
use stubs::unit::unit_service_server::UnitService;
use stubs::*;
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
        request: Request<unit::GetPositionRequest>,
    ) -> Result<Response<unit::GetPositionResponse>, Status> {
        let res: unit::GetPositionResponse = self.request("getUnitPosition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_name(
        &self,
        request: Request<unit::GetPlayerNameRequest>,
    ) -> Result<Response<unit::GetPlayerNameResponse>, Status> {
        let res: unit::GetPlayerNameResponse = self.request("getUnitPlayerName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_descriptor(
        &self,
        request: Request<unit::GetDescriptorRequest>,
    ) -> Result<Response<unit::GetDescriptorResponse>, Status> {
        let res: unit::GetDescriptorResponse = self.request("getUnitDescriptor", request).await?;
        Ok(Response::new(res))
    }

    async fn set_emission(
        &self,
        request: Request<unit::SetEmissionRequest>,
    ) -> Result<Response<unit::SetEmissionResponse>, Status> {
        self.notification("setEmission", request).await?;
        Ok(Response::new(unit::SetEmissionResponse {}))
    }

    async fn get(
        &self,
        request: Request<unit::GetRequest>,
    ) -> Result<Response<unit::GetResponse>, Status> {
        let res: unit::GetResponse = self.request("getUnit", request).await?;
        Ok(Response::new(res))
    }
}
