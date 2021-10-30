use super::MissionRpc;
use stubs::controller::controller_service_server::ControllerService;
use stubs::{common, controller};
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ControllerService for MissionRpc {
    async fn set_alarm_state(
        &self,
        request: Request<controller::SetAlarmStateRequest>,
    ) -> Result<Response<common::EmptyResponse>, Status> {
        self.notification("setAlarmState", request).await?;
        Ok(Response::new(common::EmptyResponse {}))
    }
}
