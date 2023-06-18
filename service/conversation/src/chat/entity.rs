use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::message::entity::Message;

#[derive(Serialize, Deserialize)]
pub struct Chat {
    #[serde(rename = "_id")]
    pub id: String,
    pub member_ids: Vec<String>,
    pub last_message: Option<Message>,
    pub typing_member_ids: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Chat {
    pub fn timestamp_now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    pub fn new(id: &str, member_ids: Vec<String>) -> Self {
        Self {
            id: String::from(id),
            member_ids,
            last_message: None,
            typing_member_ids: vec![],
            created_at: Self::timestamp_now(),
            updated_at: Self::timestamp_now(),
        }
    }
}