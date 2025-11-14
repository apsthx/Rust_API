use serde::{Deserialize, Serialize};
use validator::Validate;

/// Login request payload
/// Equivalent to Go's PayloadLogin in structs/auth.go
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub username: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,

    pub otp_code: Option<String>,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub access_token: String,
    pub refresh_token: String,
    pub shops: Vec<ShopAccount>,
}

/// Shop account information
#[derive(Debug, Serialize, Deserialize)]
pub struct ShopAccount {
    pub shop_id: i32,
    pub shop_name: String,
    pub shop_role_id: i32,
    pub role_id: i32,
}

/// Refresh token request
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Token response
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

/// Register request
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,

    #[validate(length(min = 1, message = "First name is required"))]
    pub fname: String,

    #[validate(length(min = 1, message = "Last name is required"))]
    pub lname: String,

    #[validate(length(min = 10, max = 10, message = "Phone number must be 10 digits"))]
    pub tel: String,
}

/// Change password request
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub old_password: String,

    #[validate(length(min = 6, message = "New password must be at least 6 characters"))]
    pub new_password: String,
}

/// Forgot password request
#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

/// Reset password request
#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    pub token: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub new_password: String,
}

/// OTP setup request
#[derive(Debug, Deserialize)]
pub struct OtpSetupRequest {
    pub user_id: i32,
}

/// OTP verify request
#[derive(Debug, Deserialize)]
pub struct OtpVerifyRequest {
    pub user_id: i32,
    pub otp_code: String,
}
