use crate::profile::entity::{Basics, Filter, Profile};
use crate::profile::repository::ProfileRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait ProfileInteractor {
    async fn get_random_profiles(
        &self,
        excepted_id: &str,
        filter: Filter,
        count: u64,
    ) -> Result<Vec<Profile>, Error>;
    async fn get_profiles(
        &self,
        excepted_id: &str,
        filter: Filter,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Profile>, Error>;
    async fn get_profile_by_id(&self, id: &str) -> Result<Option<Profile>, Error>;
    async fn create_profile(
        &self,
        id: &str,
        name: &str,
        basics: Basics,
        bio: Option<String>,
    ) -> Result<String, Error>;
    async fn update_profile(
        &self,
        id: &str,
        name: Option<String>,
        basics: Option<Basics>,
        bio: Option<String>,
    ) -> Result<Profile, Error>;
    async fn delete_profile(&self, id: &str) -> Result<String, Error>;
}

pub struct ProfileInteractorImpl {
    repository: Box<dyn ProfileRepository + Send + Sync>,
}

impl ProfileInteractorImpl {
    pub fn new(repository: Box<dyn ProfileRepository + Send + Sync>) -> Box<dyn ProfileInteractor + Send + Sync> {
        Box::new(ProfileInteractorImpl { repository })
    }
}

#[async_trait::async_trait]
impl ProfileInteractor for ProfileInteractorImpl {
    async fn get_random_profiles(&self, excepted_id: &str, filter: Filter, count: u64) -> Result<Vec<Profile>, Error> {
        self.repository.get_random_profiles(excepted_id, filter, count).await
    }


    async fn get_profiles(&self, excepted_id: &str, filter: Filter, skip: u64, limit: u64) -> Result<Vec<Profile>, Error> {
        self.repository.get_profiles(excepted_id, filter, skip, limit).await
    }

    async fn get_profile_by_id(&self, id: &str) -> Result<Option<Profile>, Error> {
        self.repository.get_profile_by_id(id).await
    }

    async fn create_profile(&self, id: &str, name: &str, basics: Basics, bio: Option<String>) -> Result<String, Error> {
        self.repository.create_profile(id, name, basics, bio).await
    }

    async fn update_profile(&self, id: &str, name: Option<String>, basics: Option<Basics>, bio: Option<String>) -> Result<Profile, Error> {
        self.repository.update_profile(id, name, basics, bio).await
    }

    async fn delete_profile(&self, id: &str) -> Result<String, Error> {
        self.repository.delete_profile(id).await
    }
}