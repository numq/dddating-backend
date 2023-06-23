use tonic::{Request, Response, Status};

use crate::matchmaking::interactor::MatchmakingInteractor;
use crate::matchmaking::pb::{BacktrackRequest, BacktrackResponse, CheckIdsRequest, CheckIdsResponse, DislikeRequest, DislikeResponse, GetLikesRequest, GetLikesResponse, LikeRequest, LikeResponse};
use crate::matchmaking::pb::matchmaking_service_server::MatchmakingService;

pub struct MatchmakingServiceImpl {
    interactor: Box<dyn MatchmakingInteractor + Send + Sync>,
}

impl MatchmakingServiceImpl {
    pub fn new(interactor: Box<dyn MatchmakingInteractor + Send + Sync>) -> impl MatchmakingService {
        MatchmakingServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl MatchmakingService for MatchmakingServiceImpl {
    async fn check_ids(&self, request: Request<CheckIdsRequest>) -> Result<Response<CheckIdsResponse>, Status> {
        let CheckIdsRequest { from_id, to_ids } = request.into_inner();
        if from_id.is_empty() || to_ids.is_empty() {
            return status::Status::invalid_arguments(vec!["from_id", "to_ids"]);
        }

        match self.interactor.check_ids(&from_id, to_ids).await {
            Ok(values) => Ok(Response::new(CheckIdsResponse { values })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_likes(&self, request: Request<GetLikesRequest>) -> Result<Response<GetLikesResponse>, Status> {
        let GetLikesRequest { id, skip, limit } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.get_likes(&id, skip, limit).await {
            Ok(identifiers) => Ok(Response::new(GetLikesResponse { identifiers })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn like(&self, request: Request<LikeRequest>) -> Result<Response<LikeResponse>, Status> {
        let LikeRequest { from_id, to_id } = request.into_inner();
        if from_id.is_empty() || to_id.is_empty() {
            return status::Status::invalid_arguments(vec!["from_id", "to_id"]);
        }

        match self.interactor.like(&from_id, &to_id).await {
            Ok(has_match) => Ok(Response::new(LikeResponse { has_match })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn dislike(&self, request: Request<DislikeRequest>) -> Result<Response<DislikeResponse>, Status> {
        let DislikeRequest { from_id, to_id } = request.into_inner();
        if from_id.is_empty() || to_id.is_empty() {
            return status::Status::invalid_arguments(vec!["from_id", "to_id"]);
        }

        match self.interactor.dislike(&from_id, &to_id).await {
            Ok(_) => Ok(Response::new(DislikeResponse {})),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn backtrack(&self, request: Request<BacktrackRequest>) -> Result<Response<BacktrackResponse>, Status> {
        let id = &request.into_inner().id.clone();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.backtrack(id).await {
            Ok(id) => Ok(Response::new(BacktrackResponse { id })),
            Err(error) => status::Status::internal(error)
        }
    }
}