use crate::authentication::repository::AuthenticationRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait AuthenticationInteractor {
    async fn is_token_valid(&self, access_token: &str) -> Result<bool, Error>;
}

pub struct AuthenticationInteractorImpl {
    repository: Box<dyn AuthenticationRepository + Send + Sync>,
}

impl AuthenticationInteractorImpl {
    pub fn new(repository: Box<dyn AuthenticationRepository + Send + Sync>) -> Box<dyn AuthenticationInteractor + Send + Sync> {
        Box::new(AuthenticationInteractorImpl { repository })
    }
}

#[async_trait::async_trait]
impl AuthenticationInteractor for AuthenticationInteractorImpl {
    async fn is_token_valid(&self, access_token: &str) -> Result<bool, Error> {
        self.repository.is_token_valid(access_token).await
    }
}