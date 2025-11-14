use lettre::{
    Message, SmtpTransport, Transport,
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
};
use std::env;
use anyhow::Result;

/// Email sending utilities
/// Equivalent to Go's email functionality

/// Send email
pub async fn send_email(
    to: &str,
    subject: &str,
    body: &str,
) -> Result<()> {
    let from_email = env::var("EMAIL_NAME")?;
    let from_password = env::var("EMAIL_PWD")?;

    // Parse email addresses
    let from: Mailbox = from_email.parse()?;
    let to: Mailbox = to.parse()?;

    // Build email
    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;

    // Setup SMTP transport
    let creds = Credentials::new(from_email, from_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    // Send email
    mailer.send(&email)?;

    tracing::info!("Email sent successfully to {}", to);

    Ok(())
}

/// Send HTML email
pub async fn send_html_email(
    to: &str,
    subject: &str,
    html_body: &str,
) -> Result<()> {
    let from_email = env::var("EMAIL_NAME")?;
    let from_password = env::var("EMAIL_PWD")?;

    let from: Mailbox = from_email.parse()?;
    let to: Mailbox = to.parse()?;

    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html_body.to_string())?;

    let creds = Credentials::new(from_email, from_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    tracing::info!("HTML email sent successfully to {}", to);

    Ok(())
}

/// Send password reset email
pub async fn send_password_reset_email(
    to: &str,
    reset_token: &str,
) -> Result<()> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8002".to_string());
    let reset_link = format!("{}/auth/reset-password?token={}", base_url, reset_token);

    let body = format!(
        r#"
        <html>
        <body>
            <h2>Reset Your Password</h2>
            <p>You have requested to reset your password. Click the link below to proceed:</p>
            <p><a href="{}">Reset Password</a></p>
            <p>This link will expire in 1 hour.</p>
            <p>If you did not request this, please ignore this email.</p>
        </body>
        </html>
        "#,
        reset_link
    );

    send_html_email(to, "Password Reset Request", &body).await
}

/// Send welcome email
pub async fn send_welcome_email(to: &str, name: &str) -> Result<()> {
    let body = format!(
        r#"
        <html>
        <body>
            <h2>Welcome to APSTH Clinic!</h2>
            <p>Dear {},</p>
            <p>Thank you for registering with us. We're excited to have you on board!</p>
            <p>If you have any questions, feel free to contact our support team.</p>
            <br>
            <p>Best regards,</p>
            <p>APSTH Team</p>
        </body>
        </html>
        "#,
        name
    );

    send_html_email(to, "Welcome to APSTH Clinic", &body).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Ignore by default to avoid sending real emails in tests
    async fn test_send_email() {
        std::env::set_var("EMAIL_NAME", "test@example.com");
        std::env::set_var("EMAIL_PWD", "test_password");

        // This would send a real email, so it's ignored
        // let result = send_email("recipient@example.com", "Test Subject", "Test Body").await;
        // assert!(result.is_ok());
    }
}
