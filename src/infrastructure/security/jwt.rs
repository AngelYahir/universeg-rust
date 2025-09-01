use crate::app::ports::{AppError, JwtService};
use async_trait::async_trait;
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;
use uuid::Uuid;

pub struct Hs256Jwt {
    key: EncodingKey,
    exp_hours: i64,
}

impl Hs256Jwt {
    pub fn new(secret: &str, exp_hours: i64) -> Self {
        Self {
            key: EncodingKey::from_secret(secret.as_bytes()),
            exp_hours,
        }
    }
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[async_trait]
impl JwtService for Hs256Jwt {
    async fn sign(&self, user_id: Uuid) -> Result<String, AppError> {
        let exp =
            (chrono::Utc::now() + chrono::Duration::hours(self.exp_hours)).timestamp() as usize;
        let token = jsonwebtoken::encode(
            &Header::default(),
            &Claims {
                sub: user_id.to_string(),
                exp,
            },
            &self.key,
        )
        .map_err(|e| AppError::Infra(anyhow::anyhow!("jwt sign error: {}", e)))?;
        Ok(token)
    }
}
