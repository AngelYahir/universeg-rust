use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub debug: bool,
    pub jwt_secret: String,
    pub jwt_exp_hours: i64,
    pub mongo_uri: String,
    pub mongo_db_name: String,
    pub from_email: String,
    pub from_name: String,
    pub relay: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub pool_size: Option<u32>,
    pub start_tls: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();

        let database_url = require_env("DATABASE_URL")?;
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| anyhow::anyhow!("Invalid Port: {e}"))?;
        let debug = env::var("LOG_DEBUG")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .map_err(|e| anyhow::anyhow!("Invalid DEBUG value: {e}"))?;
        let jwt_secret = require_env("JWT_SECRET")?;
        let jwt_exp_hours = env::var("JWT_EXP_HOURS")
            .unwrap_or_else(|_| "8".to_string())
            .parse::<i64>()
            .map_err(|e| anyhow::anyhow!("Invalid JWT_EXP_HOURS value: {e}"))?;
        let mongo_uri = require_env("MONGO_URI")?;
        let mongo_db_name = require_env("MONGO_DB_NAME")?;
        let from_email = require_env("FROM_EMAIL")?;
        let from_name = require_env("FROM_NAME")?;
        let relay = require_env("SMTP_RELAY")?;
        let username = env::var("SMTP_USERNAME").ok();
        let password = env::var("SMTP_PASSWORD").ok();
        let pool_size = env::var("SMTP_POOL_SIZE").ok().and_then(|s| s.parse().ok());
        let start_tls = env::var("SMTP_START_TLS").ok().and_then(|s| s.parse().ok()).unwrap_or(true);

        Ok(Self {
            database_url,
            port,
            debug,
            jwt_secret,
            jwt_exp_hours,
            mongo_uri,
            mongo_db_name,
            from_email,
            from_name,
            relay,
            username,
            password,
            pool_size,
            start_tls,       
        })
    }
}

fn require_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Environment variable {key} is not set"))
}
