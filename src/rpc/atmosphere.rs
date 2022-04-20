use super::MissionRpc;
use stubs::atmosphere::v0::atmosphere_service_server::AtmosphereService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl AtmosphereService for MissionRpc {
    async fn get_wind(
        &self,
        request: Request<atmosphere::v0::GetWindRequest>,
    ) -> Result<Response<atmosphere::v0::GetWindResponse>, Status> {
        let velocity: common::v0::Vector = self.request("getWind", request).await?;
        let (heading, strength) = get_wind_heading_and_strength(&velocity);
        Ok(Response::new(atmosphere::v0::GetWindResponse {
            heading,
            strength,
        }))
    }

    async fn get_wind_with_turbulence(
        &self,
        request: Request<atmosphere::v0::GetWindWithTurbulenceRequest>,
    ) -> Result<Response<atmosphere::v0::GetWindWithTurbulenceResponse>, Status> {
        let velocity: common::v0::Vector = self.request("getWindWithTurbulence", request).await?;
        let (heading, strength) = get_wind_heading_and_strength(&velocity);
        Ok(Response::new(
            atmosphere::v0::GetWindWithTurbulenceResponse { heading, strength },
        ))
    }

    async fn get_temperature_and_pressure(
        &self,
        request: Request<atmosphere::v0::GetTemperatureAndPressureRequest>,
    ) -> Result<Response<atmosphere::v0::GetTemperatureAndPressureResponse>, Status> {
        let res = self.request("getTemperatureAndPressure", request).await?;
        Ok(Response::new(res))
    }
}

fn get_wind_heading_and_strength(v: &common::v0::Vector) -> (f32, f32) {
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
