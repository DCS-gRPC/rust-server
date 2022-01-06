use super::HookRpc;
use stubs::hook::v0::hook_service_server::HookService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl HookService for HookRpc {
    async fn get_mission_name(
        &self,
        request: Request<hook::v0::GetMissionNameRequest>,
    ) -> Result<Response<hook::v0::GetMissionNameResponse>, Status> {
        let res: hook::v0::GetMissionNameResponse = self.request("getMissionName", request).await?;
        Ok(Response::new(res))
    }

    async fn eval(
        &self,
        request: Request<hook::v0::EvalRequest>,
    ) -> Result<Response<hook::v0::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: serde_json::Value = self.request("hookEval", request).await?;
        let json = serde_json::to_string(&json).map_err(|err| {
            Status::internal(format!("failed to deserialize eval result: {}", err))
        })?;
        Ok(Response::new(hook::v0::EvalResponse { json }))
    }
}
