use async_trait::async_trait;
use std::sync::Arc;

use crate::app::ports::{AppError, EmailService};
use crate::app::usecases::mail::send::{MailContent, MailSender};
use crate::domain::user::Email;

pub struct SmtpEmailService {
    mail_sender: Arc<dyn MailSender>,
    base_url: String,
}

impl SmtpEmailService {
    pub fn new(mail_sender: Arc<dyn MailSender>, base_url: String) -> Self {
        Self {
            mail_sender,
            base_url,
        }
    }
}

#[async_trait]
impl EmailService for SmtpEmailService {
    async fn send_verification_email(&self, to: &Email, token: &str) -> Result<(), AppError> {
        let verification_url = format!("{}/auth/verify-email?token={}", self.base_url, token);

        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <title>Email Verification</title>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #4CAF50; color: white; padding: 20px; text-align: center; }}
                    .content {{ background-color: #f9f9f9; padding: 30px; }}
                    .button {{
                        display: inline-block;
                        background-color: #4CAF50;
                        color: white;
                        padding: 12px 24px;
                        text-decoration: none;
                        border-radius: 4px;
                        margin: 20px 0;
                    }}
                    .footer {{ text-align: center; color: #666; font-size: 12px; margin-top: 20px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Welcome! Please verify your email</h1>
                    </div>
                    <div class="content">
                        <p>Thank you for registering with us! To complete your registration and activate your account, please verify your email address by clicking the button below:</p>

                        <div style="text-align: center;">
                            <a href="{}" class="button">Verify Email Address</a>
                        </div>

                        <p>If the button doesn't work, you can copy and paste this link into your browser:</p>
                        <p style="word-break: break-all; color: #666;">{}</p>

                        <p><strong>Important:</strong> This verification link will expire in 24 hours.</p>

                        <p>If you didn't create an account with us, you can safely ignore this email.</p>
                    </div>
                    <div class="footer">
                        <p>This is an automated message, please do not reply to this email.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            verification_url, verification_url
        );

        let mail_content = MailContent {
            subject: "Verify your email address".to_string(),
            html: html_content,
            text: Some(format!(
                "Welcome! Please verify your email\n\n\
                Thank you for registering with us! To complete your registration, please visit this link:\n\n\
                {}\n\n\
                This link will expire in 24 hours.\n\n\
                If you didn't create an account with us, you can safely ignore this email.",
                verification_url
            )),
        };

        self.mail_sender
            .send(to.as_str(), mail_content)
            .await
            .map_err(|e| AppError::Infra(anyhow::anyhow!("Failed to send email: {}", e)))?;

        Ok(())
    }
}

#[async_trait]
impl EmailService for Arc<SmtpEmailService> {
    async fn send_verification_email(&self, to: &Email, token: &str) -> Result<(), AppError> {
        self.as_ref().send_verification_email(to, token).await
    }
}
