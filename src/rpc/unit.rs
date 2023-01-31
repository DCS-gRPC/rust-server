use super::MissionRpc;
use stubs::unit;
use stubs::unit::v0::unit_service_server::UnitService;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UnitService for MissionRpc {
    async fn get_radar(
        &self,
        request: Request<unit::v0::GetRadarRequest>,
    ) -> Result<Response<unit::v0::GetRadarResponse>, Status> {
        let res = self.request("getRadar", request).await?;
        Ok(Response::new(res))
    }

    async fn get_position(
        &self,
        request: Request<unit::v0::GetPositionRequest>,
    ) -> Result<Response<unit::v0::GetPositionResponse>, Status> {
        let res = self.request("getUnitPosition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_name(
        &self,
        request: Request<unit::v0::GetPlayerNameRequest>,
    ) -> Result<Response<unit::v0::GetPlayerNameResponse>, Status> {
        let res = self.request("getUnitPlayerName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_descriptor(
        &self,
        request: Request<unit::v0::GetDescriptorRequest>,
    ) -> Result<Response<unit::v0::GetDescriptorResponse>, Status> {
        let res = self.request("getUnitDescriptor", request).await?;
        Ok(Response::new(res))
    }

    async fn set_emission(
        &self,
        request: Request<unit::v0::SetEmissionRequest>,
    ) -> Result<Response<unit::v0::SetEmissionResponse>, Status> {
        let res = self.request("setEmission", request).await?;
        Ok(Response::new(res))
    }

    async fn get(
        &self,
        request: Request<unit::v0::GetRequest>,
    ) -> Result<Response<unit::v0::GetResponse>, Status> {
        let res = self.request("getUnit", request).await?;
        Ok(Response::new(res))
    }

    async fn get_transform(
        &self,
        request: Request<unit::v0::GetTransformRequest>,
    ) -> Result<Response<unit::v0::GetTransformResponse>, Status> {
        let res = self.request("getUnitTransform", request).await?;
        Ok(Response::new(res))
    }

    async fn destroy(
        &self,
        request: Request<unit::v0::DestroyRequest>,
    ) -> Result<Response<unit::v0::DestroyResponse>, Status> {
        let res = self.request("unitDestroy", request).await?;
        Ok(Response::new(res))
    }
}
