use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockedUser {
    #[serde(rename = "_id")]
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl BlockedUser {
    pub fn timestamp_now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    pub fn new(id: &str, from_id: &str, to_id: &str) -> Self {
        let now = BlockedUser::timestamp_now();
        Self {
            id: String::from(id),
            from_id: String::from(from_id),
            to_id: String::from(to_id),
            created_at: now,
            updated_at: now,
        }
    }
}