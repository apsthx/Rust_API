use anyhow::Result;
use std::env;
use serde::{Deserialize, Serialize};

/// SMS sending service
/// Equivalent to Go's libs/sms.go (Thai Bulk SMS integration)

#[derive(Debug, Serialize)]
struct SmsRequest {
    msisdn: String,
    message: String,
    sender: String,
    force: String,
}

#[derive(Debug, Deserialize)]
struct SmsResponse {
    status: String,
    credit: Option<f64>,
}

/// Send SMS using Thai Bulk SMS API
/// Equivalent to Go's SendSMS function
pub async fn send_sms(phone: &str, message: &str) -> Result<()> {
    let api_key = env::var("SMS_API_KEY")?;
    let api_secret = env::var("SMS_API_SECRET_KEY")?;
    let sender = env::var("SMS_SENDER").unwrap_or_else(|_| "APSTH".to_string());

    // Prepare request
    let request = SmsRequest {
        msisdn: phone.to_string(),
        message: message.to_string(),
        sender,
        force: "corporate".to_string(),
    };

    // Create basic auth
    let auth = base64::encode(format!("{}:{}", api_key, api_secret));

    // Send request
    let client = reqwest::Client::new();
    let response = client
        .post("https://portal-otp.smsmkt.com/api/send-message")
        .header("Authorization", format!("Basic {}", auth))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("SMS sending failed: {}", response.status());
    }

    let result: SmsResponse = response.json().await?;
    tracing::info!("SMS sent successfully. Status: {}, Credit: {:?}", result.status, result.credit);

    Ok(())
}

/// Send OTP via SMS
pub async fn send_otp_sms(phone: &str, otp_code: &str) -> Result<()> {
    let message = format!("Your OTP code is: {}. Valid for 5 minutes.", otp_code);
    send_sms(phone, &message).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Ignore by default to avoid sending real SMS in tests
    async fn test_send_sms() {
        std::env::set_var("SMS_API_KEY", "test_key");
        std::env::set_var("SMS_API_SECRET_KEY", "test_secret");
        std::env::set_var("SMS_SENDER", "TEST");

        // This would send a real SMS, so it's ignored
        // let result = send_sms("0812345678", "Test message").await;
        // assert!(result.is_ok());
    }
}
