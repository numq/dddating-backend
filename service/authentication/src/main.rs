use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::{Channel, Server};

use configuration::Config;

use crate::account::pb::account_service_client::AccountServiceClient;
use crate::token::pb::token_service_client::TokenServiceClient;

mod account;
mod token;
mod authentication;

const ACCOUNT_SERVICE_NAME: &str = "account";
const TOKEN_SERVICE_NAME: &str = "token";
const SERVICE_NAME: &str = "authentication";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new_default(SERVICE_NAME)?;
    let create_channel_url: fn(&str, &str) -> &'static str = |hostname, port| Box::leak(format!("https://{}:{}", hostname, port).into_boxed_str());

    let account_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(ACCOUNT_SERVICE_NAME).unwrap());
    let account_channel = Channel::from_static(account_channel_url).connect().await?;
    let account_client = AccountServiceClient::new(account_channel);
    let account_service = account::api::AccountApi::new(account_client);
    let account_repository = account::repository::AccountRepositoryImpl::new(account_service);

    let token_channel_url = create_channel_url(&cfg.default_hostname.clone().unwrap(), &cfg.find_port(TOKEN_SERVICE_NAME).unwrap());
    let token_channel = Channel::from_static(&token_channel_url).connect().await?;
    let token_client = TokenServiceClient::new(token_channel);
    let token_service = token::api::TokenApi::new(token_client);
    let token_repository = token::repository::TokenRepositoryImpl::new(token_service);

    let interactor = authentication::interactor::AuthenticationInteractorImpl::new(account_repository, token_repository);
    let service = authentication::service::AuthenticationServiceImpl::new(interactor);
    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(authentication::pb::authentication_service_server::AuthenticationServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}