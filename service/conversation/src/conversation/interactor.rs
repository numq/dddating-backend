use crate::chat::entity::Chat;
use crate::chat::repository::ChatRepository;
use crate::message::entity::Message;
use crate::message::repository::MessageRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait ConversationInteractor {
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
    async fn get_messages(
        &self,
        chat_id: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Message>, Error>;
    async fn get_message_by_id(&self, message_id: &str) -> Result<Option<Message>, Error>;
    async fn send_message(
        &self,
        chat_id: &str,
        member_id: &str,
        text: Option<String>,
        images: Vec<Vec<u8>>,
    ) -> Result<Message, Error>;
    async fn delete_message(&self, message_id: &str) -> Result<String, Error>;
}

pub struct ConversationInteractorImpl {
    chat_repository: Box<dyn ChatRepository + Send + Sync>,
    message_repository: Box<dyn MessageRepository + Send + Sync>,
}

impl ConversationInteractorImpl {
    pub fn new(
        chat_repository: Box<dyn ChatRepository + Send + Sync>,
        message_repository: Box<dyn MessageRepository + Send + Sync>,
    ) -> Box<dyn ConversationInteractor + Send + Sync> {
        Box::new(ConversationInteractorImpl { chat_repository, message_repository })
    }
}

#[async_trait::async_trait]
impl ConversationInteractor for ConversationInteractorImpl {
    async fn get_chats(&self, member_id: &str, skip: u64, limit: u64) -> Result<Vec<Chat>, Error> {
        self.chat_repository.get_chats(member_id, skip, limit).await
    }

    async fn get_chat_by_id(&self, chat_id: &str) -> Result<Option<Chat>, Error> {
        self.chat_repository.get_chat_by_id(chat_id).await
    }

    async fn create_chat(&self, member_ids: Vec<String>) -> Result<Option<Chat>, Error> {
        self.chat_repository.create_chat(member_ids).await
    }

    async fn update_chat(&self, chat_id: &str, last_message: Option<Message>, typing_member_ids: Vec<String>) -> Result<Chat, Error> {
        self.chat_repository.update_chat(chat_id, last_message, typing_member_ids).await
    }

    async fn delete_chat(&self, chat_id: &str) -> Result<String, Error> {
        self.chat_repository.delete_chat(chat_id).await
    }

    async fn get_messages(&self, chat_id: &str, skip: u64, limit: u64) -> Result<Vec<Message>, Error> {
        self.message_repository.get_messages(chat_id, skip, limit).await
    }

    async fn get_message_by_id(&self, message_id: &str) -> Result<Option<Message>, Error> {
        self.message_repository.get_message_by_id(message_id).await
    }

    async fn send_message(&self, chat_id: &str, member_id: &str, text: Option<String>, images: Vec<Vec<u8>>) -> Result<Message, Error> {
        self.message_repository.create_message(chat_id, member_id, text, images).await
    }

    async fn delete_message(&self, message_id: &str) -> Result<String, Error> {
        self.message_repository.delete_message(message_id).await
    }
}