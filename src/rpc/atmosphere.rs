use super::MissionRpc;
use stubs::atmosphere::atmosphere_service_server::AtmosphereService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl AtmosphereService for MissionRpc {
    async fn get_wind(
        &self,
        request: Request<atmosphere::GetWindRequest>,
    ) -> Result<Response<atmosphere::GetWindResponse>, Status> {
        let velocity: common::Vector = self.request("getWind", request).await?;
        let (heading, strength) = get_wind_heading_and_strength(&velocity);
        Ok(Response::new(atmosphere::GetWindResponse {
            heading,
            strength,
        }))
    }

    async fn get_wind_with_turbulence(
        &self,
        request: Request<atmosphere::GetWindWithTurbulenceRequest>,
    ) -> Result<Response<atmosphere::GetWindWithTurbulenceResponse>, Status> {
        let velocity: common::Vector = self.request("getWindWithTurbulence", request).await?;
        let (heading, strength) = get_wind_heading_and_strength(&velocity);
        Ok(Response::new(atmosphere::GetWindWithTurbulenceResponse {
            heading,
            strength,
        }))
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

fn get_wind_heading_and_strength(v: &common::Vector) -> (f32, f32) {
    let mut heading = v.x.atan2(v.z).to_degrees();
    if heading < 0.0 {
        heading += 360.0;
    }

    // convert TO direction to FROM direction
    if heading > 180.0 {
        heading -= 180.0;
    } else {
        heading += 180.0;
    }

    // calc 2D strength
    let strength = (v.z.powi(2) + v.x.powi(2)).sqrt();

    (heading as f32, strength as f32)
}
