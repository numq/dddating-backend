use tonic::{Request, Response, Status};

use crate::profile::interactor::ProfileInteractor;
use crate::profile::pb::{CreateProfileRequest, CreateProfileResponse, DeleteProfileRequest, DeleteProfileResponse, GetProfileByIdRequest, GetProfileByIdResponse, GetProfilesRequest, GetProfilesResponse, GetRandomProfilesRequest, GetRandomProfilesResponse, UpdateProfileRequest, UpdateProfileResponse};
use crate::profile::pb::profile_service_server::ProfileService;

pub struct ProfileServiceImpl {
    interactor: Box<dyn ProfileInteractor + Send + Sync>,
}

impl ProfileServiceImpl {
    pub fn new(interactor: Box<dyn ProfileInteractor + Send + Sync>) -> impl ProfileService {
        ProfileServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl ProfileService for ProfileServiceImpl {
    async fn get_random_profiles(&self, request: Request<GetRandomProfilesRequest>) -> Result<Response<GetRandomProfilesResponse>, Status> {
        let GetRandomProfilesRequest { excepted_id, filter, count } = request.into_inner();
        if filter.is_none() || excepted_id.is_empty() || count < 1 {
            return status::Status::invalid_arguments(vec!["filter", "excepted_id", "count - less than 1"]);
        }

        match self.interactor.get_random_profiles(&excepted_id, filter.unwrap().into(), count).await {
            Ok(profiles) => Ok(Response::new(GetRandomProfilesResponse { profiles: profiles.into_iter().map(|profile| profile.into()).collect() })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_profiles(&self, request: Request<GetProfilesRequest>) -> Result<Response<GetProfilesResponse>, Status> {
        let GetProfilesRequest { excepted_id, filter, skip, limit } = request.into_inner();
        if filter.is_none() || excepted_id.is_empty() || limit < 1 {
            return status::Status::invalid_arguments(vec!["filter", "excepted_id", "limit - less than 1"]);
        }

        match self.interactor.get_profiles(&excepted_id, filter.unwrap().into(), skip, limit).await {
            Ok(profiles) => Ok(Response::new(GetProfilesResponse { profiles: profiles.into_iter().map(|profile| profile.into()).collect() })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_profile_by_id(&self, request: Request<GetProfileByIdRequest>) -> Result<Response<GetProfileByIdResponse>, Status> {
        let GetProfileByIdRequest { id } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.get_profile_by_id(&id).await {
            Ok(profile) => Ok(
                Response::new(
                    GetProfileByIdResponse { profile: profile.map(|profile| profile.into()) }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn create_profile(&self, request: Request<CreateProfileRequest>) -> Result<Response<CreateProfileResponse>, Status> {
        let CreateProfileRequest { id, name, basics, bio } = request.into_inner();
        if id.is_empty() || name.is_empty() || basics.is_none() {
            return status::Status::invalid_arguments(vec!["id", "name", "basics"]);
        }

        let basics = basics.unwrap().into();
        let bio = if bio.is_empty() { None } else { Some(bio) };
        match self.interactor.create_profile(&id, &name, basics, bio).await {
            Ok(id) => Ok(
                Response::new(
                    CreateProfileResponse { id: String::from(id) }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn update_profile(&self, request: Request<UpdateProfileRequest>) -> Result<Response<UpdateProfileResponse>, Status> {
        let UpdateProfileRequest { id, name, basics, bio } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        let basics = basics.map(|b| b.into());
        match self.interactor.update_profile(&id, name, basics, bio).await {
            Ok(profile) => Ok(
                Response::new(
                    UpdateProfileResponse { profile: Some(profile.into()) }
                )
            ),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn delete_profile(&self, request: Request<DeleteProfileRequest>) -> Result<Response<DeleteProfileResponse>, Status> {
        let DeleteProfileRequest { id } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.delete_profile(&id).await {
            Ok(id) => Ok(Response::new(DeleteProfileResponse { id: String::from(id) })),
            Err(error) => status::Status::internal(error)
        }
    }
}