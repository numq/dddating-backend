use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::{Channel, Server};

use configuration::Config;

mod authentication;
mod conversation;
mod matchmaking;
mod profile;
mod recommendation;

const SERVICE_NAME: &str = "gateway";
const AUTHENTICATION_SERVICE_NAME: &str = "authentication";
const CONVERSATION_SERVICE_NAME: &str = "conversation";
const MATCHMAKING_SERVICE_NAME: &str = "matchmaking";
const PROFILE_SERVICE_NAME: &str = "profile";
const RECOMMENDATION_SERVICE_NAME: &str = "recommendation";

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

    let matchmaking_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(MATCHMAKING_SERVICE_NAME).unwrap());
    let matchmaking_channel = Channel::from_static(matchmaking_channel_url).connect().await?;
    let matchmaking_client = matchmaking::pb::matchmaking_service_client::MatchmakingServiceClient::new(matchmaking_channel);
    let matchmaking_service = matchmaking::service::MatchmakingServiceImpl::new(matchmaking_client);

    let profile_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(PROFILE_SERVICE_NAME).unwrap());
    let profile_channel = Channel::from_static(profile_channel_url).connect().await?;
    let profile_client = profile::pb::profile_service_client::ProfileServiceClient::new(profile_channel);
    let profile_service = profile::service::ProfileServiceImpl::new(profile_client);

    let recommendation_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(RECOMMENDATION_SERVICE_NAME).unwrap());
    let recommendation_channel = Channel::from_static(recommendation_channel_url).connect().await?;
    let recommendation_client = recommendation::pb::recommendation_service_client::RecommendationServiceClient::new(recommendation_channel);
    let recommendation_service = recommendation::service::RecommendationServiceImpl::new(recommendation_client);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(authentication::pb::authentication_service_server::AuthenticationServiceServer::new(authentication_service))
        .add_service(conversation::pb::conversation_service_server::ConversationServiceServer::new(conversation_service))
        .add_service(matchmaking::pb::matchmaking_service_server::MatchmakingServiceServer::new(matchmaking_service))
        .add_service(profile::pb::profile_service_server::ProfileServiceServer::new(profile_service))
        .add_service(recommendation::pb::recommendation_service_server::RecommendationServiceServer::new(recommendation_service))
        .serve(server_addr)
        .await?;
    Ok(())
}
