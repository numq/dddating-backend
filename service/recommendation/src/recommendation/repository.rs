use std::time::Duration;

use redis::{Client as RedisClient, Commands, transaction};

use crate::matchmaking::api::MatchmakingApi;
use crate::profile::api::ProfileApi;
use crate::profile::entity::Filter;

type Error = Box<dyn std::error::Error + Send + Sync>;

const CHUNK_SIZE: u64 = 10;

#[async_trait::async_trait]
pub trait RecommendationRepository {
    async fn get_candidate_identifiers(&self, excepted_id: &str, filter: Filter) -> Result<Vec<String>, Error>;
    async fn delete_candidate(&self, excepted_id: &str, candidate_id: &str) -> Result<(), Error>;
}

pub struct RecommendationRepositoryImpl {
    redis_client: RedisClient,
    matchmaking_api: Box<dyn MatchmakingApi + Send + Sync>,
    profile_api: Box<dyn ProfileApi + Send + Sync>,
}

impl RecommendationRepositoryImpl {
    pub async fn new(
        redis_client: RedisClient,
        matchmaking_api: Box<dyn MatchmakingApi + Send + Sync>,
        profile_api: Box<dyn ProfileApi + Send + Sync>,
    ) -> Box<dyn RecommendationRepository + Send + Sync> {
        Box::new(RecommendationRepositoryImpl { redis_client, matchmaking_api, profile_api })
    }
}

#[async_trait::async_trait]
impl RecommendationRepository for RecommendationRepositoryImpl {
    async fn get_candidate_identifiers(&self, excepted_id: &str, filter: Filter) -> Result<Vec<String>, Error> {
        let mut redis = self.redis_client.get_connection()?;
        // try to get cached candidate identifiers
        if let Ok(count) = redis.hlen::<&str, u64>(excepted_id) {
            if count >= CHUNK_SIZE {
                if let Ok(candidate_identifiers) = redis.hgetall::<&str, Vec<String>>(excepted_id) {
                    return Ok(candidate_identifiers.into_iter().take(CHUNK_SIZE as usize).collect());
                }
            }
        }
        // request profiles from profile service
        let profiles = self.profile_api.get_random_profiles(excepted_id, filter, CHUNK_SIZE * 2).await?;
        if profiles.is_empty() {
            // no profiles to check
            return Ok(vec![]);
        }
        let profile_identifiers: Vec<String> = profiles.into_iter().map(|profile| profile.id).collect();
        let checked_identifiers: Vec<bool> = self.matchmaking_api.check_ids(excepted_id, profile_identifiers.clone()).await?;
        let mut candidate_identifiers: Vec<String> = vec![];
        for (id, liked_or_disliked) in profile_identifiers.into_iter().zip(checked_identifiers.into_iter()).collect::<Vec<(String, bool)>>() {
            // put identifier into temporary cache
            if let Ok(_) = transaction(&mut redis, &[excepted_id], |con, pipe| {
                pipe
                    .hset::<&str, &str, bool>(excepted_id, &id, liked_or_disliked)
                    .expire(excepted_id, Duration::from_secs(60 * 60 * 12).as_secs() as usize)
                    .query::<Option<()>>(con)
            }) {
                if !liked_or_disliked {
                    candidate_identifiers.push(String::from(id))
                }
            };
        }
        Ok(candidate_identifiers)
    }

    async fn delete_candidate(&self, excepted_id: &str, candidate_id: &str) -> Result<(), Error> {
        let mut redis = self.redis_client.get_connection()?;
        let _ = redis.hdel(excepted_id, candidate_id)?;
        return Ok(());
    }
}