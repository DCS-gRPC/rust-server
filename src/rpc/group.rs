use super::MissionRpc;
use stubs::group::v0::group_service_server::GroupService;
use stubs::*;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl GroupService for MissionRpc {
    async fn get_units(
        &self,
        request: Request<group::v0::GetUnitsRequest>,
    ) -> Result<Response<group::v0::GetUnitsResponse>, Status> {
        let res = self.request("getUnits", request).await?;
        Ok(Response::new(res))
    }

    async fn activate(
        &self,
        request: Request<group::v0::ActivateRequest>,
    ) -> Result<Response<group::v0::ActivateResponse>, Status> {
        let res = self.request("groupActivate", request).await?;
        Ok(Response::new(res))
    }

    async fn destroy(
        &self,
        request: Request<group::v0::DestroyRequest>,
    ) -> Result<Response<group::v0::DestroyResponse>, Status> {
        let res = self.request("groupDestroy", request).await?;
        Ok(Response::new(res))
    }
}
