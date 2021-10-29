use super::MissionRpc;
use stubs::trigger::trigger_service_server::TriggerService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl TriggerService for MissionRpc {
    async fn out_text(
        &self,
        request: Request<trigger::OutTextRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("outText", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn out_text_for_coalition(
        &self,
        request: Request<trigger::OutTextForCoalitionRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("outTextForCoalition", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn out_text_for_group(
        &self,
        request: Request<trigger::OutTextForGroupRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("outTextForGroup", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_user_flag(
        &self,
        request: Request<trigger::GetUserFlagRequest>,
    ) -> Result<Response<trigger::GetUserFlagResponse>, Status> {
        let res: trigger::GetUserFlagResponse = self.request("getUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<trigger::SetUserFlagRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("setUserFlag", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn mark_to_all(
        &self,
        request: Request<trigger::MarkToAllRequest>,
    ) -> Result<Response<trigger::MarkToAllResponse>, Status> {
        let res: trigger::MarkToAllResponse = self.request("markToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_coalition(
        &self,
        request: Request<trigger::MarkToCoalitionRequest>,
    ) -> Result<Response<trigger::MarkToCoalitionResponse>, Status> {
        let res: trigger::MarkToCoalitionResponse =
            self.request("markToCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_group(
        &self,
        request: Request<trigger::MarkToGroupRequest>,
    ) -> Result<Response<trigger::MarkToGroupResponse>, Status> {
        let res: trigger::MarkToGroupResponse = self.request("markToGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_mark(
        &self,
        request: Request<trigger::RemoveMarkRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("removeMark", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn explosion(
        &self,
        request: Request<trigger::ExplosionRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("explosion", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn smoke(
        &self,
        request: Request<trigger::SmokeRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("smoke", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn illumination_bomb(
        &self,
        request: Request<trigger::IlluminationBombRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("illuminationBomb", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn signal_flare(
        &self,
        request: Request<trigger::SignalFlareRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.notification("signalFlare", request).await?;
        Ok(Response::new(EmptyResponse {}))
    }
}
