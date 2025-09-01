use super::pool::DbPool;
use anyhow::Context;
use async_trait::async_trait;
use sqlx::FromRow;
use uuid::Uuid;

use crate::app::ports::{AppError, UserRepository};
use crate::domain::auth::{Email, PasswordHash, User, Username};

#[derive(Clone)]
pub struct PgUserRepository {
    pool: DbPool,
}

impl PgUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    username: String,
    password_hash: String,
}

impl TryFrom<UserRow> for User {
    type Error = AppError;

    fn try_from(r: UserRow) -> Result<Self, Self::Error> {
        let email = Email::parse(&r.email)?;
        let username = Username::parse(&r.username)?;
        let password_hash = PasswordHash::from_hash(r.password_hash)?;
        Ok(User::new(r.id, email, username, password_hash))
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError> {
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, email, username, password_hash FROM users WHERE email = $1",
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .context("users.find error")
        .map_err(AppError::from)?;

        row.map(User::try_from).transpose()
    }

    async fn create(
        &self,
        email: Email,
        username: Username,
        password_hash: String,
    ) -> Result<User, AppError> {
        let rec = sqlx::query_as::<_, UserRow>(
            "INSERT INTO users (email, username, password_hash)
             VALUES ($1, $2, $3)
             RETURNING id, email, username, password_hash",
        )
        .bind(email.as_str())
        .bind(username.as_str())
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await
        .context("users.create error")
        .map_err(AppError::from)?;

        User::try_from(rec)
    }
}
