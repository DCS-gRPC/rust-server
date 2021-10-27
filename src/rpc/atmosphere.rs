use super::dcs::atmosphere::atmosphere_service_server::AtmosphereService;
use super::dcs::*;
use super::MissionRpc;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl AtmosphereService for MissionRpc {
    async fn get_wind(
        &self,
        request: Request<atmosphere::GetWindRequest>,
    ) -> Result<Response<atmosphere::GetWindResponse>, Status> {
        let res: atmosphere::GetWindResponse = self.request("getWind", request).await?;
        Ok(Response::new(res))
    }

    async fn get_wind_with_turbulence(
        &self,
        request: Request<atmosphere::GetWindWithTurbulenceRequest>,
    ) -> Result<Response<atmosphere::GetWindWithTurbulenceResponse>, Status> {
        let res: atmosphere::GetWindWithTurbulenceResponse =
            self.request("getWindWithTurbulence", request).await?;
        Ok(Response::new(res))
    }

    async fn get_temperature_and_pressure(
        &self,
        request: Request<atmosphere::GetTemperatureAndPressureRequest>,
    ) -> Result<Response<atmosphere::GetTemperatureAndPressureResponse>, Status> {
        let res: atmosphere::GetTemperatureAndPressureResponse =
            self.request("getTemperatureAndPressure", request).await?;
        Ok(Response::new(res))
    }
}
