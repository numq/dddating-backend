use tonic::Request;
use tonic::transport::Channel;

use error::make_error;

use crate::profile::entity::{Filter, Profile};
use crate::profile::pb::{GetRandomProfilesRequest, GetRandomProfilesResponse};
use crate::profile::pb::profile_service_client::ProfileServiceClient;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tonic::async_trait]
pub trait ProfileApi {
    async fn get_random_profiles(&self, excepted_id: &str, filter: Filter, count: u64) -> Result<Vec<Profile>, Error>;
}

pub struct ProfileApiImpl {
    client: ProfileServiceClient<Channel>,
}

impl ProfileApiImpl {
    pub fn new(client: ProfileServiceClient<Channel>) -> Box<dyn ProfileApi + Send + Sync> {
        Box::new(ProfileApiImpl { client })
    }
}

#[tonic::async_trait]
impl ProfileApi for ProfileApiImpl {
    async fn get_random_profiles(&self, excepted_id: &str, filter: Filter, count: u64) -> Result<Vec<Profile>, Error> {
        let request = Request::new(
            GetRandomProfilesRequest {
                excepted_id: String::from(excepted_id),
                filter: Some(filter.into()),
                count,
            }
        );
        if let Ok(response) = self.client.clone().get_random_profiles(request).await {
            let GetRandomProfilesResponse { profiles } = response.into_inner();
            return Ok(profiles.into_iter().map(|profile| profile.into()).collect());
        }
        Err(make_error!("unable to get profiles"))
    }
}