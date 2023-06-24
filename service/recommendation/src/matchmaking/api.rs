use tonic::Request;
use tonic::transport::Channel;

use error::make_error;

use crate::matchmaking::pb::{CheckIdsRequest, CheckIdsResponse};
use crate::matchmaking::pb::matchmaking_service_client::MatchmakingServiceClient;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tonic::async_trait]
pub trait MatchmakingApi {
    async fn check_ids(&self, from_id: &str, to_ids: Vec<String>) -> Result<Vec<bool>, Error>;
}

pub struct MatchmakingApiImpl {
    client: MatchmakingServiceClient<Channel>,
}

impl MatchmakingApiImpl {
    pub fn new(client: MatchmakingServiceClient<Channel>) -> Box<dyn MatchmakingApi + Send + Sync> {
        Box::new(MatchmakingApiImpl { client })
    }
}

#[tonic::async_trait]
impl MatchmakingApi for MatchmakingApiImpl {
    async fn check_ids(&self, from_id: &str, to_ids: Vec<String>) -> Result<Vec<bool>, Error> {
        let request = Request::new(
            CheckIdsRequest {
                from_id: String::from(from_id),
                to_ids,
            }
        );
        if let Ok(response) = self.client.clone().check_ids(request).await {
            let CheckIdsResponse { values } = response.into_inner();
            return Ok(values);
        }
        Err(make_error!("unable to check identifiers"))
    }
}