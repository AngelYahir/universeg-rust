use crate::app::ports::{AppError, PasswordHasher};
use async_trait::async_trait;

pub struct BcryptHasher;

#[async_trait]
impl PasswordHasher for BcryptHasher {
    async fn hash(&self, password: &str) -> Result<String, AppError> {
        let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::Infra(anyhow::anyhow!("bcrypt hash error: {}", e)))?;
        Ok(hashed)
    }
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let is_valid = bcrypt::verify(password, hash)
            .map_err(|e| AppError::Infra(anyhow::anyhow!("bcrypt verify error: {}", e)))?;
        Ok(is_valid)
    }
}
