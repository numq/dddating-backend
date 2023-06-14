use tonic::{Code, Request, Response, Status};

use crate::token::interactor::TokenInteractor;
use crate::token::pb::{GenerateAccessTokenRequest, GenerateAccessTokenResponse, GenerateRefreshTokenRequest, GenerateRefreshTokenResponse, InvalidateTokenRequest, InvalidateTokenResponse, ValidateTokenRequest, ValidateTokenResponse};
use crate::token::pb::token_service_server::TokenService;

pub struct TokenServiceImpl {
    interactor: Box<dyn TokenInteractor + Send + Sync>,
}

impl TokenServiceImpl {
    pub fn new(interactor: Box<dyn TokenInteractor + Send + Sync>) -> impl TokenService {
        TokenServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl TokenService for TokenServiceImpl {
    async fn generate_access_token(&self, request: Request<GenerateAccessTokenRequest>) -> Result<Response<GenerateAccessTokenResponse>, Status> {
        let GenerateAccessTokenRequest { payload } = request.into_inner();
        if payload.is_empty() {
            return status::Status::invalid_arguments(vec!["payload"]);
        }

        match self.interactor.generate_access_token(&payload).await {
            Ok(token) => Ok(
                Response::new(
                    GenerateAccessTokenResponse { token }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn generate_refresh_token(&self, request: Request<GenerateRefreshTokenRequest>) -> Result<Response<GenerateRefreshTokenResponse>, Status> {
        let GenerateRefreshTokenRequest { payload } = request.into_inner();
        if payload.is_empty() {
            return status::Status::invalid_arguments(vec!["payload"]);
        }

        match self.interactor.generate_refresh_token(&payload).await {
            Ok(token) => Ok(
                Response::new(
                    GenerateRefreshTokenResponse { token }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        let ValidateTokenRequest { token } = request.into_inner();
        if token.is_empty() {
            return status::Status::invalid_arguments(vec!["token"]);
        }

        match self.interactor.validate_token(&token).await {
            Ok(payload) => Ok(
                Response::new(
                    ValidateTokenResponse { payload }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn invalidate_token(&self, request: Request<InvalidateTokenRequest>) -> Result<Response<InvalidateTokenResponse>, Status> {
        let InvalidateTokenRequest { token } = request.into_inner();
        if token.is_empty() {
            return status::Status::invalid_arguments(vec!["token"]);
        }

        match self.interactor.invalidate_token(&token).await {
            Ok(_) => Ok(
                Response::new(
                    InvalidateTokenResponse::default()
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }
}