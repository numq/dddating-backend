use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::{Channel, Server};

use configuration::Config;

mod authentication;
mod conversation;

const SERVICE_NAME: &str = "gateway";
const AUTHENTICATION_SERVICE_NAME: &str = "authentication";
const CONVERSATION_SERVICE_NAME: &str = "conversation";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new("service/.env", SERVICE_NAME)?;

    let create_channel_url: fn(&str, &str) -> &'static str = |hostname, port| Box::leak(format!("https://{}:{}", hostname, port).into_boxed_str());

    let authentication_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(AUTHENTICATION_SERVICE_NAME).unwrap());
    let authentication_channel = Channel::from_static(authentication_channel_url).connect().await?;
    let authentication_client = authentication::pb::authentication_service_client::AuthenticationServiceClient::new(authentication_channel);
    let authentication_service = authentication::service::AuthenticationServiceImpl::new(authentication_client);

    let conversation_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(CONVERSATION_SERVICE_NAME).unwrap());
    let conversation_channel = Channel::from_static(conversation_channel_url).connect().await?;
    let conversation_client = conversation::pb::conversation_service_client::ConversationServiceClient::new(conversation_channel);
    let conversation_service = conversation::service::ConversationServiceImpl::new(conversation_client);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(authentication::pb::authentication_service_server::AuthenticationServiceServer::new(authentication_service))
        .add_service(conversation::pb::conversation_service_server::ConversationServiceServer::new(conversation_service))
        .serve(server_addr)
        .await?;
    Ok(())
}
