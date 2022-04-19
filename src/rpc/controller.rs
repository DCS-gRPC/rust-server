use super::MissionRpc;
use stubs::controller;
use stubs::controller::v0::controller_service_server::ControllerService;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ControllerService for MissionRpc {
    async fn set_alarm_state(
        &self,
        request: Request<controller::v0::SetAlarmStateRequest>,
    ) -> Result<Response<controller::v0::SetAlarmStateResponse>, Status> {
        self.request("setAlarmState", request).await?;
        Ok(Response::new(controller::v0::SetAlarmStateResponse {}))
    }
}
