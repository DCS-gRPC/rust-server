use std::ops::Neg;

use super::MissionRpc;
use serde::Deserialize;
use stubs::unit::v0::unit_service_server::UnitService;
use stubs::{common, unit};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl UnitService for MissionRpc {
    async fn get_radar(
        &self,
        request: Request<unit::v0::GetRadarRequest>,
    ) -> Result<Response<unit::v0::GetRadarResponse>, Status> {
        let res: unit::v0::GetRadarResponse = self.request("getRadar", request).await?;
        Ok(Response::new(res))
    }

    async fn get_position(
        &self,
        request: Request<unit::v0::GetPositionRequest>,
    ) -> Result<Response<unit::v0::GetPositionResponse>, Status> {
        let res: unit::v0::GetPositionResponse = self.request("getUnitPosition", request).await?;
        Ok(Response::new(res))
    }

    async fn get_player_name(
        &self,
        request: Request<unit::v0::GetPlayerNameRequest>,
    ) -> Result<Response<unit::v0::GetPlayerNameResponse>, Status> {
        let res: unit::v0::GetPlayerNameResponse =
            self.request("getUnitPlayerName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_descriptor(
        &self,
        request: Request<unit::v0::GetDescriptorRequest>,
    ) -> Result<Response<unit::v0::GetDescriptorResponse>, Status> {
        let res: unit::v0::GetDescriptorResponse =
            self.request("getUnitDescriptor", request).await?;
        Ok(Response::new(res))
    }

    async fn set_emission(
        &self,
        request: Request<unit::v0::SetEmissionRequest>,
    ) -> Result<Response<unit::v0::SetEmissionResponse>, Status> {
        self.request("setEmission", request).await?;
        Ok(Response::new(unit::v0::SetEmissionResponse {}))
    }

    async fn get(
        &self,
        request: Request<unit::v0::GetRequest>,
    ) -> Result<Response<unit::v0::GetResponse>, Status> {
        let res: unit::v0::GetResponse = self.request("getUnit", request).await?;
        Ok(Response::new(res))
    }

    async fn get_transform(
        &self,
        request: Request<unit::v0::GetTransformRequest>,
    ) -> Result<Response<unit::v0::GetTransformResponse>, Status> {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Orientation {
            forward: common::v0::Vector,
            right: common::v0::Vector,
            up: common::v0::Vector,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Payload {
            position: common::v0::Position,
            u: f64,
            v: f64,
            position_north: common::v0::Vector,
            orientation: Orientation,
            velocity: common::v0::Vector,
            time: f64,
        }

        let res: Payload = self.request("getUnitTransform", request).await?;

        let projection_error = (res.position_north.x - res.u).atan2(res.position_north.z - res.v);

        let forward = &res.orientation.forward;
        let heading = forward.x.atan2(forward.z);

        Ok(Response::new(unit::v0::GetTransformResponse {
            position: Some(res.position),
            u: res.u,
            v: res.v,
            heading: heading.to_degrees(),
            orientation: Some({
                let right = &res.orientation.right;
                unit::v0::Orientation {
                    up: Some(res.orientation.up),
                    pitch: forward.y.asin().to_degrees(),
                    yaw: (heading - projection_error).to_degrees(),
                    roll: right.y.asin().neg().to_degrees(),
                    forward: Some(res.orientation.forward),
                    right: Some(res.orientation.right),
                }
            }),
            velocity: Some(res.velocity),
            time: res.time,
        }))
    }
}
