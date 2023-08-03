use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;

use crate::profile::entity::Profile;

mod profile;

const SERVICE_NAME: &str = "profile";
const PROFILES_COLLECTION: &str = "profiles";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::default(SERVICE_NAME)?;

    let mongodb = mongodb::Client::with_uri_str(format!("mongodb://{}:{}", cfg.mongo_hostname.unwrap(), cfg.mongo_port.unwrap())).await?;
    let database = mongodb.database(SERVICE_NAME);
    let collection = database.collection::<Profile>(PROFILES_COLLECTION);

    let repository = profile::repository::ProfileRepositoryImpl::new(collection);
    let interactor = profile::interactor::ProfileInteractorImpl::new(repository);
    let service = profile::service::ProfileServiceImpl::new(interactor);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());

    println!("Service '{}' started at address: {}", SERVICE_NAME, server_addr);

    Server::builder()
        .add_service(profile::pb::profile_service_server::ProfileServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}
