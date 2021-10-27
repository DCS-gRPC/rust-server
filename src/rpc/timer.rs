use super::dcs::timer::timer_service_server::TimerService;
use super::dcs::*;
use super::MissionRpc;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl TimerService for MissionRpc {
    async fn get_time(
        &self,
        request: Request<timer::GetTimeRequest>,
    ) -> Result<Response<timer::GetTimeResponse>, Status> {
        let res: timer::GetTimeResponse = self.request("getTime", request).await?;
        Ok(Response::new(res))
    }

    async fn get_absolute_time(
        &self,
        request: Request<timer::GetAbsoluteTimeRequest>,
    ) -> Result<Response<timer::GetAbsoluteTimeResponse>, Status> {
        let res: timer::GetAbsoluteTimeResponse = self.request("getAbsoluteTime", request).await?;
        Ok(Response::new(res))
    }

    async fn get_time_zero(
        &self,
        request: Request<timer::GetTimeZeroRequest>,
    ) -> Result<Response<timer::GetTimeZeroResponse>, Status> {
        let res: timer::GetTimeZeroResponse = self.request("getTimeZero", request).await?;
        Ok(Response::new(res))
    }
}
