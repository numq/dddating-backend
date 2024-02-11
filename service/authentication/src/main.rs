use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::{Channel, Server};

use crate::account::pb::account_service_client::AccountServiceClient;
use crate::token::pb::token_service_client::TokenServiceClient;

mod account;
mod token;
mod authentication;

const SERVICE_NAME: &str = "authentication";
const ACCOUNT_SERVICE_NAME: &str = "account";
const TOKEN_SERVICE_NAME: &str = "token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::default(SERVICE_NAME)?;
    let account_cfg = configuration::Config::default(ACCOUNT_SERVICE_NAME)?;
    let token_cfg = configuration::Config::default(TOKEN_SERVICE_NAME)?;

    let create_channel_url: fn(&str, &str) -> &'static str = |hostname, port| Box::leak(format!("https://{}:{}", hostname, port).into_boxed_str());

    let account_channel_url = create_channel_url(&account_cfg.service_hostname.unwrap(), &account_cfg.service_port.unwrap());
    let account_channel = Channel::from_static(account_channel_url).connect_lazy();
    let account_client = AccountServiceClient::new(account_channel);
    let account_api = account::api::AccountApiImpl::new(account_client);
    let account_repository = account::repository::AccountRepositoryImpl::new(account_api);

    let token_channel_url = create_channel_url(&token_cfg.service_hostname.unwrap(), &token_cfg.service_port.unwrap());
    let token_channel = Channel::from_static(token_channel_url).connect_lazy();
    let token_client = TokenServiceClient::new(token_channel);
    let token_api = token::api::TokenApiImpl::new(token_client);
    let token_repository = token::repository::TokenRepositoryImpl::new(token_api);

    let interactor = authentication::interactor::AuthenticationInteractorImpl::new(account_repository, token_repository);
    let service = authentication::service::AuthenticationServiceImpl::new(interactor);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());

    println!("Service '{}' started at address: {}", SERVICE_NAME, server_addr);

    Server::builder()
        .add_service(authentication::pb::authentication_service_server::AuthenticationServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}