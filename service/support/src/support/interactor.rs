use async_trait::async_trait;

use crate::support::entity::{Status, Ticket};
use crate::support::repository::SupportRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait SupportInteractor {
    async fn get_tickets(
        &self,
        user_id: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Ticket>, Error>;
    async fn get_ticket_by_id(&self, id: &str) -> Result<Ticket, Error>;
    async fn create_ticket(
        &self,
        user_id: &str,
        topic: &str,
        description: &str,
    ) -> Result<Status, Error>;
    async fn update_ticket(
        &self,
        ticket_id: &str,
        description: &str,
    ) -> Result<Status, Error>;
    async fn delete_ticket(&self, ticket_id: &str) -> Result<String, Error>;
}

pub struct SupportInteractorImpl {
    repository: Box<dyn SupportRepository + Send + Sync>,
}

impl SupportInteractorImpl {
    pub fn new(repository: Box<dyn SupportRepository + Send + Sync>) -> Box<dyn SupportInteractor + Send + Sync> {
        Box::new(SupportInteractorImpl { repository })
    }
}

#[async_trait]
impl SupportInteractor for SupportInteractorImpl {
    async fn get_tickets(
        &self,
        user_id: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Ticket>, Error> {
        self.repository.get_tickets(user_id, skip, limit).await
    }

    async fn get_ticket_by_id(&self, id: &str) -> Result<Ticket, Error> {
        self.repository.get_ticket_by_id(id).await
    }

    async fn create_ticket(
        &self,
        user_id: &str,
        topic: &str,
        description: &str,
    ) -> Result<Status, Error> {
        self.repository.create_ticket(user_id, topic, description).await
    }

    async fn update_ticket(&self, ticket_id: &str, description: &str) -> Result<Status, Error> {
        self.repository.update_ticket(ticket_id, description).await
    }

    async fn delete_ticket(&self, ticket_id: &str) -> Result<String, Error> {
        self.repository.delete_ticket(ticket_id).await
    }
}