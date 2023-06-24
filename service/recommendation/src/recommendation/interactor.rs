use crate::profile::entity::Filter;
use crate::recommendation::repository::RecommendationRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait RecommendationInteractor {
    async fn get_candidates(&self, excepted_id: &str, filter: Filter) -> Result<Vec<String>, Error>;
    async fn delete_candidate(&self, excepted_id: &str, candidate_id: &str) -> Result<(), Error>;
}

pub struct RecommendationInteractorImpl {
    repository: Box<dyn RecommendationRepository + Send + Sync>,
}

impl RecommendationInteractorImpl {
    pub fn new(repository: Box<dyn RecommendationRepository + Send + Sync>) -> Box<dyn RecommendationInteractor + Send + Sync> {
        Box::new(RecommendationInteractorImpl { repository })
    }
}

#[async_trait::async_trait]
impl RecommendationInteractor for RecommendationInteractorImpl {
    async fn get_candidates(&self, excepted_id: &str, filter: Filter) -> Result<Vec<String>, Error> {
        self.repository.get_candidate_identifiers(excepted_id, filter).await
    }

    async fn delete_candidate(&self, excepted_id: &str, candidate_id: &str) -> Result<(), Error> {
        self.repository.delete_candidate(excepted_id, candidate_id).await
    }
}