use tonic::{Request, Response, Status};
use tonic::transport::Channel;

use crate::profile::pb::{CreateProfileRequest, CreateProfileResponse, DeleteProfileRequest, DeleteProfileResponse, GetProfileByIdRequest, GetProfileByIdResponse, GetProfilesRequest, GetProfilesResponse, GetRandomProfilesRequest, GetRandomProfilesResponse, UpdateProfileRequest, UpdateProfileResponse};
use crate::profile::pb::profile_service_client::ProfileServiceClient;
use crate::profile::pb::profile_service_server::ProfileService;

pub struct ProfileServiceImpl {
    client: ProfileServiceClient<Channel>,
}

impl ProfileServiceImpl {
    pub fn new(client: ProfileServiceClient<Channel>) -> impl ProfileService {
        ProfileServiceImpl { client }
    }
}

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    async fn get_random_profiles(&self, request: Request<GetRandomProfilesRequest>) -> Result<Response<GetRandomProfilesResponse>, Status> {
        self.client.clone().get_random_profiles(request).await
    }

    async fn get_profiles(&self, request: Request<GetProfilesRequest>) -> Result<Response<GetProfilesResponse>, Status> {
        self.client.clone().get_profiles(request).await
    }

    async fn get_profile_by_id(&self, request: Request<GetProfileByIdRequest>) -> Result<Response<GetProfileByIdResponse>, Status> {
        self.client.clone().get_profile_by_id(request).await
    }

    async fn create_profile(&self, request: Request<CreateProfileRequest>) -> Result<Response<CreateProfileResponse>, Status> {
        self.client.clone().create_profile(request).await
    }

    async fn update_profile(&self, request: Request<UpdateProfileRequest>) -> Result<Response<UpdateProfileResponse>, Status> {
        self.client.clone().update_profile(request).await
    }

    async fn delete_profile(&self, request: Request<DeleteProfileRequest>) -> Result<Response<DeleteProfileResponse>, Status> {
        self.client.clone().delete_profile(request).await
    }
}