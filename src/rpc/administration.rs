use stubs::administration::v0::administration_service_server::AdministrationService;
use stubs::*;
use tonic::async_trait;
use tonic::{Request, Response, Status};

use super::MissionRpc;

#[async_trait]
impl AdministrationService for MissionRpc {
    async fn get_health(
        &self,  
        _request: Request<administration::v0::GetHealthRequest>,
    ) -> Result<Response<administration::v0::GetHealthResponse>, Status> {
        let alive : bool = true;
        return Ok(Response::new(administration::v0::GetHealthResponse { 
            alive,
        }))
    }

    async fn get_version(
        &self,  
        _request: Request<administration::v0::GetVersionRequest>,
    ) -> Result<Response<administration::v0::GetVersionResponse>, Status>
    {
        const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
        let version = VERSION.unwrap_or("unknown").to_string();
        return Ok(Response::new(administration::v0::GetVersionResponse {
            version,
        }));
    }
}