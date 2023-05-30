use tonic::{Code, Request, Response, Status};
use tonic::transport::Channel;

use crate::account::pb::{CreateAccountRequest, CreateAccountResponse, DeleteAccountRequest, DeleteAccountResponse, GetAccountByCredentialsRequest, GetAccountByCredentialsResponse, GetAccountByIdRequest, GetAccountByIdResponse, UpdateAccountRequest, UpdateAccountResponse};
use crate::account::pb::account_service_client::AccountServiceClient;
use crate::account::pb::account_service_server::AccountService;

pub struct AccountApi {
    client: AccountServiceClient<Channel>,
}

impl AccountApi {
    pub fn new(client: AccountServiceClient<Channel>) -> Box<dyn AccountService + Send + Sync> {
        Box::new(AccountApi { client })
    }
}

#[tonic::async_trait]
impl AccountService for AccountApi {
    async fn get_account_by_id(&self, request: Request<GetAccountByIdRequest>) -> Result<Response<GetAccountByIdResponse>, Status> {
        let GetAccountByIdRequest { id } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        self.client
            .clone()
            .get_account_by_id(
                Request::new(GetAccountByIdRequest { id })
            ).await
    }

    async fn get_account_by_credentials(&self, request: Request<GetAccountByCredentialsRequest>) -> Result<Response<GetAccountByCredentialsResponse>, Status> {
        let GetAccountByCredentialsRequest { email, password } = request.into_inner();
        if email.is_empty() || password.is_empty() {
            return status::Status::invalid_arguments(vec!["email", "password"]);
        }

        self.client
            .clone()
            .get_account_by_credentials(
                Request::new(GetAccountByCredentialsRequest { email, password })
            ).await
    }

    async fn create_account(&self, request: Request<CreateAccountRequest>) -> Result<Response<CreateAccountResponse>, Status> {
        let CreateAccountRequest { email, password, role } = request.into_inner();
        if email.is_empty() || password.is_empty() {
            return status::Status::invalid_arguments(vec!["email", "password"]);
        }

        self.client
            .clone()
            .create_account(
                Request::new(CreateAccountRequest { email, password, role })
            ).await
    }

    async fn update_account(&self, request: Request<UpdateAccountRequest>) -> Result<Response<UpdateAccountResponse>, Status> {
        let UpdateAccountRequest { id, email, password, role } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        self.client
            .clone()
            .update_account(
                Request::new(UpdateAccountRequest { id, email, password, role })
            ).await
    }

    async fn delete_account(&self, request: Request<DeleteAccountRequest>) -> Result<Response<DeleteAccountResponse>, Status> {
        let DeleteAccountRequest { id } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        self.client.clone().delete_account(
            Request::new(DeleteAccountRequest { id })
        ).await
    }
}