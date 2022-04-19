use std::ops::Neg;

use super::MissionRpc;
use stubs::custom::v0::custom_service_server::CustomService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl CustomService for MissionRpc {
    async fn request_mission_assignment(
        &self,
        request: Request<custom::v0::RequestMissionAssignmentRequest>,
    ) -> Result<Response<custom::v0::RequestMissionAssignmentResponse>, Status> {
        self.request("requestMissionAssignment", request).await?;
        Ok(Response::new(
            custom::v0::RequestMissionAssignmentResponse {},
        ))
    }

    async fn join_mission(
        &self,
        request: Request<custom::v0::JoinMissionRequest>,
    ) -> Result<Response<custom::v0::JoinMissionResponse>, Status> {
        self.request("joinMission", request).await?;
        Ok(Response::new(custom::v0::JoinMissionResponse {}))
    }

    async fn abort_mission(
        &self,
        request: Request<custom::v0::AbortMissionRequest>,
    ) -> Result<Response<custom::v0::AbortMissionResponse>, Status> {
        self.request("abortMission", request).await?;
        Ok(Response::new(custom::v0::AbortMissionResponse {}))
    }

    async fn get_mission_status(
        &self,
        request: Request<custom::v0::GetMissionStatusRequest>,
    ) -> Result<Response<custom::v0::GetMissionStatusResponse>, Status> {
        self.request("getMissionStatus", request).await?;
        Ok(Response::new(custom::v0::GetMissionStatusResponse {}))
    }

    async fn eval(
        &self,
        request: Request<custom::v0::EvalRequest>,
    ) -> Result<Response<custom::v0::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: serde_json::Value = self.request("missionEval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(custom::v0::EvalResponse { json }))
    }

    async fn get_magnetic_declination(
        &self,
        request: Request<custom::v0::GetMagneticDeclinationRequest>,
    ) -> Result<Response<custom::v0::GetMagneticDeclinationResponse>, Status> {
        let position = request.into_inner();

        // As only the date is relevant, and a difference of some days don't really matter, it is
        // fine to just use the scenario's start time, especially since it is cached and thus
        // prevents unnecessary roundtrips to the MSE.
        let date = self.get_scenario_start_time().await?.date();
        let declination = igrf::declination(position.lat, position.lon, position.alt as u32, date)
            .map(|f| f.d)
            .or_else(|err| match err {
                igrf::Error::DateOutOfRange(f) => Ok(f.d),
                err => Err(Status::internal(format!(
                    "failed to estimate magnetic declination: {}",
                    err
                ))),
            })?;

        // reduce precision to two decimal places
        let declination = ((declination * 100.0).round() / 100.0).neg();

        Ok(Response::new(custom::v0::GetMagneticDeclinationResponse {
            declination,
        }))
    }
}
