use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;

mod account;
mod password;

const SERVICE_NAME: &str = "account";
const ACCOUNTS_COLLECTION: &str = "accounts";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::new("service/.env", SERVICE_NAME)?;

    let mongodb = mongodb::Client::with_uri_str(format!("mongodb://{}:{}", cfg.mongo_hostname.unwrap(), cfg.mongo_port.unwrap())).await?;
    let database = mongodb.database(SERVICE_NAME);
    let collection = database.collection::<account::entity::Account>(ACCOUNTS_COLLECTION);

    let hasher = password::hasher::DefaultHasher::new();
    let repository = account::repository::AccountRepositoryImpl::new(collection);
    let interactor = account::interactor::AccountInteractorImpl::new(hasher, repository);
    let service = account::service::AccountServiceImpl::new(interactor);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());

    println!("Service '{}' started at address: {}", SERVICE_NAME, server_addr);

    Server::builder()
        .add_service(account::pb::account_service_server::AccountServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}
