use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::authentication::pb::{ValidateTokenRequest, ValidateTokenResponse};
use crate::authentication::pb::authentication_service_client::AuthenticationServiceClient;

#[tonic::async_trait]
pub trait AuthenticationApi {
    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status>;
}

pub struct AuthenticationApiImpl {
    client: AuthenticationServiceClient<Channel>,
}

impl AuthenticationApiImpl {
    pub fn new(client: AuthenticationServiceClient<Channel>) -> Box<dyn AuthenticationApi + Send + Sync> {
        Box::new(AuthenticationApiImpl { client })
    }
}

#[tonic::async_trait]
impl AuthenticationApi for AuthenticationApiImpl {
    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        self.client.clone().validate_token(request).await
    }
}