use crate::matchmaking::repository::MatchmakingRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait MatchmakingInteractor {
    async fn check_ids(&self, from_id: &str, to_ids: Vec<String>) -> Result<Vec<bool>, Error>;
    async fn get_likes(&self, id: &str, skip: u64, limit: u64) -> Result<Vec<String>, Error>;
    async fn like(&self, from_id: &str, to_id: &str) -> Result<bool, Error>;
    async fn dislike(&self, from_id: &str, to_id: &str) -> Result<(), Error>;
    async fn backtrack(&self, id: &str) -> Result<String, Error>;
}

pub struct MatchmakingInteractorImpl {
    repository: Box<dyn MatchmakingRepository + Send + Sync>,
}

impl MatchmakingInteractorImpl {
    pub fn new(repository: Box<dyn MatchmakingRepository + Send + Sync>) -> Box<dyn MatchmakingInteractor + Send + Sync> {
        Box::new(MatchmakingInteractorImpl { repository })
    }
}

#[async_trait::async_trait]
impl MatchmakingInteractor for MatchmakingInteractorImpl {
    async fn check_ids(&self, from_id: &str, to_ids: Vec<String>) -> Result<Vec<bool>, Error> {
        self.repository.check_ids(from_id, to_ids).await
    }

    async fn get_likes(&self, id: &str, skip: u64, limit: u64) -> Result<Vec<String>, Error> {
        self.repository.get_likes(id, skip, limit).await
    }

    async fn like(&self, from_id: &str, to_id: &str) -> Result<bool, Error> {
        self.repository.like(from_id, to_id).await
    }

    async fn dislike(&self, from_id: &str, to_id: &str) -> Result<(), Error> {
        self.repository.dislike(from_id, to_id).await
    }

    async fn backtrack(&self, id: &str) -> Result<String, Error> {
        self.repository.backtrack(id).await
    }
}