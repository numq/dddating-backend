use tonic::Request;

use crate::token::pb::{GenerateAccessTokenRequest, GenerateAccessTokenResponse, GenerateRefreshTokenRequest, GenerateRefreshTokenResponse, InvalidateTokenRequest, InvalidateTokenResponse, ValidateTokenRequest, ValidateTokenResponse};
use crate::token::pb::token_service_server::TokenService;

type Error = Box<dyn error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait TokenRepository {
    async fn generate_access_token(&self, payload: &str) -> Result<String, Error>;
    async fn generate_refresh_token(&self, payload: &str) -> Result<String, Error>;
    async fn validate_token(&self, token: &str) -> Result<String, Error>;
    async fn invalidate_token(&self, token: &str) -> Result<(), Error>;
}

pub struct TokenRepositoryImpl {
    api: Box<dyn TokenService + Send + Sync>,
}

impl TokenRepositoryImpl {
    pub fn new(api: Box<dyn TokenService + Send + Sync>) -> Box<dyn TokenRepository + Send + Sync> {
        Box::new(TokenRepositoryImpl { api })
    }
}

#[async_trait::async_trait]
impl TokenRepository for TokenRepositoryImpl {
    async fn generate_access_token(&self, payload: &str) -> Result<String, Error> {
        let GenerateAccessTokenResponse { token } = self.api.generate_access_token(
            Request::new(
                GenerateAccessTokenRequest { payload: String::from(payload) }
            )
        ).await?.into_inner();
        Ok(token)
    }

    async fn generate_refresh_token(&self, payload: &str) -> Result<String, Error> {
        let GenerateRefreshTokenResponse { token } = self.api.generate_refresh_token(
            Request::new(
                GenerateRefreshTokenRequest { payload: String::from(payload) }
            )
        ).await?.into_inner();
        Ok(token)
    }

    async fn validate_token(&self, token: &str) -> Result<String, Error> {
        let ValidateTokenResponse { payload } = self.api.validate_token(
            Request::new(
                ValidateTokenRequest { token: String::from(token) }
            )
        ).await?.into_inner();
        Ok(payload)
    }

    async fn invalidate_token(&self, token: &str) -> Result<(), Error> {
        let InvalidateTokenResponse {} = self.api.invalidate_token(
            Request::new(
                InvalidateTokenRequest { token: String::from(token) }
            )
        ).await?.into_inner();
        Ok(())
    }
}