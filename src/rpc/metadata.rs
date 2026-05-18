use stubs::metadata::v0::metadata_service_server::MetadataService;
use stubs::*;
use tonic::{Request, Response, Status, async_trait};

use super::MissionRpc;

#[async_trait]
impl MetadataService for MissionRpc {
    async fn get_health(
        &self,
        _request: Request<metadata::v0::GetHealthRequest>,
    ) -> Result<Response<metadata::v0::GetHealthResponse>, Status> {
        let alive: bool = true;
        return Ok(Response::new(metadata::v0::GetHealthResponse { alive }));
    }

    async fn get_version(
        &self,
        _request: Request<metadata::v0::GetVersionRequest>,
    ) -> Result<Response<metadata::v0::GetVersionResponse>, Status> {
        const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
        let version = VERSION.unwrap_or("unknown").to_string();
        return Ok(Response::new(metadata::v0::GetVersionResponse { version }));
    }
}
