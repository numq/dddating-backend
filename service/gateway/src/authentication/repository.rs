use tonic::Request;

use crate::authentication::api::AuthenticationApi;
use crate::authentication::pb::ValidateTokenRequest;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait AuthenticationRepository {
    async fn is_token_valid(&self, access_token: &str) -> Result<bool, Error>;
}

pub struct AuthenticationRepositoryImpl {
    service: Box<dyn AuthenticationApi + Send + Sync>,
}

impl AuthenticationRepositoryImpl {
    pub fn new(service: Box<dyn AuthenticationApi + Send + Sync>) -> Box<dyn AuthenticationRepository + Send + Sync> {
        Box::new(AuthenticationRepositoryImpl { service })
    }
}

#[async_trait::async_trait]
impl AuthenticationRepository for AuthenticationRepositoryImpl {
    async fn is_token_valid(&self, access_token: &str) -> Result<bool, Error> {
        self.service.validate_token(Request::new(
            ValidateTokenRequest {
                access_token: String::from(access_token)
            }
        )).await?;
        Ok(true)
    }
}