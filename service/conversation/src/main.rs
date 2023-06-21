use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use tonic::transport::Server;

use crate::chat::entity::Chat;
use crate::message::entity::Message;

mod chat;
mod message;
mod conversation;

const SERVICE_NAME: &str = "conversation";
const CHATS_COLLECTION: &str = "chats";
const MESSAGES_COLLECTION: &str = "messages";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::new("service/.env", SERVICE_NAME)?;

    let mongodb = mongodb::Client::with_uri_str(format!("mongodb://{}:{}", cfg.mongo_hostname.unwrap(), cfg.mongo_port.unwrap())).await?;
    let database = mongodb.database(SERVICE_NAME);

    let chats_collection = database.collection::<Chat>(CHATS_COLLECTION);
    let chat_repository = chat::repository::ChatRepositoryImpl::new(chats_collection);

    let messages_collection = database.collection::<Message>(MESSAGES_COLLECTION);
    let message_repository = message::repository::MessageRepositoryImpl::new(messages_collection);

    let interactor = Arc::new(conversation::interactor::ConversationInteractorImpl::new(chat_repository, message_repository));
    let service = conversation::service::ConversationServiceImpl::new(Arc::clone(&interactor));

    let message_queue = amqp::MessageQueue::connect(&cfg.amqp_hostname.unwrap(), &cfg.amqp_port.unwrap()).await?;
    let mut message_queue_handler = conversation::amqp::MessageQueueHandler::new(Arc::clone(&interactor), message_queue);
    message_queue_handler.consume_new_chats().await;

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(conversation::pb::conversation_service_server::ConversationServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}