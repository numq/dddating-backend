use error::make_error;

use crate::account::entity::{Account, Role};
use crate::account::repository::AccountRepository;
use crate::password::hasher::Hasher;

type Error = Box<dyn error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait AccountInteractor {
    async fn get_account_by_id(&self, id: &str) -> Result<Account, Error>;
    async fn get_account_by_credentials(&self, email: &str, password: &str) -> Result<Account, Error>;
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
    ) -> Result<Account, Error>;
    async fn delete_account(&self, id: &str) -> Result<String, Error>;
}

pub struct AccountInteractorImpl {
    password_hasher: Box<dyn Hasher + Send + Sync>,
    repository: Box<dyn AccountRepository + Send + Sync>,
}

impl AccountInteractorImpl {
    pub fn new(password_hasher: Box<dyn Hasher + Send + Sync>, repository: Box<dyn AccountRepository + Send + Sync>) -> Box<dyn AccountInteractor + Send + Sync> {
        Box::new(AccountInteractorImpl { password_hasher, repository })
    }
}

#[async_trait::async_trait]
impl AccountInteractor for AccountInteractorImpl {
    async fn get_account_by_id(&self, id: &str) -> Result<Account, Error> {
        self.repository.get_account_by_id(id).await
    }

    async fn get_account_by_credentials(&self, email: &str, password: &str) -> Result<Account, Error> {
        let account = self.repository.get_account_by_email(email).await?;
        if let Ok(_) = self.password_hasher.verify_password(password, &account.password_hash) {
            Ok(account)
        } else {
            Err(make_error!("invalid password"))
        }
    }

    async fn create_account(&self, email: &str, password: &str, role: Role) -> Result<String, Error> {
        let (hash, salt) = self.password_hasher.hash_password(password)?;
        self.repository.create_account(&email.to_string(), &hash, &salt, role).await
    }

    async fn update_account(&self, id: &str, email: Option<String>, password: Option<String>, role: Option<Role>) -> Result<Account, Error> {
        let (hash, salt) = match password {
            Some(password) => self.password_hasher
                .hash_password(&password)
                .map(|(h, s)| (Some(h), Some(s)))?,
            _ => (None, None)
        };
        self.repository.update_account(id, email, hash, salt, role).await
    }

    async fn delete_account(&self, id: &str) -> Result<String, Error> {
        self.repository.delete_account(id).await
    }
}