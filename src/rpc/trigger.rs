use super::MissionRpc;
use stubs::trigger;
use stubs::trigger::v0::trigger_service_server::TriggerService;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl TriggerService for MissionRpc {
    async fn out_text(
        &self,
        request: Request<trigger::v0::OutTextRequest>,
    ) -> Result<Response<trigger::v0::OutTextResponse>, Status> {
        self.notification("outText", request).await?;
        Ok(Response::new(trigger::v0::OutTextResponse {}))
    }

    async fn out_text_for_coalition(
        &self,
        request: Request<trigger::v0::OutTextForCoalitionRequest>,
    ) -> Result<Response<trigger::v0::OutTextForCoalitionResponse>, Status> {
        self.notification("outTextForCoalition", request).await?;
        Ok(Response::new(trigger::v0::OutTextForCoalitionResponse {}))
    }

    async fn out_text_for_group(
        &self,
        request: Request<trigger::v0::OutTextForGroupRequest>,
    ) -> Result<Response<trigger::v0::OutTextForGroupResponse>, Status> {
        self.notification("outTextForGroup", request).await?;
        Ok(Response::new(trigger::v0::OutTextForGroupResponse {}))
    }

    async fn get_user_flag(
        &self,
        request: Request<trigger::v0::GetUserFlagRequest>,
    ) -> Result<Response<trigger::v0::GetUserFlagResponse>, Status> {
        let res: trigger::v0::GetUserFlagResponse = self.request("getUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<trigger::v0::SetUserFlagRequest>,
    ) -> Result<Response<trigger::v0::SetUserFlagResponse>, Status> {
        self.notification("setUserFlag", request).await?;
        Ok(Response::new(trigger::v0::SetUserFlagResponse {}))
    }

    async fn mark_to_all(
        &self,
        request: Request<trigger::v0::MarkToAllRequest>,
    ) -> Result<Response<trigger::v0::MarkToAllResponse>, Status> {
        let res: trigger::v0::MarkToAllResponse = self.request("markToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_coalition(
        &self,
        request: Request<trigger::v0::MarkToCoalitionRequest>,
    ) -> Result<Response<trigger::v0::MarkToCoalitionResponse>, Status> {
        let res: trigger::v0::MarkToCoalitionResponse =
            self.request("markToCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_group(
        &self,
        request: Request<trigger::v0::MarkToGroupRequest>,
    ) -> Result<Response<trigger::v0::MarkToGroupResponse>, Status> {
        let res: trigger::v0::MarkToGroupResponse = self.request("markToGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_mark(
        &self,
        request: Request<trigger::v0::RemoveMarkRequest>,
    ) -> Result<Response<trigger::v0::RemoveMarkResponse>, Status> {
        self.notification("removeMark", request).await?;
        Ok(Response::new(trigger::v0::RemoveMarkResponse {}))
    }

    async fn explosion(
        &self,
        request: Request<trigger::v0::ExplosionRequest>,
    ) -> Result<Response<trigger::v0::ExplosionResponse>, Status> {
        self.notification("explosion", request).await?;
        Ok(Response::new(trigger::v0::ExplosionResponse {}))
    }

    async fn smoke(
        &self,
        request: Request<trigger::v0::SmokeRequest>,
    ) -> Result<Response<trigger::v0::SmokeResponse>, Status> {
        self.notification("smoke", request).await?;
        Ok(Response::new(trigger::v0::SmokeResponse {}))
    }

    async fn illumination_bomb(
        &self,
        request: Request<trigger::v0::IlluminationBombRequest>,
    ) -> Result<Response<trigger::v0::IlluminationBombResponse>, Status> {
        self.notification("illuminationBomb", request).await?;
        Ok(Response::new(trigger::v0::IlluminationBombResponse {}))
    }

    async fn signal_flare(
        &self,
        request: Request<trigger::v0::SignalFlareRequest>,
    ) -> Result<Response<trigger::v0::SignalFlareResponse>, Status> {
        self.notification("signalFlare", request).await?;
        Ok(Response::new(trigger::v0::SignalFlareResponse {}))
    }
}
