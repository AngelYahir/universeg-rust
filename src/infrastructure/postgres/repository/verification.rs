use anyhow::Context;
use async_trait::async_trait;
use tracing::error;
use uuid::Uuid;

use super::super::pool::DbPool;
use crate::app::ports::{AppError, VerificationRepository};

#[derive(Clone)]
pub struct PgVerificationRepository {
    pool: DbPool,
}

impl PgVerificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VerificationRepository for PgVerificationRepository {
    async fn verify_email(&self, token: String) -> Result<(), AppError> {
        // Decode hex string to bytes for bytea compatibility
        let token_bytes = hex::decode(&token).map_err(|_| AppError::InvalidToken)?;

        let result = sqlx::query("CALL verify_email($1)")
            .bind(token_bytes)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) => {
                let message = db_err.message();
                error!(error = %message, "Database error during email verification");

                match message {
                    "Invalid verification token" => Err(AppError::InvalidToken),
                    "Verification token expired" => Err(AppError::TokenExpired),
                    "Verification token already used" => Err(AppError::TokenAlreadyUsed),
                    _ => Err(AppError::Infra(anyhow::anyhow!(
                        "Database error: {}",
                        message
                    ))),
                }
            }
            Err(e) => {
                error!(error = %e, "SQL query failed");
                Err(AppError::Infra(anyhow::anyhow!("SQL query error: {}", e)))
            }
        }
    }

    async fn add_ver_token(
        &self,
        user_id: Uuid,
        token: String,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError> {
        // Decode hex string to bytes for bytea compatibility
        let token_bytes = hex::decode(&token).map_err(|_| AppError::InvalidToken)?;

        sqlx::query("CALL add_email_verification($1, $2, $3)")
            .bind(user_id)
            .bind(token_bytes)
            .bind(expiration)
            .execute(&self.pool)
            .await
            .inspect_err(|e| {
                error!(error = %e, "SQL query failed");
            })
            .context("users.add_ver_token error")
            .map_err(AppError::from)?;

        Ok(())
    }
}
