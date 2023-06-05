use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Active,
    Canceled,
    Closed,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    #[serde(rename = "_id")]
    pub id: String,
    pub user_id: String,
    pub topic: String,
    pub description: String,
    pub status: Status,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Ticket {
    pub fn timestamp_now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    pub fn new(id: &str, user_id: &str, topic: &str, description: &str) -> Self {
        let now = Ticket::timestamp_now();
        Self {
            id: String::from(id),
            user_id: String::from(user_id),
            topic: String::from(topic),
            description: String::from(description),
            status: Status::Active,
            created_at: now,
            updated_at: now,
        }
    }
}