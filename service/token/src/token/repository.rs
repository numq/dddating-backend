use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use redis::{Client as RedisClient, Commands};
use sha2::Sha256;

use error::make_error;

type Error = Box<dyn error::Error + Send + Sync>;

#[async_trait]
pub trait TokenRepository {
    async fn generate_access_token(&self, payload: &str) -> Result<String, Error>;
    async fn generate_refresh_token(&self, payload: &str) -> Result<String, Error>;
    async fn validate_token(&self, token: &str) -> Result<String, Error>;
    async fn invalidate_token(&self, token: &str) -> Result<(), Error>;
}

pub struct TokenRepositoryImpl {
    secret_key: String,
    redis_client: RedisClient,
}

impl TokenRepositoryImpl {
    fn generate_token(&self, payload: &str, expiration_millis: u64) -> Result<String, Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret_key.as_ref())?;
        let mut claims = BTreeMap::new();
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let now_millis = now.as_millis() as u64;
        let exp = now_millis + expiration_millis;
        claims.insert("iat", now_millis.to_string());
        claims.insert("exp", exp.to_string());
        claims.insert("payload", payload.to_string());
        if let Ok(token) = claims.sign_with_key(&key) {
            return Ok(token);
        }
        Err(make_error!("unable to generate token"))
    }

    fn verify_token(&self, token: &str) -> Result<BTreeMap<String, String>, Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.secret_key.as_ref())?;
        let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
        if vec!("iat", "exp", "payload").iter().all(|&key| claims.contains_key(key)) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
            if let Some(exp) = claims.get("exp") {
                if now < exp.parse::<u64>()? {
                    return Ok(claims);
                }
            }
        }
        Err(make_error!("invalid token"))
    }

    pub fn new(secret_key: String, redis_client: RedisClient) -> Box<dyn TokenRepository + Send + Sync> {
        Box::new(TokenRepositoryImpl { secret_key, redis_client })
    }
}

#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
    async fn generate_access_token(&self, payload: &str) -> Result<String, Error> {
        self.generate_token(payload, 1000 * 60 * 30)
    }

    async fn generate_refresh_token(&self, payload: &str) -> Result<String, Error> {
        self.generate_token(payload, 1000 * 60 * 60 * 24)
    }

    async fn validate_token(&self, token: &str) -> Result<String, Error> {
        let mut redis = self.redis_client.get_connection()?;
        if let false = redis.exists(token)? {
            if let Ok(claims) = self.verify_token(token) {
                if let Some(payload) = claims.get("payload") {
                    return Ok(String::from(payload));
                }
            }
        }
        Err(make_error!("invalid token"))
    }

    async fn invalidate_token(&self, token: &str) -> Result<(), Error> {
        let mut redis = self.redis_client.get_connection()?;
        if let Ok(claims) = self.verify_token(token) {
            if let Some(exp) = claims.get("exp") {
                if let Ok(exp) = exp.parse::<u64>() {
                    let _ = redis.set_ex::<&str, &str, usize>(token, "", exp as usize);
                }
            }
        }
        Ok(())
    }
}