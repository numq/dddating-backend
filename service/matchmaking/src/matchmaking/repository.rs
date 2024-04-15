use std::i64;
use std::time::Duration;

use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;
use mongodb::options::FindOptions;
use redis::{Client as RedisClient, Commands, transaction};

use amqp::MessageQueue;
use error::make_error;

use crate::matchmaking::entity::Like;

type Error = Box<dyn std::error::Error + Send + Sync>;

const EXCHANGE_NAME: &str = "action";
const MATCH_ROUTING_KEY: &str = "match";
const DISLIKE_ROUTING_KEY: &str = "dislike";

#[async_trait::async_trait]
pub trait MatchmakingRepository {
    async fn check_ids(&self, from_id: &str, to_ids: Vec<String>) -> Result<Vec<bool>, Error>;
    async fn get_likes(&self, id: &str, skip: u64, limit: u64) -> Result<Vec<String>, Error>;
    async fn like(&self, from_id: &str, to_id: &str) -> Result<bool, Error>;
    async fn dislike(&self, from_id: &str, to_id: &str) -> Result<(), Error>;
    async fn backtrack(&self, id: &str) -> Result<String, Error>;
}

pub struct MatchmakingRepositoryImpl {
    collection: Collection<Like>,
    redis_client: RedisClient,
    message_queue: MessageQueue,
}

impl MatchmakingRepositoryImpl {
    pub async fn new(collection: Collection<Like>, redis_client: RedisClient, message_queue: MessageQueue) -> Box<dyn MatchmakingRepository + Send + Sync> {
        Box::new(MatchmakingRepositoryImpl { collection, redis_client, message_queue })
    }
}

#[async_trait::async_trait]
impl MatchmakingRepository for MatchmakingRepositoryImpl {
    async fn check_ids(&self, from_id: &str, to_ids: Vec<String>) -> Result<Vec<bool>, Error> {
        let mut identifiers: Vec<bool> = vec![];
        let mut redis = self.redis_client.get_connection()?;
        for to_id in to_ids {
            identifiers.push(if redis.hget::<&str, &str, Option<i32>>(from_id, &to_id)?.is_some() {
                true
            } else {
                self.collection.find_one(doc! { "from_id": from_id, "to_id": to_id }, None).await?.is_some()
            });
        }
        Ok(identifiers)
    }

    async fn get_likes(&self, id: &str, skip: u64, limit: u64) -> Result<Vec<String>, Error> {
        let filter = doc! { "to_id": id };
        let mut identifiers: Vec<String> = vec![];
        let (skip, limit) = (skip.try_into().unwrap_or(0), limit.try_into().unwrap_or(50));
        let options = FindOptions::builder().skip(skip).limit(limit).sort(doc! { "created_at": 1 }).build();
        let mut cursor = self.collection.find(filter, options).await?;
        while let Some(action) = cursor.try_next().await? {
            identifiers.push(action.from_id)
        }
        Ok(identifiers)
    }

    async fn like(&self, from_id: &str, to_id: &str) -> Result<bool, Error> {
        let id = ObjectId::new().to_hex();
        if let Ok(_) = self.collection.insert_one(Like::new(&id, from_id, to_id), None).await {
            if let Some(_) = self.collection.find_one(doc! { "from_id": to_id, "to_id": from_id }, None).await? {
                let _ = self.message_queue.publish(EXCHANGE_NAME, MATCH_ROUTING_KEY, vec![to_id, from_id].join(",").as_bytes());
                return Ok(true);
            }
            return Ok(false);
        }
        Err(make_error!("unable to like"))
    }

    async fn dislike(&self, from_id: &str, to_id: &str) -> Result<(), Error> {
        let mut redis = self.redis_client.get_connection()?;
        transaction(&mut redis, &[from_id], |con, pipe| {
            pipe
                .sadd(from_id, to_id)
                .expire(from_id, 60 * 60 * 12 * 30)
                .query(con)
        })?;

        let message = [to_id, from_id].join(",");
        let _ = self.message_queue.publish(EXCHANGE_NAME, DISLIKE_ROUTING_KEY, message.as_bytes());

        Err(make_error!("unable to dislike"))
    }

    async fn backtrack(&self, id: &str) -> Result<String, Error> {
        let mut redis = self.redis_client.get_connection()?;
        if let Ok(backtracked_id) = redis.spop::<&str, String>(id) {
            return Ok(backtracked_id);
        }
        Err(make_error!("unable to backtrack"))
    }
}