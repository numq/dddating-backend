use tonic::{Code, Request, Response, Status};
use tonic::transport::Channel;

use crate::token::pb::{GenerateAccessTokenRequest, GenerateAccessTokenResponse, GenerateRefreshTokenRequest, GenerateRefreshTokenResponse, InvalidateTokenRequest, InvalidateTokenResponse, ValidateTokenRequest, ValidateTokenResponse};
use crate::token::pb::token_service_client::TokenServiceClient;
use crate::token::pb::token_service_server::TokenService;

pub struct TokenApi {
    client: TokenServiceClient<Channel>,
}

impl TokenApi {
    pub fn new(client: TokenServiceClient<Channel>) -> Box<dyn TokenService + Send + Sync> {
        Box::new(TokenApi { client })
    }
}

#[tonic::async_trait]
impl TokenService for TokenApi {
    async fn generate_access_token(&self, request: Request<GenerateAccessTokenRequest>) -> Result<Response<GenerateAccessTokenResponse>, Status> {
        let GenerateAccessTokenRequest { payload } = request.into_inner();
        if payload.is_empty() {
            return status::Status::invalid_arguments(vec!["payload"]);
        }

        self.client
            .clone()
            .generate_access_token(
                Request::new(GenerateAccessTokenRequest { payload })
            )
            .await
    }

    async fn generate_refresh_token(&self, request: Request<GenerateRefreshTokenRequest>) -> Result<Response<GenerateRefreshTokenResponse>, Status> {
        let GenerateRefreshTokenRequest { payload } = request.into_inner();
        if payload.is_empty() {
            return status::Status::invalid_arguments(vec!["payload"]);
        }

        self.client
            .clone()
            .generate_refresh_token(
                Request::new(GenerateRefreshTokenRequest { payload })
            ).await
    }

    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        let ValidateTokenRequest { token } = request.into_inner();
        if token.is_empty() {
            return status::Status::invalid_arguments(vec!["token"]);
        }

        self.client
            .clone()
            .validate_token(
                Request::new(ValidateTokenRequest { token })
            ).await
    }

    async fn invalidate_token(&self, request: Request<InvalidateTokenRequest>) -> Result<Response<InvalidateTokenResponse>, Status> {
        let InvalidateTokenRequest { token } = request.into_inner();
        if token.is_empty() {
            return status::Status::invalid_arguments(vec!["token"]);
        }

        self.client
            .clone()
            .invalidate_token(
                Request::new(InvalidateTokenRequest { token })
            ).await
    }
}