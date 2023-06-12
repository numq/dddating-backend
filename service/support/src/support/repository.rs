use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson, Collection};
use mongodb::bson::{bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;

use error::make_error;

use crate::support::entity::{Status, Ticket};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait SupportRepository {
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

pub struct SupportRepositoryImpl {
    collection: Collection<Ticket>,
}

impl SupportRepositoryImpl {
    pub fn new(collection: Collection<Ticket>) -> Box<dyn SupportRepository + Send + Sync> {
        Box::new(SupportRepositoryImpl { collection })
    }
}

#[async_trait]
impl SupportRepository for SupportRepositoryImpl {
    async fn get_tickets(&self, user_id: &str, skip: u64, limit: u64) -> Result<Vec<Ticket>, Error> {
        let filter = doc! {"user_id": user_id };
        let mut tickets: Vec<Ticket> = vec![];
        let options = FindOptions::builder().skip(skip).limit(limit.try_into().ok()).build();
        let mut cursor = self.collection.find(filter, options).await?;
        while let Some(ticket) = cursor.try_next().await? {
            tickets.push(ticket)
        }
        Ok(tickets)
    }

    async fn get_ticket_by_id(&self, id: &str) -> Result<Ticket, Error> {
        if let Some(ticket) = self.collection.find_one(doc! { "_id": id }, None).await? {
            return Ok(ticket);
        }
        Err(make_error!("unable to get ticket by id"))
    }

    async fn create_ticket(&self, user_id: &str, topic: &str, description: &str) -> Result<Status, Error> {
        let id = ObjectId::new().to_hex();
        let ticket = Ticket::new(&id, user_id, topic, description);
        let result = self.collection.insert_one(ticket.clone(), None).await;
        if let Ok(_) = result {
            return Ok(ticket.status);
        }
        Err(make_error!("unable to create ticket"))
    }

    async fn update_ticket(&self, ticket_id: &str, description: &str) -> Result<Status, Error> {
        let timestamp = bson!(Ticket::timestamp_now() as i64);
        let mut document = doc! {
            "description": description,
            "updated_at": timestamp
        };
        let result = self.collection.update_one(doc! { "_id": ticket_id }, doc! { "$set": document }, None).await?;
        if result.modified_count > 0 {
            return self.get_ticket_by_id(ticket_id).await.map(|ticket| ticket.status);
        }
        Err(make_error!("unable to update ticket"))
    }

    async fn delete_ticket(&self, ticket_id: &str) -> Result<String, Error> {
        if self.collection.delete_one(doc! { "_id": ticket_id }, None).await?.deleted_count > 0 {
            return Ok(String::from(ticket_id));
        }
        Err(make_error!("unable to delete ticket"))
    }
}