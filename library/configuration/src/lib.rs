use std::env::var;
use std::error::Error;
use std::path::Path;

pub struct Config {
    pub secret_key: Option<String>,
    pub default_hostname: Option<String>,
    pub amqp_hostname: Option<String>,
    pub amqp_port: Option<String>,
    pub mongo_hostname: Option<String>,
    pub mongo_port: Option<String>,
    pub redis_hostname: Option<String>,
    pub redis_port: Option<String>,
    pub service_hostname: Option<String>,
    pub service_port: Option<String>,
}

impl Config {
    ///
    /// # Arguments
    ///
    /// * `path` - A string that represents the path to the env file
    ///
    /// * `service_name` - A string that represents the name of the service name and exists in env file
    ///
    pub fn new(path: &str, service_name: &str) -> Result<Self, Box<dyn Error>> {
        dotenv::from_path(Path::new(path))?;
        let service_name = service_name.trim().to_uppercase();
        Ok(Self {
            secret_key: var("SECRET_KEY").ok(),
            default_hostname: var("DEFAULT_HOSTNAME").ok(),
            amqp_hostname: var("DEFAULT_HOSTNAME").ok(),
            amqp_port: var("AMQP_PORT").ok(),
            redis_hostname: var("DEFAULT_HOSTNAME").ok(),
            redis_port: var("REDIS_PORT").ok(),
            mongo_hostname: var("DEFAULT_HOSTNAME").ok(),
            mongo_port: var("MONGO_PORT").ok(),
            service_hostname: var(format!("{}_HOSTNAME", service_name)).or_else(|_| var("DEFAULT_HOSTNAME")).ok(),
            service_port: var(format!("{}_PORT", service_name)).ok(),
        })
    }

    pub fn default(service_name: &str) -> Result<Self, Box<dyn Error>> {
        Self::new(".env", service_name)
    }

    pub fn find_port(&self, service_name: &str) -> Result<String, Box<dyn Error>> {
        var(format!("{}_PORT", service_name)).map_err(|err| Box::new(err) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let service_name = "test";
        let config = Config::new(".env", service_name).unwrap();
        assert_eq!(config.service_hostname, Some(String::from("0.0.0.0")));
        assert_eq!(config.service_port, Some(String::from("8080")));
    }

    #[test]
    fn test_default() {
        let service_name = "test";
        let config = Config::default(service_name).unwrap();
        assert_eq!(config.service_hostname, Some(String::from("0.0.0.0")));
        assert_eq!(config.service_port, Some(String::from("8080")));
    }
}
