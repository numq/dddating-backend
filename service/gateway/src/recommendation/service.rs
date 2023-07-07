use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::recommendation::pb::{GetCandidatesRequest, GetCandidatesResponse};
use crate::recommendation::pb::recommendation_service_client::RecommendationServiceClient;
use crate::recommendation::pb::recommendation_service_server::RecommendationService;

pub struct RecommendationServiceImpl {
    client: RecommendationServiceClient<Channel>,
}

impl RecommendationServiceImpl {
    pub fn new(client: RecommendationServiceClient<Channel>) -> impl RecommendationService {
        RecommendationServiceImpl { client }
    }
}

#[tonic::async_trait]
impl RecommendationService for RecommendationServiceImpl {
    async fn get_candidates(&self, request: Request<GetCandidatesRequest>) -> Result<Response<GetCandidatesResponse>, Status> {
        self.client.clone().get_candidates(request).await
    }
}