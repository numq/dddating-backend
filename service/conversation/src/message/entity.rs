use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,
    pub chat_id: String,
    pub sender_id: String,
    pub text: Option<String>,
    pub images: Vec<Vec<u8>>,
    pub is_delivered: bool,
    pub is_read: bool,
    pub sent_at: u64,
}

impl Message {
    pub fn timestamp_now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    pub fn new(id: &str, chat_id: &str, sender_id: &str, text: Option<String>, images: Vec<Vec<u8>>) -> Self {
        Self {
            id: String::from(id),
            chat_id: String::from(chat_id),
            sender_id: String::from(sender_id),
            text,
            images,
            is_delivered: false,
            is_read: false,
            sent_at: Self::timestamp_now(),
        }
    }
}