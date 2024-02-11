use std::error::Error;
use std::net::SocketAddr;

use tonic::transport::Server;

mod matchmaking;

const SERVICE_NAME: &str = "matchmaking";
const LIKE_COLLECTION: &str = "likes";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = configuration::Config::default(SERVICE_NAME)?;

    let mongodb = mongodb::Client::with_uri_str(format!("mongodb://{}:{}", cfg.mongo_hostname.unwrap(), cfg.mongo_port.unwrap())).await?;
    let database = mongodb.database(SERVICE_NAME);
    let collection = database.collection(LIKE_COLLECTION);

    let redis_client = redis::Client::open(format!("redis://{}:{}", cfg.redis_hostname.unwrap(), cfg.redis_port.unwrap()))?;

    let message_queue = amqp::MessageQueue::connect(&cfg.amqp_hostname.unwrap(), &cfg.amqp_port.unwrap()).await?;

    let repository = matchmaking::repository::MatchmakingRepositoryImpl::new(collection, redis_client, message_queue).await;
    let interactor = matchmaking::interactor::MatchmakingInteractorImpl::new(repository);
    let service = matchmaking::service::MatchmakingServiceImpl::new(interactor);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());

    println!("Service '{}' started at address: {}", SERVICE_NAME, server_addr);

    Server::builder()
        .add_service(matchmaking::pb::matchmaking_service_server::MatchmakingServiceServer::new(service))
        .serve(server_addr)
        .await?;
    Ok(())
}