use stubs::controller;
use stubs::controller::v0::controller_service_server::ControllerService;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[tonic::async_trait]
impl ControllerService for MissionRpc {
    async fn set_alarm_state(
        &self,
        request: Request<controller::v0::SetAlarmStateRequest>,
    ) -> Result<Response<controller::v0::SetAlarmStateResponse>, Status> {
        let res = self.request("setAlarmState", request).await?;
        Ok(Response::new(res))
    }
    async fn get_detected_targets(
        &self,
        request: Request<controller::v0::GetDetectedTargetsRequest>,
    ) -> Result<Response<controller::v0::GetDetectedTargetsResponse>, Status> {
        let res = self.request("getDetectedTargets", request).await?;
        Ok(Response::new(res))
    }
    async fn perform_orbit_task(
        &self,
        request: Request<controller::v0::PerformOrbitTaskRequest>,
    ) -> Result<Response<controller::v0::PerformOrbitTaskResponse>, Status> {
        let res = self.request("performOrbitTask", request).await?;
        Ok(Response::new(res))
    }
    async fn perform_racetrack_task(
        &self,
        request: Request<controller::v0::PerformRacetrackTaskRequest>,
    ) -> Result<Response<controller::v0::PerformRacetrackTaskResponse>, Status> {
        let res = self.request("performRacetrackTask", request).await?;
        Ok(Response::new(res))
    }
}
