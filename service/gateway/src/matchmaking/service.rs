use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::matchmaking::pb::{BacktrackRequest, BacktrackResponse, CheckIdsRequest, CheckIdsResponse, DislikeRequest, DislikeResponse, GetLikesRequest, GetLikesResponse, LikeRequest, LikeResponse};
use crate::matchmaking::pb::matchmaking_service_client::MatchmakingServiceClient;
use crate::matchmaking::pb::matchmaking_service_server::MatchmakingService;

pub struct MatchmakingServiceImpl {
    client: MatchmakingServiceClient<Channel>,
}

impl MatchmakingServiceImpl {
    pub fn new(client: MatchmakingServiceClient<Channel>) -> impl MatchmakingService {
        MatchmakingServiceImpl { client }
    }
}

#[tonic::async_trait]
impl MatchmakingService for MatchmakingServiceImpl {
    async fn check_ids(&self, request: Request<CheckIdsRequest>) -> Result<Response<CheckIdsResponse>, Status> {
        self.client.clone().check_ids(request).await
    }

    async fn get_likes(&self, request: Request<GetLikesRequest>) -> Result<Response<GetLikesResponse>, Status> {
        self.client.clone().get_likes(request).await
    }

    async fn like(&self, request: Request<LikeRequest>) -> Result<Response<LikeResponse>, Status> {
        self.client.clone().like(request).await
    }

    async fn dislike(&self, request: Request<DislikeRequest>) -> Result<Response<DislikeResponse>, Status> {
        self.client.clone().dislike(request).await
    }

    async fn backtrack(&self, request: Request<BacktrackRequest>) -> Result<Response<BacktrackResponse>, Status> {
        self.client.clone().backtrack(request).await
    }
}