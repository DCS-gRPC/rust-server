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
        if !self.auth_config.enabled {
            Ok(req)
        } else {
            match req.headers().get("X-API-Key").map(|v| v.to_str()) {
                Some(Ok(token)) => {
                    //check if token is correct if auth is 
                    let mut client: Option<&String> = None;
                    for key in &self.auth_config.tokens {
                        if key.token == token {
                            client = Some(&key.client);
                            break;
                        }
                    }

                    if client.is_some() {
                        log::debug!("Authenticated client: {}", client.unwrap());
                        Ok(req)
                    } else {
                        Err(Status::unauthenticated("Unauthenticated"))
                    }
                }
                _ => Err(Status::unauthenticated("Unauthenticated")),
            }
        }
    }
}
