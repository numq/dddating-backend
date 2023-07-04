use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::conversation::pb::{DeleteChatRequest, DeleteChatResponse, DeleteMessageRequest, DeleteMessageResponse, GetChatByIdRequest, GetChatByIdResponse, GetChatsRequest, GetChatsResponse, GetMessageByIdRequest, GetMessageByIdResponse, GetMessagesRequest, GetMessagesResponse, SendMessageRequest, SendMessageResponse, UpdateChatRequest, UpdateChatResponse};
use crate::conversation::pb::conversation_service_client::ConversationServiceClient;
use crate::conversation::pb::conversation_service_server::ConversationService;

pub struct ConversationServiceImpl {
    client: ConversationServiceClient<Channel>,
}

impl ConversationServiceImpl {
    pub fn new(client: ConversationServiceClient<Channel>) -> impl ConversationService {
        ConversationServiceImpl { client }
    }
}

#[tonic::async_trait]
impl ConversationService for ConversationServiceImpl {
    async fn get_chats(&self, request: Request<GetChatsRequest>) -> Result<Response<GetChatsResponse>, Status> {
        self.client.clone().get_chats(request).await
    }

    async fn get_chat_by_id(&self, request: Request<GetChatByIdRequest>) -> Result<Response<GetChatByIdResponse>, Status> {
        self.client.clone().get_chat_by_id(request).await
    }

    async fn update_chat(&self, request: Request<UpdateChatRequest>) -> Result<Response<UpdateChatResponse>, Status> {
        self.client.clone().update_chat(request).await
    }

    async fn delete_chat(&self, request: Request<DeleteChatRequest>) -> Result<Response<DeleteChatResponse>, Status> {
        self.client.clone().delete_chat(request).await
    }

    async fn get_messages(&self, request: Request<GetMessagesRequest>) -> Result<Response<GetMessagesResponse>, Status> {
        self.client.clone().get_messages(request).await
    }

    async fn get_message_by_id(&self, request: Request<GetMessageByIdRequest>) -> Result<Response<GetMessageByIdResponse>, Status> {
        self.client.clone().get_message_by_id(request).await
    }

    async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<SendMessageResponse>, Status> {
        self.client.clone().send_message(request).await
    }

    async fn delete_message(&self, request: Request<DeleteMessageRequest>) -> Result<Response<DeleteMessageResponse>, Status> {
        self.client.clone().delete_message(request).await
    }
}