use async_trait::async_trait;

#[derive(Debug)]
pub struct MailContent {
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum MailError {
    #[error("Failed to send email: {0}")]
    SendError(String),
    #[error("Invalid email address: {0}")]
    InvalidEmail(String),
    #[error("Unknown mail error: {0}")]
    Unknown(String),
}

#[async_trait]
pub trait MailSender: Send + Sync {
    async fn send(&self, _to: &str, _content: MailContent) -> Result<(), MailError>;
}

#[allow(dead_code)]
pub struct TestMailSender;
#[async_trait]
impl MailSender for TestMailSender {
    async fn send(&self, _to: &str, _content: MailContent) -> Result<(), MailError> {
        Ok(())
    }
}
