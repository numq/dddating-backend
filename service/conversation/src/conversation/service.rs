use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::conversation::interactor::ConversationInteractor;
use crate::conversation::pb::{DeleteChatRequest, DeleteChatResponse, DeleteMessageRequest, DeleteMessageResponse, GetChatByIdRequest, GetChatByIdResponse, GetChatsRequest, GetChatsResponse, GetMessageByIdRequest, GetMessageByIdResponse, GetMessagesRequest, GetMessagesResponse, SendMessageRequest, SendMessageResponse, UpdateChatRequest, UpdateChatResponse};
use crate::conversation::pb::conversation_service_server::ConversationService;

pub struct ConversationServiceImpl {
    interactor: Arc<Box<dyn ConversationInteractor + Send + Sync>>,
}

impl ConversationServiceImpl {
    pub fn new(interactor: Arc<Box<dyn ConversationInteractor + Send + Sync>>) -> impl ConversationService {
        ConversationServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl ConversationService for ConversationServiceImpl {
    async fn get_chats(&self, request: Request<GetChatsRequest>) -> Result<Response<GetChatsResponse>, Status> {
        let GetChatsRequest { member_id, skip, limit } = request.into_inner();
        if member_id.is_empty() {
            return status::Status::invalid_arguments(vec!["member_id"]);
        }

        match self.interactor.get_chats(&member_id, skip, limit).await {
            Ok(chats) => Ok(
                Response::new(
                    GetChatsResponse {
                        chats: chats.into_iter().map(|chat| chat.into()).collect()
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_chat_by_id(&self, request: Request<GetChatByIdRequest>) -> Result<Response<GetChatByIdResponse>, Status> {
        let GetChatByIdRequest { chat_id } = request.into_inner();
        if chat_id.is_empty() {
            return status::Status::invalid_arguments(vec!["chat_id"]);
        }

        match self.interactor.get_chat_by_id(&chat_id).await {
            Ok(chat) => Ok(
                Response::new(
                    GetChatByIdResponse {
                        chat: chat.map(|chat| chat.into())
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn update_chat(&self, request: Request<UpdateChatRequest>) -> Result<Response<UpdateChatResponse>, Status> {
        let UpdateChatRequest { chat_id, typing_member_ids, .. } = request.into_inner();
        if chat_id.is_empty() {
            return status::Status::invalid_arguments(vec!["chat_id"]);
        }

        match self.interactor.update_chat(&chat_id, None, typing_member_ids).await {
            Ok(chat) => Ok(
                Response::new(
                    UpdateChatResponse {
                        chat: Some(chat.into())
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn delete_chat(&self, request: Request<DeleteChatRequest>) -> Result<Response<DeleteChatResponse>, Status> {
        let DeleteChatRequest { chat_id } = request.into_inner();
        if chat_id.is_empty() {
            return status::Status::invalid_arguments(vec!["chat_id"]);
        }

        match self.interactor.delete_chat(&chat_id).await {
            Ok(chat_id) => Ok(
                Response::new(
                    DeleteChatResponse { chat_id }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_messages(&self, request: Request<GetMessagesRequest>) -> Result<Response<GetMessagesResponse>, Status> {
        let GetMessagesRequest { chat_id, skip, limit } = request.into_inner();
        if chat_id.is_empty() {
            return status::Status::invalid_arguments(vec!["chat_id"]);
        }

        match self.interactor.get_messages(&chat_id, skip, limit).await {
            Ok(messages) => Ok(
                Response::new(
                    GetMessagesResponse {
                        messages: messages.into_iter().map(|message| message.into()).collect()
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_message_by_id(&self, request: Request<GetMessageByIdRequest>) -> Result<Response<GetMessageByIdResponse>, Status> {
        let GetMessageByIdRequest { message_id } = request.into_inner();
        if message_id.is_empty() {
            return status::Status::invalid_arguments(vec!["message_id"]);
        }

        match self.interactor.get_message_by_id(&message_id).await {
            Ok(message) => Ok(
                Response::new(
                    GetMessageByIdResponse {
                        message: message.map(|message| message.into())
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<SendMessageResponse>, Status> {
        let SendMessageRequest { chat_id, member_id, text, images } = request.into_inner();

        let text = if text.is_empty() { None } else { Some(text) };

        if chat_id.is_empty() || member_id.is_empty() || (text.is_none() && images.is_empty()) {
            return status::Status::invalid_arguments(vec!["chat_id", "member_id", "text", "images"]);
        }

        match self.interactor.send_message(&chat_id, &member_id, text, images).await {
            Ok(message) => Ok(
                Response::new(
                    SendMessageResponse {
                        message: Some(message.into())
                    }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn delete_message(&self, request: Request<DeleteMessageRequest>) -> Result<Response<DeleteMessageResponse>, Status> {
        let DeleteMessageRequest { message_id } = request.into_inner();
        if message_id.is_empty() {
            return status::Status::invalid_arguments(vec!["message_id"]);
        }

        match self.interactor.delete_message(&message_id).await {
            Ok(message_id) => Ok(
                Response::new(
                    DeleteMessageResponse { message_id }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }
}