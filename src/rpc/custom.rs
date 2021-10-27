use super::dcs::custom::custom_service_server::CustomService;
use super::dcs::*;
use super::MissionRpc;
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
}
