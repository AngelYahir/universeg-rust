use crate::app::ports::{AppError, JwtService};
use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Hs256Jwt {
    key: EncodingKey,
    dec: DecodingKey,
    exp_hours: i64,
}

impl Hs256Jwt {
    pub fn new(secret: &str, exp_hours: i64) -> Self {
        Self {
            key: EncodingKey::from_secret(secret.as_bytes()),
            dec: DecodingKey::from_secret(secret.as_bytes()),
            exp_hours,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
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

    async fn verify(&self, token: &str) -> Result<Uuid, AppError> {
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = jsonwebtoken::decode::<Claims>(token, &self.dec, &validation)
            .map_err(|e| AppError::Infra(anyhow::anyhow!("jwt verify error: {}", e)))?;
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|e| AppError::Infra(anyhow::anyhow!("invalid user id in token: {}", e)))?;
        Ok(user_id)
    }
}
