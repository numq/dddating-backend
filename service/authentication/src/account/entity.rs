use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Role {
    User,
    Moderator,
}

pub struct Account {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub role: Role,
    pub created_at: u64,
    pub updated_at: u64,
}