use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use error::make_error;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Hash = String;
type Salt = String;

pub trait Hasher {
    fn hash_password(&self, password_string: &str) -> Result<(Hash, Salt), Error>;
    fn verify_password(&self, password_string: &str, hash: &str) -> Result<(), Error>;
}

pub struct DefaultHasher<'a> {
    context: Argon2<'a>,
}

impl DefaultHasher<'_> {
    pub fn new() -> Box<dyn Hasher + Send + Sync> {
        Box::new(DefaultHasher { context: Argon2::default() })
    }
}

impl Hasher for DefaultHasher<'_> {
    fn hash_password(&self, password_string: &str) -> Result<(Hash, Salt), Error> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        return if let Ok(hash) = self.context.hash_password(password_string.as_bytes(), &salt) {
            Ok((hash.to_string(), salt.to_string()))
        } else {
            Err(make_error!("unable to encrypt password"))
        };
    }

    fn verify_password(&self, password_string: &str, hash: &str) -> Result<(), Error> {
        let password_hash = PasswordHash::new(hash).map_err(|_| make_error!("unable to hash password")).unwrap();
        return if let Ok(_) = self.context.verify_password(password_string.as_bytes(), &password_hash) {
            Ok(())
        } else {
            Err(make_error!("invalid password"))
        };
    }
}