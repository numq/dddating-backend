use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;

use configuration::Config;

use crate::account::entity::Account;

mod account;
mod password;

const SERVICE_NAME: &str = "account";
const ACCOUNT_COLLECTION: &str = "accounts";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new_default(SERVICE_NAME)?;
    let mongodb = mongodb::Client::with_uri_str(format!("mongodb://{}:{}", cfg.mongo_hostname.unwrap(), cfg.mongo_port.unwrap())).await?;
    let database = mongodb.database(SERVICE_NAME);
    let collection = database.collection::<Account>(ACCOUNT_COLLECTION);
    let repository = account::repository::AccountRepositoryImpl::new(collection);
    let hasher = password::hasher::DefaultHasher::new();
    let interactor = account::interactor::AccountInteractorImpl::new(hasher, repository);
    let service = account::service::AccountServiceImpl::new(interactor);
    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(account::pb::account_service_server::AccountServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}
