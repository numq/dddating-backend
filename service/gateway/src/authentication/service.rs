use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::authentication::pb::{ChangePasswordRequest, ChangePasswordResponse, RefreshTokenRequest, RefreshTokenResponse, SignInRequest, SignInResponse, SignOutRequest, SignOutResponse, SignUpRequest, SignUpResponse, ValidateTokenRequest, ValidateTokenResponse};
use crate::authentication::pb::authentication_service_client::AuthenticationServiceClient;
use crate::authentication::pb::authentication_service_server::AuthenticationService;

pub struct AuthenticationServiceImpl {
    client: AuthenticationServiceClient<Channel>,
}

impl AuthenticationServiceImpl {
    pub fn new(client: AuthenticationServiceClient<Channel>) -> impl AuthenticationService {
        AuthenticationServiceImpl { client }
    }
}

#[tonic::async_trait]
impl AuthenticationService for AuthenticationServiceImpl {
    async fn sign_up(&self, request: Request<SignUpRequest>) -> Result<Response<SignUpResponse>, Status> {
        self.client.clone().sign_up(request).await
    }

    async fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status> {
        self.client.clone().sign_in(request).await
    }

    async fn sign_out(&self, request: Request<SignOutRequest>) -> Result<Response<SignOutResponse>, Status> {
        self.client.clone().sign_out(request).await
    }

    async fn change_password(&self, request: Request<ChangePasswordRequest>) -> Result<Response<ChangePasswordResponse>, Status> {
        self.client.clone().change_password(request).await
    }

    async fn refresh_token(&self, request: Request<RefreshTokenRequest>) -> Result<Response<RefreshTokenResponse>, Status> {
        self.client.clone().refresh_token(request).await
    }

    async fn validate_token(&self, request: Request<ValidateTokenRequest>) -> Result<Response<ValidateTokenResponse>, Status> {
        self.client.clone().validate_token(request).await
    }
}