use crate::app::ports::{AppError, UserRepository};
use async_trait::async_trait;
use uuid::Uuid;

pub struct GetInfoCommand {
    pub user_id: Uuid,
}

pub struct GetInfoResult {
    pub username: String,
    pub email: String,
    pub id: String,
    pub is_email_verified: bool,
}

#[async_trait]
pub trait GetInfoHandler: Send + Sync {
    async fn handle(&self, cmd: GetInfoCommand) -> Result<GetInfoResult, AppError>;
}

pub struct GetInfoHandlerImpl<R> {
    repo: R,
}

impl<R> GetInfoHandlerImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> GetInfoHandler for GetInfoHandlerImpl<R>
where
    R: UserRepository + Send + Sync,
{
    async fn handle(&self, cmd: GetInfoCommand) -> Result<GetInfoResult, AppError> {
        let user = self
            .repo
            .find_by_id(cmd.user_id)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(GetInfoResult {
            username: user.username.as_str().to_owned(),
            email: user.email.as_str().to_owned(),
            id: user.id.to_string(),
            is_email_verified: user.is_email_verified,
        })
    }
}
