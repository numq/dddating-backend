use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;

use crate::safety::entity::BlockedUser;

mod safety;

const SERVICE_NAME: &str = "safety";
const BLOCKED_USERS_COLLECTION: &str = "blocked_users";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::new("service/.env", SERVICE_NAME)?;
    let mongodb = mongodb::Client::with_uri_str(format!("mongodb://{}:{}", cfg.mongo_hostname.unwrap(), cfg.mongo_port.unwrap())).await?;
    let database = mongodb.database(SERVICE_NAME);
    let collection = database.collection::<BlockedUser>(BLOCKED_USERS_COLLECTION);
    let repository = safety::repository::SafetyRepositoryImpl::new(collection);
    let interactor = safety::interactor::SafetyInteractorImpl::new(repository);
    let service = safety::service::SafetyServiceImpl::new(interactor);
    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(safety::pb::safety_service_server::SafetyServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}
