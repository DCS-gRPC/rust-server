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
        let res = self.request("outText", request).await?;
        Ok(Response::new(res))
    }

    async fn out_text_for_coalition(
        &self,
        request: Request<trigger::v0::OutTextForCoalitionRequest>,
    ) -> Result<Response<trigger::v0::OutTextForCoalitionResponse>, Status> {
        let res = self.request("outTextForCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn out_text_for_group(
        &self,
        request: Request<trigger::v0::OutTextForGroupRequest>,
    ) -> Result<Response<trigger::v0::OutTextForGroupResponse>, Status> {
        let res = self.request("outTextForGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn out_text_for_unit(
        &self,
        request: Request<trigger::v0::OutTextForUnitRequest>,
    ) -> Result<Response<trigger::v0::OutTextForUnitResponse>, Status> {
        let res = self.request("outTextForUnit", request).await?;
        Ok(Response::new(res))
    }

    async fn get_user_flag(
        &self,
        request: Request<trigger::v0::GetUserFlagRequest>,
    ) -> Result<Response<trigger::v0::GetUserFlagResponse>, Status> {
        let res = self.request("getUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn set_user_flag(
        &self,
        request: Request<trigger::v0::SetUserFlagRequest>,
    ) -> Result<Response<trigger::v0::SetUserFlagResponse>, Status> {
        let res = self.request("setUserFlag", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_all(
        &self,
        request: Request<trigger::v0::MarkToAllRequest>,
    ) -> Result<Response<trigger::v0::MarkToAllResponse>, Status> {
        let res = self.request("markToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_coalition(
        &self,
        request: Request<trigger::v0::MarkToCoalitionRequest>,
    ) -> Result<Response<trigger::v0::MarkToCoalitionResponse>, Status> {
        let res = self.request("markToCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn mark_to_group(
        &self,
        request: Request<trigger::v0::MarkToGroupRequest>,
    ) -> Result<Response<trigger::v0::MarkToGroupResponse>, Status> {
        let res = self.request("markToGroup", request).await?;
        Ok(Response::new(res))
    }

    async fn remove_mark(
        &self,
        request: Request<trigger::v0::RemoveMarkRequest>,
    ) -> Result<Response<trigger::v0::RemoveMarkResponse>, Status> {
        let res = self.request("removeMark", request).await?;
        Ok(Response::new(res))
    }

    async fn markup_to_all(
        &self,
        request: Request<trigger::v0::MarkupToAllRequest>,
    ) -> Result<Response<trigger::v0::MarkupToAllResponse>, Status> {
        let res = self.request("markupToAll", request).await?;
        Ok(Response::new(res))
    }

    async fn markup_to_coalition(
        &self,
        request: Request<trigger::v0::MarkupToCoalitionRequest>,
    ) -> Result<Response<trigger::v0::MarkupToCoalitionResponse>, Status> {
        let res = self.request("markupToCoalition", request).await?;
        Ok(Response::new(res))
    }

    async fn explosion(
        &self,
        request: Request<trigger::v0::ExplosionRequest>,
    ) -> Result<Response<trigger::v0::ExplosionResponse>, Status> {
        let res = self.request("explosion", request).await?;
        Ok(Response::new(res))
    }

    async fn smoke(
        &self,
        request: Request<trigger::v0::SmokeRequest>,
    ) -> Result<Response<trigger::v0::SmokeResponse>, Status> {
        let res = self.request("smoke", request).await?;
        Ok(Response::new(res))
    }

    async fn illumination_bomb(
        &self,
        request: Request<trigger::v0::IlluminationBombRequest>,
    ) -> Result<Response<trigger::v0::IlluminationBombResponse>, Status> {
        let res = self.request("illuminationBomb", request).await?;
        Ok(Response::new(res))
    }

    async fn signal_flare(
        &self,
        request: Request<trigger::v0::SignalFlareRequest>,
    ) -> Result<Response<trigger::v0::SignalFlareResponse>, Status> {
        let res = self.request("signalFlare", request).await?;
        Ok(Response::new(res))
    }
}
