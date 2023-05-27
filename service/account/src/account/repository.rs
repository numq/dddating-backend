use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use error::make_error;

use crate::account::entity::{Account, Role};

type Error = Box<dyn error::Error + Send + Sync>;

#[async_trait]
pub trait AccountRepository {
    async fn get_account_by_id(&self, id: &str) -> Result<Account, Error>;
    async fn get_account_by_email(&self, email: &str) -> Result<Account, Error>;
    async fn create_account(
        &self,
        email: &str,
        password_hash: &str,
        password_salt: &str,
        role: Role,
    ) -> Result<String, Error>;
    async fn update_account(
        &self,
        id: &str,
        email: Option<String>,
        password_hash: Option<String>,
        password_salt: Option<String>,
        role: Option<Role>,
    ) -> Result<Account, Error>;
    async fn delete_account(&self, id: &str) -> Result<String, Error>;
}

pub struct AccountRepositoryImpl {
    collection: Collection<Account>,
}

impl AccountRepositoryImpl {
    pub fn new(collection: Collection<Account>) -> Box<dyn AccountRepository + Send + Sync> {
        Box::new(AccountRepositoryImpl { collection })
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn get_account_by_id(&self, id: &str) -> Result<Account, Error> {
        if let Some(account) = self.collection.find_one(doc! { "_id": id }, None).await? {
            return Ok(account);
        }
        Err(make_error!("unable to get account by id"))
    }


    async fn get_account_by_email(&self, email: &str) -> Result<Account, Error> {
        if let Some(account) = self.collection.find_one(doc! { "email": email }, None).await? {
            return Ok(account);
        }
        Err(make_error!("unable to get account by email and password"))
    }

    async fn create_account(
        &self,
        email: &str,
        password_hash: &str,
        password_salt: &str,
        role: Role,
    ) -> Result<String, Error> {
        let id = ObjectId::new().to_hex();
        let account = Account::new(&id, email, password_hash, password_salt, role);
        let result = self.collection.insert_one(account, None).await?;
        if let Some(id) = result.inserted_id.as_str() {
            return Ok(String::from(id));
        }
        Err(make_error!("unable to create account"))
    }

    async fn update_account(
        &self,
        id: &str,
        email: Option<String>,
        password_hash: Option<String>,
        password_salt: Option<String>,
        role: Option<Role>,
    ) -> Result<Account, Error> {
        let timestamp = bson!(Account::timestamp_now() as i64);
        let mut document = doc! { "updated_at": timestamp };
        if let Some(email) = email {
            document.insert("email", email);
        }
        if !password_hash.is_none() && !password_salt.is_none() {
            document.insert("password_hash", password_hash.unwrap());
            document.insert("password_salt", password_salt.unwrap());
        }
        if let Some(role) = role {
            document.insert("role", match role {
                Role::User => 0,
                Role::Moderator => 1
            });
        }
        let result = self.collection.update_one(doc! { "_id": id }, document, None).await?;
        if result.modified_count > 0 {
            return self.get_account_by_id(id).await;
        }
        Err(make_error!("unable to update account"))
    }

    async fn delete_account(&self, id: &str) -> Result<String, Error> {
        if self.collection.delete_one(doc! { "_id": id }, None).await?.deleted_count > 0 {
            return Ok(String::from(id));
        }
        Err(make_error!("unable to delete account"))
    }
}