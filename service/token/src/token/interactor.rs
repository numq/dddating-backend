use crate::token::repository::TokenRepository;

type Error = Box<dyn error::Error + Send + Sync>;

#[async_trait::async_trait]
pub trait TokenInteractor {
    async fn generate_access_token(&self, payload: &str) -> Result<String, Error>;
    async fn generate_refresh_token(&self, payload: &str) -> Result<String, Error>;
    async fn validate_token(&self, token: &str) -> Result<String, Error>;
    async fn invalidate_token(&self, token: &str) -> Result<(), Error>;
}

pub struct TokenInteractorImpl {
    repository: Box<dyn TokenRepository + Send + Sync>,
}

impl TokenInteractorImpl {
    pub fn new(repository: Box<dyn TokenRepository + Send + Sync>) -> Box<dyn TokenInteractor + Send + Sync> {
        Box::new(TokenInteractorImpl { repository })
    }
}

#[async_trait::async_trait]
impl TokenInteractor for TokenInteractorImpl {
    async fn generate_access_token(&self, payload: &str) -> Result<String, Error> {
        self.repository.generate_access_token(payload).await
    }

    async fn generate_refresh_token(&self, payload: &str) -> Result<String, Error> {
        self.repository.generate_refresh_token(payload).await
    }

    async fn validate_token(&self, token: &str) -> Result<String, Error> {
        self.repository.validate_token(token).await
    }

    async fn invalidate_token(&self, token: &str) -> Result<(), Error> {
        self.repository.invalidate_token(token).await
    }
}