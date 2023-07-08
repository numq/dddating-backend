use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::safety::pb::{BlockUserRequest, BlockUserResponse, CheckUserIdRequest, CheckUserIdResponse, GetBlockedUsersRequest, GetBlockedUsersResponse, UnblockUserRequest, UnblockUserResponse};
use crate::safety::pb::safety_service_client::SafetyServiceClient;
use crate::safety::pb::safety_service_server::SafetyService;

pub struct SafetyServiceImpl {
    client: SafetyServiceClient<Channel>,
}

impl SafetyServiceImpl {
    pub fn new(client: SafetyServiceClient<Channel>) -> impl SafetyService {
        SafetyServiceImpl { client }
    }
}

#[tonic::async_trait]
impl SafetyService for SafetyServiceImpl {
    async fn check_user_id(&self, request: Request<CheckUserIdRequest>) -> Result<Response<CheckUserIdResponse>, Status> {
        self.client.clone().check_user_id(request).await
    }

    async fn get_blocked_users(&self, request: Request<GetBlockedUsersRequest>) -> Result<Response<GetBlockedUsersResponse>, Status> {
        self.client.clone().get_blocked_users(request).await
    }

    async fn block_user(&self, request: Request<BlockUserRequest>) -> Result<Response<BlockUserResponse>, Status> {
        self.client.clone().block_user(request).await
    }

    async fn unblock_user(&self, request: Request<UnblockUserRequest>) -> Result<Response<UnblockUserResponse>, Status> {
        self.client.clone().unblock_user(request).await
    }
}