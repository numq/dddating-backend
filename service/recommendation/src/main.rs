use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use tonic::transport::{Channel, Server};

use configuration::Config;

use crate::matchmaking::pb::matchmaking_service_client::MatchmakingServiceClient;
use crate::profile::pb::profile_service_client::ProfileServiceClient;

mod profile;
mod matchmaking;
mod recommendation;

const SERVICE_NAME: &str = "recommendation";
const MATCHMAKING_SERVICE_NAME: &str = "matchmaking";
const PROFILE_SERVICE_NAME: &str = "profile";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::default(SERVICE_NAME)?;

    let redis_client = redis::Client::open(format!("redis://{}:{}", cfg.redis_hostname.clone().unwrap(), cfg.redis_port.clone().unwrap()))?;

    let create_channel_url: fn(&str, &str) -> &'static str = |hostname, port| Box::leak(format!("https://{}:{}", hostname, port).into_boxed_str());

    let matchmaking_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(MATCHMAKING_SERVICE_NAME).unwrap());
    let matchmaking_channel = Channel::from_static(matchmaking_channel_url).connect().await?;
    let matchmaking_client = MatchmakingServiceClient::new(matchmaking_channel);
    let matchmaking_api = matchmaking::api::MatchmakingApiImpl::new(matchmaking_client);

    let profile_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(PROFILE_SERVICE_NAME).unwrap());
    let profile_channel = Channel::from_static(profile_channel_url).connect().await?;
    let profile_client = ProfileServiceClient::new(profile_channel);
    let profile_api = profile::api::ProfileApiImpl::new(profile_client);

    let repository = recommendation::repository::RecommendationRepositoryImpl::new(redis_client, matchmaking_api, profile_api).await;
    let interactor = Arc::new(recommendation::interactor::RecommendationInteractorImpl::new(repository));
    let service = recommendation::service::RecommendationServiceImpl::new(Arc::clone(&interactor));

    let message_queue = amqp::MessageQueue::connect(&cfg.amqp_hostname.unwrap(), &cfg.amqp_port.unwrap()).await?;
    let mut message_queue_handler = recommendation::amqp::MessageQueueHandler::new(Arc::clone(&interactor), message_queue);
    message_queue_handler.consume_new_chats().await;

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());

    println!("Service '{}' started at address: {}", SERVICE_NAME, server_addr);

    Server::builder()
        .add_service(recommendation::pb::recommendation_service_server::RecommendationServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}
