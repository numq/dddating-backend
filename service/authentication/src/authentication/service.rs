use std::error::Error;

use tonic::{Code, Request, Response, Status};

use crate::{account, authentication};
use crate::account::entity::Role;
use crate::authentication::entity::TokenPair;
use crate::authentication::pb::{ChangePasswordRequest, ChangePasswordResponse, RefreshTokenRequest, RefreshTokenResponse, Role as RoleMessage, SignInRequest, SignInResponse, SignOutRequest, SignOutResponse, SignUpRequest, SignUpResponse, ValidateTokenRequest, ValidateTokenResponse};
use crate::authentication::pb::authentication_service_server::AuthenticationService;

pub struct AuthenticationServiceImpl {
    interactor: Box<dyn authentication::interactor::AuthenticationInteractor + Send + Sync>,
}

impl AuthenticationServiceImpl {
    pub fn new(interactor: Box<dyn authentication::interactor::AuthenticationInteractor + Send + Sync>) -> impl AuthenticationService {
        AuthenticationServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl AuthenticationService for AuthenticationServiceImpl {
    async fn sign_up(&self, request: Request<SignUpRequest>) -> Result<Response<SignUpResponse>, Status> {
        let SignUpRequest { email, password, role } = request.into_inner();
        if email.is_empty() || password.is_empty() {
            return status::Status::invalid_arguments(vec!["email", "password"]);
        }

        let role = match RoleMessage::from_i32(role).unwrap() {
            RoleMessage::User => Role::User,
            RoleMessage::Moderator => Role::Moderator
        };
        match self.interactor.sign_up(&email, &password, role).await {
            Ok(TokenPair { access_token, refresh_token }) => Ok(
                Response::new(
                    SignUpResponse {
                        access_token,
                        refresh_token,
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status> {
        let SignInRequest { email, password } = request.into_inner();
        if email.is_empty() || password.is_empty() {
            return status::Status::invalid_arguments(vec!["email", "password"]);
        }

        match self.interactor.sign_in(&email, &password).await {
            Ok(TokenPair { access_token, refresh_token }) => Ok(
                Response::new(
                    SignInResponse {
                        access_token,
                        refresh_token,
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn sign_out(&self, request: Request<SignOutRequest>) -> Result<Response<SignOutResponse>, Status> {
        let SignOutRequest { access_token, refresh_token } = request.into_inner();
        if access_token.is_empty() || refresh_token.is_empty() {
            return status::Status::invalid_arguments(vec!["access_token", "refresh_token"]);
        }

        match self.interactor.sign_out(&access_token, &refresh_token).await {
            Ok(is_success) => Ok(
                Response::new(
                    SignOutResponse { is_success }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn change_password(&self, request: Request<ChangePasswordRequest>) -> Result<Response<ChangePasswordResponse>, Status> {
        let ChangePasswordRequest { access_token, refresh_token, new_password } = request.into_inner();
        if access_token.is_empty() || refresh_token.is_empty() || new_password.is_empty() {
            return status::Status::invalid_arguments(vec!["access_token", "refresh_token", "new_password"]);
        }

        match self.interactor.change_password(&access_token, &refresh_token, &new_password).await {
            Ok(TokenPair { access_token, refresh_token }) => Ok(
                Response::new(
                    ChangePasswordResponse {
                        access_token,
                        refresh_token,
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn refresh_token(&self, request: Request<RefreshTokenRequest>) -> Result<Response<RefreshTokenResponse>, Status> {
        let RefreshTokenRequest { refresh_token } = request.into_inner();
        if refresh_token.is_empty() {
            return status::Status::invalid_arguments(vec!["refresh_token"]);
        }

        match self.interactor.validate_token(&refresh_token).await {
            Ok(_) => {
                match self.interactor.refresh_token(&refresh_token).await {
                    Ok(TokenPair { access_token, refresh_token }) => Ok(
                        Response::new(
                            RefreshTokenResponse {
                                access_token,
                                refresh_token,
                            }
                        )
                    ),
                    _ => status::Status::unauthenticated("invalid token")
                }
            }
            Err(error) => status::Status::internal(error)
        }
    }

    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        let ValidateTokenRequest { access_token } = request.into_inner();
        if access_token.is_empty() {
            return status::Status::invalid_arguments(vec!["access_token"]);
        }

        match self.interactor.validate_token(&access_token).await {
            Ok(payload) => Ok(
                Response::new(
                    ValidateTokenResponse { payload }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }
}