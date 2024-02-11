use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;

use tonic::transport::{Channel, Server};

mod interceptor;

mod authentication;
mod conversation;
mod matchmaking;
mod profile;
mod recommendation;
mod safety;
mod support;

const SERVICE_NAME: &str = "gateway";
const AUTHENTICATION_SERVICE_NAME: &str = "authentication";
const CONVERSATION_SERVICE_NAME: &str = "conversation";
const MATCHMAKING_SERVICE_NAME: &str = "matchmaking";
const PROFILE_SERVICE_NAME: &str = "profile";
const RECOMMENDATION_SERVICE_NAME: &str = "recommendation";
const SAFETY_SERVICE_NAME: &str = "safety";
const SUPPORT_SERVICE_NAME: &str = "support";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::default(SERVICE_NAME)?;
    let authentication_cfg = configuration::Config::default(AUTHENTICATION_SERVICE_NAME)?;
    let conversation_cfg = configuration::Config::default(CONVERSATION_SERVICE_NAME)?;
    let matchmaking_cfg = configuration::Config::default(MATCHMAKING_SERVICE_NAME)?;
    let profile_cfg = configuration::Config::default(PROFILE_SERVICE_NAME)?;
    let recommendation_cfg = configuration::Config::default(RECOMMENDATION_SERVICE_NAME)?;
    let safety_cfg = configuration::Config::default(SAFETY_SERVICE_NAME)?;
    let support_cfg = configuration::Config::default(SUPPORT_SERVICE_NAME)?;

    let create_channel_url: fn(&str, &str) -> &'static str = |hostname, port| Box::leak(format!("https://{}:{}", hostname, port).into_boxed_str());

    let authentication_channel_url = create_channel_url(&authentication_cfg.service_hostname.unwrap(), &authentication_cfg.service_port.unwrap());
    let authentication_channel = Channel::from_static(authentication_channel_url).connect_lazy();
    let authentication_client = authentication::pb::authentication_service_client::AuthenticationServiceClient::new(authentication_channel);
    let authentication_service = authentication::service::AuthenticationServiceImpl::new(authentication_client.clone());

    let authentication_api = authentication::api::AuthenticationApiImpl::new(authentication_client.clone());
    let authentication_repository = authentication::repository::AuthenticationRepositoryImpl::new(authentication_api);
    let authentication_interactor = Arc::new(authentication::interactor::AuthenticationInteractorImpl::new(authentication_repository));

    let conversation_channel_url = create_channel_url(&conversation_cfg.service_hostname.unwrap(), &conversation_cfg.service_port.unwrap());
    let conversation_channel = Channel::from_static(conversation_channel_url).connect_lazy();
    let conversation_client = conversation::pb::conversation_service_client::ConversationServiceClient::new(conversation_channel);
    let conversation_service = conversation::service::ConversationServiceImpl::new(conversation_client);

    let matchmaking_channel_url = create_channel_url(&matchmaking_cfg.service_hostname.unwrap(), &matchmaking_cfg.service_port.unwrap());
    let matchmaking_channel = Channel::from_static(matchmaking_channel_url).connect_lazy();
    let matchmaking_client = matchmaking::pb::matchmaking_service_client::MatchmakingServiceClient::new(matchmaking_channel);
    let matchmaking_service = matchmaking::service::MatchmakingServiceImpl::new(matchmaking_client);

    let profile_channel_url = create_channel_url(&profile_cfg.service_hostname.unwrap(), &profile_cfg.service_port.unwrap());
    let profile_channel = Channel::from_static(profile_channel_url).connect_lazy();
    let profile_client = profile::pb::profile_service_client::ProfileServiceClient::new(profile_channel);
    let profile_service = profile::service::ProfileServiceImpl::new(profile_client);

    let recommendation_channel_url = create_channel_url(&recommendation_cfg.service_hostname.unwrap(), &recommendation_cfg.service_port.unwrap());
    let recommendation_channel = Channel::from_static(recommendation_channel_url).connect_lazy();
    let recommendation_client = recommendation::pb::recommendation_service_client::RecommendationServiceClient::new(recommendation_channel);
    let recommendation_service = recommendation::service::RecommendationServiceImpl::new(recommendation_client);

    let safety_channel_url = create_channel_url(&safety_cfg.service_hostname.unwrap(), &safety_cfg.service_port.unwrap());
    let safety_channel = Channel::from_static(safety_channel_url).connect_lazy();
    let safety_client = safety::pb::safety_service_client::SafetyServiceClient::new(safety_channel);
    let safety_service = safety::service::SafetyServiceImpl::new(safety_client);

    let support_channel_url = create_channel_url(&support_cfg.service_hostname.unwrap(), &support_cfg.service_port.unwrap());
    let support_channel = Channel::from_static(support_channel_url).connect_lazy();
    let support_client = support::pb::support_service_client::SupportServiceClient::new(support_channel);
    let support_service = support::service::SupportServiceImpl::new(support_client);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());

    println!("Service '{}' started at address: {}", SERVICE_NAME, server_addr);

    Server::builder()
        .add_service(authentication::pb::authentication_service_server::AuthenticationServiceServer::new(authentication_service))
        .add_service(interceptor::authorization::with_auth_interceptor(conversation::pb::conversation_service_server::ConversationServiceServer::new(conversation_service), Arc::clone(&authentication_interactor)))
        .add_service(interceptor::authorization::with_auth_interceptor(matchmaking::pb::matchmaking_service_server::MatchmakingServiceServer::new(matchmaking_service), Arc::clone(&authentication_interactor)))
        .add_service(interceptor::authorization::with_auth_interceptor(profile::pb::profile_service_server::ProfileServiceServer::new(profile_service), Arc::clone(&authentication_interactor)))
        .add_service(interceptor::authorization::with_auth_interceptor(recommendation::pb::recommendation_service_server::RecommendationServiceServer::new(recommendation_service), Arc::clone(&authentication_interactor)))
        .add_service(interceptor::authorization::with_auth_interceptor(safety::pb::safety_service_server::SafetyServiceServer::new(safety_service), Arc::clone(&authentication_interactor)))
        .add_service(interceptor::authorization::with_auth_interceptor(support::pb::support_service_server::SupportServiceServer::new(support_service), Arc::clone(&authentication_interactor)))
        .serve(server_addr)
        .await?;

    Ok(())
}
