use crate::config::AuthConfig;
use tonic::codegen::http::Request;
use tonic::transport::Body;
use tonic::{async_trait, Status};
use tonic_middleware::RequestInterceptor;

#[derive(Clone)]
pub struct AuthInterceptor {
    pub auth_config: AuthConfig,
}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn intercept(&self, req: Request<Body>) -> Result<Request<Body>, Status> {
        match req.headers().get("bearer").map(|v| v.to_str()) {
            Some(Ok(token)) => {
                //check if token is correct if auth is enabled
                if self.auth_config.enabled == false || token == self.auth_config.token {
                    Ok(req)
                } else {
                    Err(Status::unauthenticated("Unauthenticated"))
                }
            }
            _ => Err(Status::unauthenticated("Unauthenticated")),
        }
    }
}
