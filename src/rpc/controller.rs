use super::MissionRpc;
use stubs::controller::controller_service_server::ControllerService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ControllerService for MissionRpc {
    async fn set_alarm_state(
        &self,
        request: Request<controller::SetAlarmStateRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("setAlarmState", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }
}
