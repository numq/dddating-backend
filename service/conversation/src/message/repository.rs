use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::options::FindOptions;

use error::make_error;

use crate::message::entity::Message;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait MessageRepository {
    async fn get_messages(
        &self,
        chat_id: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Message>, Error>;
    async fn get_message_by_id(&self, message_id: &str) -> Result<Option<Message>, Error>;
    async fn create_message(
        &self,
        chat_id: &str,
        member_id: &str,
        text: Option<String>,
        images: Vec<Vec<u8>>,
    ) -> Result<Message, Error>;
    async fn delete_message(&self, message_id: &str) -> Result<String, Error>;
}

pub struct MessageRepositoryImpl {
    collection: Collection<Message>,
}

impl MessageRepositoryImpl {
    pub fn new(collection: Collection<Message>) -> Box<dyn MessageRepository + Send + Sync> {
        Box::new(MessageRepositoryImpl { collection })
    }
}

#[async_trait::async_trait]
impl MessageRepository for MessageRepositoryImpl {
    async fn get_messages(&self, chat_id: &str, skip: u64, limit: u64) -> Result<Vec<Message>, Error> {
        let filter = doc! {"chat_id": chat_id };
        let mut messages: Vec<Message> = vec![];
        let options = FindOptions::builder().skip(skip).limit(limit.try_into().ok()).build();
        let mut cursor = self.collection.find(filter, options).await?;
        while let Some(message) = cursor.try_next().await? {
            messages.push(message)
        }
        Ok(messages)
    }

    async fn get_message_by_id(&self, message_id: &str) -> Result<Option<Message>, Error> {
        if let Ok(message) = self.collection.find_one(doc! { "_id": message_id }, None).await {
            return Ok(message);
        }
        Err(make_error!("unable to get message by id"))
    }

    async fn create_message(&self, chat_id: &str, member_id: &str, text: Option<String>, images: Vec<Vec<u8>>) -> Result<Message, Error> {
        let id = ObjectId::new().to_hex();
        let message = Message::new(&id, chat_id, member_id, text, images);
        let result = self.collection.insert_one(message, None).await?;
        if let Some(id) = result.inserted_id.as_str() {
            if let Some(message) = self.get_message_by_id(id).await? {
                return Ok(message);
            }
        }
        Err(make_error!("unable to create message"))
    }

    async fn delete_message(&self, message_id: &str) -> Result<String, Error> {
        if self.collection.delete_one(doc! { "_id": message_id }, None).await?.deleted_count > 0 {
            return Ok(String::from(message_id));
        }
        Err(make_error!("unable to delete message"))
    }
}