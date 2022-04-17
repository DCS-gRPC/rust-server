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

    async fn get_mission_filename(
        &self,
        request: Request<hook::v0::GetMissionFilenameRequest>,
    ) -> Result<Response<hook::v0::GetMissionFilenameResponse>, Status> {
        let res: hook::v0::GetMissionFilenameResponse =
            self.request("getMissionFilename", request).await?;
        Ok(Response::new(res))
    }

    async fn get_paused(
        &self,
        request: Request<hook::v0::GetPausedRequest>,
    ) -> Result<Response<hook::v0::GetPausedResponse>, Status> {
        let res: hook::v0::GetPausedResponse = self.request("getPaused", request).await?;
        Ok(Response::new(res))
    }

    async fn set_paused(
        &self,
        request: Request<hook::v0::SetPausedRequest>,
    ) -> Result<Response<hook::v0::SetPausedResponse>, Status> {
        self.notification("setPaused", request).await?;
        Ok(Response::new(hook::v0::SetPausedResponse {}))
    }

    async fn stop_mission(
        &self,
        request: Request<hook::v0::StopMissionRequest>,
    ) -> Result<Response<hook::v0::StopMissionResponse>, Status> {
        self.notification("stopMission", request).await?;
        Ok(Response::new(hook::v0::StopMissionResponse {}))
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

    async fn exit_process(
        &self,
        request: Request<hook::v0::ExitProcessRequest>,
    ) -> Result<Response<hook::v0::ExitProcessResponse>, Status> {
        self.notification("exitProcess", request).await?;
        Ok(Response::new(hook::v0::ExitProcessResponse {}))
    }

    async fn is_multiplayer(
        &self,
        request: Request<hook::v0::IsMultiplayerRequest>,
    ) -> Result<Response<hook::v0::IsMultiplayerResponse>, Status> {
        let res: hook::v0::IsMultiplayerResponse = self.request("isMultiplayer", request).await?;
        Ok(Response::new(res))
    }

    async fn is_server(
        &self,
        request: Request<hook::v0::IsServerRequest>,
    ) -> Result<Response<hook::v0::IsServerResponse>, Status> {
        let res: hook::v0::IsServerResponse = self.request("isServer", request).await?;
        Ok(Response::new(res))
    }
}
