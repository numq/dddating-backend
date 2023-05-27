use tonic::{Code, Request, Response, Status};

use crate::account::entity::Role;
use crate::account::interactor::AccountInteractor;
use crate::account::pb::{CreateAccountRequest, CreateAccountResponse, DeleteAccountRequest, DeleteAccountResponse, GetAccountByCredentialsRequest, GetAccountByCredentialsResponse, GetAccountByIdRequest, GetAccountByIdResponse, Role as RoleMessage, UpdateAccountRequest, UpdateAccountResponse};
use crate::account::pb::account_service_server::AccountService;

pub struct AccountServiceImpl {
    interactor: Box<dyn AccountInteractor + Send + Sync>,
}

impl AccountServiceImpl {
    pub fn new(interactor: Box<dyn AccountInteractor + Send + Sync>) -> impl AccountService {
        AccountServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl AccountService for AccountServiceImpl {
    async fn get_account_by_id(&self, request: Request<GetAccountByIdRequest>) -> Result<Response<GetAccountByIdResponse>, Status> {
        let GetAccountByIdRequest { id } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.get_account_by_id(&id).await {
            Ok(account) => Ok(Response::new(GetAccountByIdResponse { account: Some(account.into()) })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_account_by_credentials(&self, request: Request<GetAccountByCredentialsRequest>) -> Result<Response<GetAccountByCredentialsResponse>, Status> {
        let GetAccountByCredentialsRequest { email, password } = request.into_inner();
        if email.is_empty() || password.is_empty() {
            return status::Status::invalid_arguments(vec!["email", "password"]);
        }

        match self.interactor.get_account_by_credentials(&email, &password).await {
            Ok(account) => Ok(Response::new(GetAccountByCredentialsResponse { account: Some(account.into()) })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn create_account(&self, request: Request<CreateAccountRequest>) -> Result<Response<CreateAccountResponse>, Status> {
        let CreateAccountRequest { email, password, role } = request.into_inner();
        if email.is_empty() || password.is_empty() {
            return status::Status::invalid_arguments(vec!["email", "password"]);
        }

        match self.interactor.create_account(&email, &password, match RoleMessage::from_i32(role).unwrap() {
            RoleMessage::User => Role::User,
            RoleMessage::Moderator => Role::Moderator
        }).await {
            Ok(id) => Ok(Response::new(CreateAccountResponse { id })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn update_account(&self, request: Request<UpdateAccountRequest>) -> Result<Response<UpdateAccountResponse>, Status> {
        let UpdateAccountRequest { id, email, password, role } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.update_account(&id, email, password, role.map(|r| match RoleMessage::from_i32(r).unwrap() {
            RoleMessage::User => Role::User,
            RoleMessage::Moderator => Role::Moderator
        })).await {
            Ok(account) => Ok(Response::new(UpdateAccountResponse { account: Some(account.into()) })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn delete_account(&self, request: Request<DeleteAccountRequest>) -> Result<Response<DeleteAccountResponse>, Status> {
        let DeleteAccountRequest { id } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.delete_account(&id).await {
            Ok(id) => Ok(Response::new(DeleteAccountResponse { id })),
            Err(error) => status::Status::internal(error)
        }
    }
}