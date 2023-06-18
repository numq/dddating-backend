use futures::TryStreamExt;
use mongodb::{bson, Collection};
use mongodb::bson::{bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;

use error::make_error;

use crate::chat::entity::Chat;
use crate::message::entity::Message;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait ChatRepository {
    async fn get_chats(
        &self,
        member_id: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Chat>, Error>;
    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Option<Chat>, Error>;
    async fn create_chat(&self, member_ids: Vec<String>) -> Result<Option<Chat>, Error>;
    async fn update_chat(
        &self,
        chat_id: &str,
        last_message: Option<Message>,
        typing_member_ids: Vec<String>,
    ) -> Result<Chat, Error>;
    async fn delete_chat(&self, chat_id: &str) -> Result<String, Error>;
}

pub struct ChatRepositoryImpl {
    collection: Collection<Chat>,
}

impl ChatRepositoryImpl {
    pub fn new(collection: Collection<Chat>) -> Box<dyn ChatRepository + Send + Sync> {
        Box::new(ChatRepositoryImpl { collection })
    }
}

#[async_trait::async_trait]
impl ChatRepository for ChatRepositoryImpl {
    async fn get_chats(&self, member_id: &str, skip: u64, limit: u64) -> Result<Vec<Chat>, Error> {
        let filter = doc! {
            "member_ids": {
                "$elemMatch": {
                    "member_id": member_id
                }
            }
        };
        let mut chats: Vec<Chat> = vec![];
        let options = FindOptions::builder().skip(skip).limit(limit.try_into().ok()).build();
        let mut cursor = self.collection.find(filter, options).await?;
        while let Some(chat) = cursor.try_next().await? {
            chats.push(chat)
        }
        Ok(chats)
    }

    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Option<Chat>, Error> {
        if let Ok(chat) = self.collection.find_one(doc! { "_id": chat_id }, None).await {
            return Ok(chat);
        }
        Err(make_error!("unable to get chat by id"))
    }

    async fn create_chat(&self, member_ids: Vec<String>) -> Result<Option<Chat>, Error> {
        let id = ObjectId::new().to_hex();
        let chat = Chat::new(&id, member_ids.clone());
        let filter = doc! {
            "member_ids": {
                "$all": member_ids
            }
        };
        if let None = self.collection.find_one(filter, None).await? {
            let result = self.collection.insert_one(chat, None).await?;
            if let Some(id) = result.inserted_id.as_str() {
                return self.get_chat_by_id(id).await;
            }
            return Err(make_error!("unable to create chat"));
        }
        Ok(None)
    }

    async fn update_chat(&self, chat_id: &str, last_message: Option<Message>, typing_member_ids: Vec<String>) -> Result<Chat, Error> {
        let timestamp = bson!(Chat::timestamp_now() as i64);
        let mut document = doc! {
            "typing_member_ids": typing_member_ids,
            "updated_at": timestamp
        };
        if let Some(last_message) = last_message {
            document.insert("last_message", bson::to_bson(&last_message)?);
        }
        let result = self.collection.update_one(doc! {"_id": chat_id}, doc! { "$set": document }, None).await?;
        if result.modified_count > 0 {
            if let Some(chat) = self.get_chat_by_id(chat_id).await? {
                return Ok(chat);
            }
        }
        Err(make_error!("unable to update chat"))
    }

    async fn delete_chat(&self, chat_id: &str) -> Result<String, Error> {
        if self.collection.delete_one(doc! { "_id": chat_id }, None).await?.deleted_count > 0 {
            return Ok(String::from(chat_id));
        }
        Err(make_error!("unable to delete chat"))
    }
}