use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Like {
    #[serde(rename = "_id")]
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub created_at: u64,
}

impl Like {
    pub fn new(id: &str, from_id: &str, to_id: &str) -> Self {
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        Self { id: String::from(id), from_id: String::from(from_id), to_id: String::from(to_id), created_at }
    }
}