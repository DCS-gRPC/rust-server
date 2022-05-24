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
        let res = self.request("getMissionName", request).await?;
        Ok(Response::new(res))
    }

    async fn get_mission_description(
        &self,
        request: Request<hook::v0::GetMissionDescriptionRequest>,
    ) -> Result<Response<hook::v0::GetMissionDescriptionResponse>, Status> {
        let res = self.request("getMissionDescription", request).await?;
        Ok(Response::new(res))
    }

    async fn get_mission_filename(
        &self,
        request: Request<hook::v0::GetMissionFilenameRequest>,
    ) -> Result<Response<hook::v0::GetMissionFilenameResponse>, Status> {
        let res = self.request("getMissionFilename", request).await?;
        Ok(Response::new(res))
    }

    async fn get_paused(
        &self,
        request: Request<hook::v0::GetPausedRequest>,
    ) -> Result<Response<hook::v0::GetPausedResponse>, Status> {
        let res = self.request("getPaused", request).await?;
        Ok(Response::new(res))
    }

    async fn set_paused(
        &self,
        request: Request<hook::v0::SetPausedRequest>,
    ) -> Result<Response<hook::v0::SetPausedResponse>, Status> {
        let res = self.request("setPaused", request).await?;
        Ok(Response::new(res))
    }

    async fn reload_current_mission(
        &self,
        request: Request<hook::v0::ReloadCurrentMissionRequest>,
    ) -> Result<Response<hook::v0::ReloadCurrentMissionResponse>, Status> {
        let res = self.request("reloadCurrentMission", request).await?;
        Ok(Response::new(res))
    }

    async fn load_next_mission(
        &self,
        request: Request<hook::v0::LoadNextMissionRequest>,
    ) -> Result<Response<hook::v0::LoadNextMissionResponse>, Status> {
        let res = self.request("loadNextMission", request).await?;
        Ok(Response::new(res))
    }

    async fn load_mission(
        &self,
        request: Request<hook::v0::LoadMissionRequest>,
    ) -> Result<Response<hook::v0::LoadMissionResponse>, Status> {
        let res = self.request("loadMission", request).await?;
        Ok(Response::new(res))
    }

    async fn stop_mission(
        &self,
        request: Request<hook::v0::StopMissionRequest>,
    ) -> Result<Response<hook::v0::StopMissionResponse>, Status> {
        let res = self.request("stopMission", request).await?;
        Ok(Response::new(res))
    }

    async fn eval(
        &self,
        request: Request<hook::v0::EvalRequest>,
    ) -> Result<Response<hook::v0::EvalResponse>, Status> {
        if !self.eval_enabled {
            return Err(Status::permission_denied("eval operation is disabled"));
        }

        let json: String = self.request("hookEval", request).await?;
        Ok(Response::new(hook::v0::EvalResponse { json }))
    }

    async fn exit_process(
        &self,
        request: Request<hook::v0::ExitProcessRequest>,
    ) -> Result<Response<hook::v0::ExitProcessResponse>, Status> {
        let res = self.request("exitProcess", request).await?;
        Ok(Response::new(res))
    }

    async fn is_multiplayer(
        &self,
        request: Request<hook::v0::IsMultiplayerRequest>,
    ) -> Result<Response<hook::v0::IsMultiplayerResponse>, Status> {
        let res = self.request("isMultiplayer", request).await?;
        Ok(Response::new(res))
    }

    async fn is_server(
        &self,
        request: Request<hook::v0::IsServerRequest>,
    ) -> Result<Response<hook::v0::IsServerResponse>, Status> {
        let res = self.request("isServer", request).await?;
        Ok(Response::new(res))
    }

    async fn ban_player(
        &self,
        request: Request<hook::v0::BanPlayerRequest>,
    ) -> Result<Response<hook::v0::BanPlayerResponse>, Status> {
        let res = self.request("banPlayer", request).await?;
        Ok(Response::new(res))
    }

    async fn unban_player(
        &self,
        request: Request<hook::v0::UnbanPlayerRequest>,
    ) -> Result<Response<hook::v0::UnbanPlayerResponse>, Status> {
        let res = self.request("unbanPlayer", request).await?;
        Ok(Response::new(res))
    }

    async fn get_banned_players(
        &self,
        request: Request<hook::v0::GetBannedPlayersRequest>,
    ) -> Result<Response<hook::v0::GetBannedPlayersResponse>, Status> {
        let res = self.request("getBannedPlayers", request).await?;
        Ok(Response::new(res))
    }

    async fn get_unit_type(
        &self,
        request: Request<hook::v0::GetUnitTypeRequest>,
    ) -> Result<Response<hook::v0::GetUnitTypeResponse>, Status> {
        let res = self.request("getUnitType", request).await?;
        Ok(Response::new(res))
    }
}
