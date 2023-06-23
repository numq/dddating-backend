use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Gender {
    NonBinary,
    Male,
    Female,
}

#[derive(Serialize, Deserialize)]
pub struct Basics {
    pub age: u32,
    pub gender: Gender,
    pub location: String,
    pub preferences: Vec<Gender>,
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
    pub min_age: u32,
    pub max_age: u32,
    pub location: String,
    pub preferences: Vec<Gender>,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub basics: Basics,
    pub bio: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Profile {
    pub fn timestamp_now() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    pub fn new(id: &str, name: &str, basics: Basics, bio: Option<String>) -> Self {
        let now = Profile::timestamp_now();
        Self {
            id: String::from(id),
            name: String::from(name),
            basics,
            bio: bio.map_or(String::new(), |s| String::from(s)),
            created_at: now,
            updated_at: now,
        }
    }
}