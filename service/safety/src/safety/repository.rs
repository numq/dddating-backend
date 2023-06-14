use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::options::FindOptions;

use error::make_error;

use crate::safety::entity::BlockedUser;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait SafetyRepository {
    async fn check_user_id(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<bool, Error>;
    async fn get_blocked_users(
        &self,
        from_id: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<String>, Error>;
    async fn block_user(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<(), Error>;
    async fn unblock_user(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<(), Error>;
}

pub struct SafetyRepositoryImpl {
    collection: Collection<BlockedUser>,
}

impl SafetyRepositoryImpl {
    pub fn new(collection: Collection<BlockedUser>) -> Box<dyn SafetyRepository + Send + Sync> {
        Box::new(SafetyRepositoryImpl { collection })
    }
}

#[async_trait::async_trait]
impl SafetyRepository for SafetyRepositoryImpl {
    async fn check_user_id(&self, from_id: &str, to_id: &str) -> Result<bool, Error> {
        let result = self.collection.find_one(doc! { "from_id": from_id, "to_id": to_id }, None).await?;
        Ok(result.is_some())
    }

    async fn get_blocked_users(&self, from_id: &str, skip: u64, limit: u64) -> Result<Vec<String>, Error> {
        let filter = doc! {"from_id": from_id };
        let mut user_ids: Vec<String> = vec![];
        let options = FindOptions::builder().skip(skip).limit(limit.try_into().ok()).build();
        let mut cursor = self.collection.find(filter, options).await?;
        while let Some(blocked_user) = cursor.try_next().await? {
            user_ids.push(blocked_user.to_id)
        }
        Ok(user_ids)
    }

    async fn block_user(&self, from_id: &str, to_id: &str) -> Result<(), Error> {
        let id = ObjectId::new().to_hex();
        let blocked_user = BlockedUser::new(&id, from_id, to_id);
        let result = self.collection.insert_one(blocked_user.clone(), None).await;
        if let Ok(_) = result {
            return Ok(());
        }
        Err(make_error!("unable to block user"))
    }

    async fn unblock_user(&self, from_id: &str, to_id: &str) -> Result<(), Error> {
        if self.collection.delete_one(doc! { "from_id": from_id, "to_id": to_id }, None).await?.deleted_count > 0 {
            return Ok(());
        };
        Err(make_error!("unable to block user"))
    }
}