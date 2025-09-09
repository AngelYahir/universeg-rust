use anyhow::{Context, Result};
use async_trait::async_trait;
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, MultiPart, SinglePart},
};

use std::sync::Arc;

use tracing::{debug, error, info, warn};

use crate::app::usecases::mail::send::{MailContent, MailError, MailSender};

#[derive(Clone)]
pub struct SmtpMailSender {
    from: Mailbox,
    transport: Arc<SmtpTransport>,
}

#[derive(Clone)]
pub struct SmtpMailerConfig {
    pub from_name: String,
    pub from_email: String,
    pub relay: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub pool_size: Option<u32>,
    pub start_tls: bool,
}

impl SmtpMailSender {
    pub fn new(config: SmtpMailerConfig) -> Result<Self> {
        debug!("SMTP: Initializing SMTP mail sender");
        debug!("SMTP: Relay server: {}", config.relay);
        debug!("SMTP: From name: {}", config.from_name);
        debug!("SMTP: From email: {}", config.from_email);
        debug!("SMTP: Start TLS: {}", config.start_tls);
        debug!(
            "SMTP: Has credentials: {}",
            config.username.is_some() && config.password.is_some()
        );

        let from = format!("{} <{}>", config.from_name, config.from_email)
            .parse()
            .context("Invalid email address")?;

        debug!("SMTP: Configuring transport for relay: {}", config.relay);
        let mut builder = if config.start_tls {
            debug!("SMTP: Using TLS transport on port 587");
            SmtpTransport::starttls_relay(&config.relay)
                .context("Invalid SMTP relay")?
                .port(587)
        } else {
            warn!("SMTP: Using unencrypted transport on port 25 - not recommended for production");
            SmtpTransport::builder_dangerous(&config.relay).port(25)
        };

        if let (Some(u), Some(p)) = (config.username.as_ref(), config.password.as_ref()) {
            debug!("SMTP: Adding authentication credentials for user: {}", u);
            builder = builder.credentials(
                lettre::transport::smtp::authentication::Credentials::new(u.clone(), p.clone()),
            );
        } else {
            warn!("SMTP: No authentication credentials provided - this may cause sending failures");
        }

        let mut pool_config = lettre::transport::smtp::PoolConfig::new();
        if let Some(n) = config.pool_size {
            debug!("SMTP: Setting connection pool size to: {}", n);
            pool_config = pool_config.max_size(n);
        }

        debug!("SMTP: Building transport with connection timeout");
        let transport = builder
            .pool_config(pool_config)
            .timeout(Some(std::time::Duration::from_secs(10)))
            .build();

        debug!("SMTP: SMTP mail sender configured successfully");
        Ok(Self {
            from,
            transport: Arc::new(transport),
        })
    }
}

#[async_trait]
impl MailSender for SmtpMailSender {
    async fn send(&self, to: &str, content: MailContent) -> Result<(), MailError> {
        debug!("SMTP: Starting email send process to: {}", to);
        debug!("SMTP: Email subject: {}", content.subject);

        let recipient_str = to.to_string();
        let to_mailbox: Mailbox = to.parse().map_err(|e| {
            error!(
                "SMTP: Failed to parse recipient email '{}': {:?}",
                recipient_str, e
            );
            MailError::InvalidEmail(recipient_str.clone())
        })?;

        debug!("SMTP: Recipient email parsed successfully: {}", to_mailbox);
        debug!("SMTP: Building email message from: {}", self.from);

        let builder = Message::builder()
            .from(self.from.clone())
            .to(to_mailbox)
            .subject(content.subject)
            .date_now();

        debug!("SMTP: Creating email body with HTML and text content");

        let email: Message = if let Some(text) = content.text {
            builder
                .multipart(
                    MultiPart::alternative()
                        .singlepart(
                            SinglePart::builder()
                                .header(lettre::message::header::ContentType::TEXT_PLAIN)
                                .body(text),
                        )
                        .singlepart(
                            SinglePart::builder()
                                .header(lettre::message::header::ContentType::TEXT_HTML)
                                .body(content.html),
                        ),
                )
                .map_err(|e| {
                    error!("SMTP: Failed to create multipart email: {:?}", e);
                    MailError::SendError(anyhow::anyhow!(e).to_string())
                })?
        } else {
            builder
                .singlepart(
                    SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_HTML)
                        .body(content.html),
                )
                .map_err(|e| {
                    error!("SMTP: Failed to create HTML email body: {:?}", e);
                    MailError::Unknown(anyhow::anyhow!(e).to_string())
                })?
        };

        debug!("SMTP: Email message constructed successfully, sending via transport");

        // Debug: Show email content for debugging
        let email_bytes = email.formatted();
        match String::from_utf8(email_bytes.clone()) {
            Ok(email_content) => {
                debug!("SMTP: Raw email message:\n{}", email_content);
            }
            Err(_) => {
                debug!(
                    "SMTP: Raw email message (binary data, {} bytes): {:?}",
                    email_bytes.len(),
                    &email_bytes[..std::cmp::min(100, email_bytes.len())]
                );
            }
        }

        debug!("SMTP: Attempting to connect to SMTP server...");

        match self.transport.send(&email) {
            Ok(response) => {
                info!("SMTP: Email sent successfully to: {}", recipient_str);
                debug!("SMTP: Server response: {:?}", response);
                Ok(())
            }
            Err(e) => {
                error!("SMTP: Failed to send email to '{}': {:?}", recipient_str, e);
                error!("SMTP: Transport error details: {}", e);
                Err(MailError::Unknown(anyhow::anyhow!(e).to_string()))
            }
        }
    }
}
