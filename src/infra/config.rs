use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();

        let database_url = require_env("DATABASE_URL")?;
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| anyhow::anyhow!("Invalid Port: {e}"))?;

        Ok(Self { database_url, port })
    }
}

fn require_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Environment variable {key} is not set"))
}
