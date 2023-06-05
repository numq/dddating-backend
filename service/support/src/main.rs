use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;

use configuration::Config;

use crate::support::entity::Ticket;

pub mod support;

const SERVICE_NAME: &str = "support";
const TICKETS_COLLECTION: &str = "tickets";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new_default(SERVICE_NAME)?;
    let mongodb = mongodb::Client::with_uri_str(format!("mongodb://{}:{}", cfg.mongo_hostname.unwrap(), cfg.mongo_port.unwrap())).await?;
    let database = mongodb.database(SERVICE_NAME);
    let collection = database.collection::<Ticket>(TICKETS_COLLECTION);
    let repository = support::repository::SupportRepositoryImpl::new(collection);
    let interactor = support::interactor::SupportInteractorImpl::new(repository);
    let service = support::service::SupportServiceImpl::new(interactor);
    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(support::pb::support_service_server::SupportServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}
