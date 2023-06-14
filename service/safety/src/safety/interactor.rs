use crate::safety::repository::SafetyRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait SafetyInteractor {
    async fn check_user_id(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<bool, Error>;
    async fn get_blocked_users(
        &self,
        from_id: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<String>, Error>;
    async fn block_user(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<(), Error>;
    async fn unblock_user(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<(), Error>;
}

pub struct SafetyInteractorImpl {
    repository: Box<dyn SafetyRepository + Send + Sync>,
}

impl SafetyInteractorImpl {
    pub fn new(repository: Box<dyn SafetyRepository + Send + Sync>) -> Box<dyn SafetyInteractor + Send + Sync> {
        Box::new(SafetyInteractorImpl { repository })
    }
}

#[async_trait::async_trait]
impl SafetyInteractor for SafetyInteractorImpl {
    async fn check_user_id(&self, from_id: &str, to_id: &str) -> Result<bool, Error> {
        self.repository.check_user_id(from_id, to_id).await
    }

    async fn get_blocked_users(&self, from_id: &str, skip: u64, limit: u64) -> Result<Vec<String>, Error> {
        self.repository.get_blocked_users(from_id, skip, limit).await
    }

    async fn block_user(&self, from_id: &str, to_id: &str) -> Result<(), Error> {
        self.repository.block_user(from_id, to_id).await
    }

    async fn unblock_user(&self, from_id: &str, to_id: &str) -> Result<(), Error> {
        self.repository.unblock_user(from_id, to_id).await
    }
}