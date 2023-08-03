use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;

mod token;

const SERVICE_NAME: &str = "token";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::default(SERVICE_NAME)?;

    let redis_client = redis::Client::open(format!("redis://{}:{}", cfg.redis_hostname.unwrap(), cfg.redis_port.unwrap()))?;

    let repository = token::repository::TokenRepositoryImpl::new(cfg.secret_key.unwrap(), redis_client);
    let interactor = token::interactor::TokenInteractorImpl::new(repository);
    let service = token::service::TokenServiceImpl::new(interactor);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());

    println!("Service '{}' started at address: {}", SERVICE_NAME, server_addr);

    Server::builder()
        .add_service(token::pb::token_service_server::TokenServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}