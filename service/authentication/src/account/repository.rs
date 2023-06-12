use async_trait::async_trait;
use tonic::Request;

use crate::account::entity::{Account, Role};
use crate::account::pb::{CreateAccountRequest, CreateAccountResponse, DeleteAccountRequest, DeleteAccountResponse, GetAccountByCredentialsRequest, GetAccountByCredentialsResponse, GetAccountByIdRequest, GetAccountByIdResponse, Role as RoleMessage, UpdateAccountRequest, UpdateAccountResponse};
use crate::account::pb::account_service_server::AccountService;

type Error = Box<dyn error::Error + Send + Sync>;

#[async_trait]
pub trait AccountRepository {
    async fn get_account_by_id(&self, id: &str) -> Result<Option<Account>, Error>;
    async fn get_account_by_credentials(&self, email: &str, password: &str) -> Result<Option<Account>, Error>;
    async fn create_account(
        &self,
        email: &str,
        password: &str,
        role: Role,
    ) -> Result<String, Error>;
    async fn update_account(
        &self,
        id: &str,
        email: Option<String>,
        password: Option<String>,
        role: Option<Role>,
    ) -> Result<Option<Account>, Error>;
    async fn delete_account(&self, id: &str) -> Result<String, Error>;
}

pub struct AccountRepositoryImpl {
    api: Box<dyn AccountService + Send + Sync>,
}

impl AccountRepositoryImpl {
    pub fn new(api: Box<dyn AccountService + Send + Sync>) -> Box<dyn AccountRepository + Send + Sync> {
        Box::new(AccountRepositoryImpl { api })
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn get_account_by_id(&self, id: &str) -> Result<Option<Account>, Error> {
        let GetAccountByIdResponse { account } = self.api.get_account_by_id(
            Request::new(
                GetAccountByIdRequest {
                    id: String::from(id)
                }
            )
        ).await?.into_inner();
        Ok(account.map(|a| a.into()))
    }

    async fn get_account_by_credentials(&self, email: &str, password: &str) -> Result<Option<Account>, Error> {
        let GetAccountByCredentialsResponse { account } = self.api.get_account_by_credentials(
            Request::new(
                GetAccountByCredentialsRequest {
                    email: String::from(email),
                    password: String::from(password),
                }
            )
        ).await?.into_inner();
        Ok(account.map(|a| a.into()))
    }

    async fn create_account(&self, email: &str, password: &str, role: Role) -> Result<String, Error> {
        let CreateAccountResponse { id } = self.api.create_account(
            Request::new(
                CreateAccountRequest {
                    email: String::from(email),
                    password: String::from(password),
                    role: i32::from(match role {
                        Role::User => RoleMessage::User,
                        Role::Moderator => RoleMessage::Moderator
                    }),
                }
            )
        ).await?.into_inner();
        Ok(id)
    }

    async fn update_account(&self, id: &str, email: Option<String>, password: Option<String>, role: Option<Role>) -> Result<Option<Account>, Error> {
        let UpdateAccountResponse { account } = self.api.update_account(
            Request::new(
                UpdateAccountRequest {
                    id: String::from(id),
                    email,
                    password,
                    role: role.map(|r| i32::from(match r {
                        Role::User => RoleMessage::User,
                        Role::Moderator => RoleMessage::Moderator
                    })),
                }
            )
        ).await?.into_inner();
        Ok(account.map(|a| a.into()))
    }

    async fn delete_account(&self, id: &str) -> Result<String, Error> {
        let DeleteAccountResponse { id } = self.api.delete_account(
            Request::new(
                DeleteAccountRequest {
                    id: String::from(id)
                }
            )
        ).await?.into_inner();
        Ok(id)
    }
}