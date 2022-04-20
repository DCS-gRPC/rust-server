use super::MissionRpc;
use stubs::timer::v0::timer_service_server::TimerService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl TimerService for MissionRpc {
    async fn get_time(
        &self,
        request: Request<timer::v0::GetTimeRequest>,
    ) -> Result<Response<timer::v0::GetTimeResponse>, Status> {
        let res = self.request("getTime", request).await?;
        Ok(Response::new(res))
    }

    async fn get_absolute_time(
        &self,
        request: Request<timer::v0::GetAbsoluteTimeRequest>,
    ) -> Result<Response<timer::v0::GetAbsoluteTimeResponse>, Status> {
        let res = self.request("getAbsoluteTime", request).await?;
        Ok(Response::new(res))
    }

    async fn get_time_zero(
        &self,
        request: Request<timer::v0::GetTimeZeroRequest>,
    ) -> Result<Response<timer::v0::GetTimeZeroResponse>, Status> {
        let res = self.request("getTimeZero", request).await?;
        Ok(Response::new(res))
    }
}
