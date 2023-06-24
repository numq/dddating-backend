use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::recommendation::interactor::RecommendationInteractor;
use crate::recommendation::pb::{GetCandidatesRequest, GetCandidatesResponse};
use crate::recommendation::pb::recommendation_service_server::RecommendationService;

pub struct RecommendationServiceImpl {
    interactor: Arc<Box<dyn RecommendationInteractor + Send + Sync>>,
}

impl RecommendationServiceImpl {
    pub fn new(interactor: Arc<Box<dyn RecommendationInteractor + Send + Sync>>) -> impl RecommendationService {
        RecommendationServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl RecommendationService for RecommendationServiceImpl {
    async fn get_candidates(&self, request: Request<GetCandidatesRequest>) -> Result<Response<GetCandidatesResponse>, Status> {
        let GetCandidatesRequest { excepted_id, filter } = request.into_inner();
        if filter.is_none() || excepted_id.is_empty() {
            return status::Status::invalid_arguments(vec!["filter", "excepted_id"]);
        }

        match self.interactor.get_candidates(&excepted_id, filter.unwrap().into()).await {
            Ok(candidate_identifiers) => Ok(Response::new(GetCandidatesResponse { candidate_identifiers })),
            Err(error) => status::Status::internal(error)
        }
    }
}