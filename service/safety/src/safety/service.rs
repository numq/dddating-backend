use tonic::{Request, Response, Status};

use crate::safety::interactor::SafetyInteractor;
use crate::safety::pb::{BlockUserRequest, BlockUserResponse, CheckUserIdRequest, CheckUserIdResponse, GetBlockedUsersRequest, GetBlockedUsersResponse, UnblockUserRequest, UnblockUserResponse};
use crate::safety::pb::safety_service_server::SafetyService;

pub struct SafetyServiceImpl {
    interactor: Box<dyn SafetyInteractor + Send + Sync>,
}

impl SafetyServiceImpl {
    pub fn new(interactor: Box<dyn SafetyInteractor + Send + Sync>) -> impl SafetyService {
        SafetyServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl SafetyService for SafetyServiceImpl {
    async fn check_user_id(&self, request: Request<CheckUserIdRequest>) -> Result<Response<CheckUserIdResponse>, Status> {
        let CheckUserIdRequest { from_id, to_id } = request.into_inner();
        if from_id.is_empty() || to_id.is_empty() {
            return status::Status::invalid_arguments(vec!["from_id", "to_id"]);
        }

        match self.interactor.check_user_id(&from_id, &to_id).await {
            Ok(is_blocked) => Ok(
                Response::new(
                    CheckUserIdResponse { is_blocked }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_blocked_users(&self, request: Request<GetBlockedUsersRequest>) -> Result<Response<GetBlockedUsersResponse>, Status> {
        let GetBlockedUsersRequest { from_id, skip, limit } = request.into_inner();
        if from_id.is_empty() {
            return status::Status::invalid_arguments(vec!["from_id"]);
        }

        match self.interactor.get_blocked_users(&from_id, skip, limit).await {
            Ok(user_ids) => Ok(
                Response::new(
                    GetBlockedUsersResponse { user_ids: user_ids.into_iter().collect() }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn block_user(&self, request: Request<BlockUserRequest>) -> Result<Response<BlockUserResponse>, Status> {
        let BlockUserRequest { from_id, to_id } = request.into_inner();
        if from_id.is_empty() || to_id.is_empty() {
            return status::Status::invalid_arguments(vec!["from_id", "to_id"]);
        }

        match self.interactor.block_user(&from_id, &to_id).await {
            Ok(_) => Ok(
                Response::new(
                    BlockUserResponse::default()
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn unblock_user(&self, request: Request<UnblockUserRequest>) -> Result<Response<UnblockUserResponse>, Status> {
        let UnblockUserRequest { from_id, to_id } = request.into_inner();
        if from_id.is_empty() || to_id.is_empty() {
            return status::Status::invalid_arguments(vec!["from_id", "to_id"]);
        }

        match self.interactor.unblock_user(&from_id, &to_id).await {
            Ok(_) => Ok(
                Response::new(
                    UnblockUserResponse::default()
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }
}