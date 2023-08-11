use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Role {
    User,
    Moderator,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub role: Role,
    pub premium_expiration_date: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Account {
    pub fn timestamp_now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    pub fn new(id: &str, email: &str, password_hash: &str, password_salt: &str, role: Role) -> Self {
        let now = Account::timestamp_now();
        Self {
            id: String::from(id),
            email: String::from(email),
            password_hash: String::from(password_hash),
            password_salt: String::from(password_salt),
            role,
            premium_expiration_date: 0,
            created_at: now,
            updated_at: now,
        }
    }
}