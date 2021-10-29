use std::ops::Neg;

use super::MissionRpc;
use stubs::custom::custom_service_server::CustomService;
use stubs::mission;
use stubs::mission::mission_service_server::MissionService;
use stubs::*;
use time::format_description::well_known::Rfc3339;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl CustomService for MissionRpc {
    async fn request_mission_assignment(
        &self,
        request: Request<custom::MissionAssignmentRequest>,
    ) -> Result<Response<custom::MissionAssignmentResponse>, Status> {
        self.notification("requestMissionAssignment", request)
            .await?;
        Ok(Response::new(custom::MissionAssignmentResponse {}))
    }

    async fn join_mission(
        &self,
        request: Request<custom::MissionJoinRequest>,
    ) -> Result<Response<custom::MissionJoinResponse>, Status> {
        self.notification("joinMission", request).await?;
        Ok(Response::new(custom::MissionJoinResponse {}))
    }

    async fn eval(
        &self,
        request: Request<custom::EvalRequest>,
    ) -> Result<Response<custom::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: serde_json::Value = self.request("missionEval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(custom::EvalResponse { json }))
    }

    async fn get_magnetic_declination(
        &self,
        request: Request<custom::GetMagneticDeclinationRequest>,
    ) -> Result<Response<custom::GetMagneticDeclinationResponse>, Status> {
        let position = request.into_inner();
        let datetime = self
            .get_scenario_current_time(Request::new(mission::GetScenarioCurrentTimeRequest {}))
            .await?
            .into_inner()
            .datetime;
        let date = time::Date::parse(&datetime, &Rfc3339).map_err(|err| {
            Status::internal(format!("failed to parse scenario's current time: {}", err))
        })?;

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

        Ok(Response::new(custom::GetMagneticDeclinationResponse {
            declination,
        }))
    }
}
