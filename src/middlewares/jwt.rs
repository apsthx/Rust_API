use axum::{
    extract::{Request, FromRequestParts},
    middleware::Next,
    response::{Response, IntoResponse},
    http::{StatusCode, HeaderMap, header},
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};
use anyhow::Result;

/// Access Token Claims structure
/// Equivalent to Go's AccessTokenClaims in middlewares/jwt.go
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenClaims {
    pub user_id: i32,
    pub shop_id: i32,
    pub shop_mother_id: i32,
    pub role_id: i32,
    pub shop_role_id: i32,
    pub user_email: String,
    pub sr_discount_type_id: i32,
    pub sr_discount: f32,
    pub password_version: i32,
    pub exp: i64,
    pub iat: i64,
}

/// Refresh Token Claims structure
/// Equivalent to Go's RefreshTokenClaims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
    pub user_id: i32,
    pub shop_id: i32,
    pub shop_mother_id: i32,
    pub role_id: i32,
    pub shop_role_id: i32,
    pub user_email: String,
    pub sr_discount_type_id: i32,
    pub sr_discount: f32,
    pub password_version: i32,
    pub user_type: i32,
    pub exp: i64,
    pub iat: i64,
}

/// Authenticated user information extracted from JWT
/// Used in request handlers as an extractor
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i32,
    pub shop_id: i32,
    pub shop_mother_id: i32,
    pub role_id: i32,
    pub shop_role_id: i32,
    pub user_email: String,
    pub sr_discount_type_id: i32,
    pub sr_discount: f32,
    pub password_version: i32,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthUser>()
            .cloned()
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Unauthorized: No auth user found".to_string(),
            ))
    }
}

/// Create Access Token (short-lived, default 90 minutes)
/// Equivalent to Go's CreateAccessToken function
pub fn create_access_token(
    user_id: i32,
    shop_id: i32,
    shop_mother_id: i32,
    role_id: i32,
    shop_role_id: i32,
    user_email: String,
    sr_discount_type_id: i32,
    sr_discount: f32,
    password_version: i32,
) -> Result<String> {
    let expiration_minutes = env::var("JWT_AC_EXPIRE")
        .unwrap_or_else(|_| "90".to_string())
        .parse::<i64>()
        .unwrap_or(90);

    let now = Utc::now();
    let exp = (now + Duration::minutes(expiration_minutes)).timestamp();

    let claims = AccessTokenClaims {
        user_id,
        shop_id,
        shop_mother_id,
        role_id,
        shop_role_id,
        user_email,
        sr_discount_type_id,
        sr_discount,
        password_version,
        exp,
        iat: now.timestamp(),
    };

    let secret = env::var("JWT_AC_KEY").expect("JWT_AC_KEY must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Create Refresh Token (long-lived, default 720 hours)
/// Equivalent to Go's CreateRefreshToken function
pub fn create_refresh_token(
    user_id: i32,
    shop_id: i32,
    shop_mother_id: i32,
    role_id: i32,
    shop_role_id: i32,
    user_email: String,
    sr_discount_type_id: i32,
    sr_discount: f32,
    password_version: i32,
    user_type: i32,
) -> Result<String> {
    let expiration_hours = env::var("JWT_RF_EXPIRE")
        .unwrap_or_else(|_| "720".to_string())
        .parse::<i64>()
        .unwrap_or(720);

    let now = Utc::now();
    let exp = (now + Duration::hours(expiration_hours)).timestamp();

    let claims = RefreshTokenClaims {
        user_id,
        shop_id,
        shop_mother_id,
        role_id,
        shop_role_id,
        user_email,
        sr_discount_type_id,
        sr_discount,
        password_version,
        user_type,
        exp,
        iat: now.timestamp(),
    };

    let secret = env::var("JWT_RF_KEY").expect("JWT_RF_KEY must be set");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Middleware to check access token validity
/// Equivalent to Go's CheckAccessToken middleware
pub async fn check_access_token(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Extract token from Authorization header
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header".to_string(),
            )
        })?;

    // Decode and validate token
    let secret = env::var("JWT_AC_KEY").expect("JWT_AC_KEY must be set");
    let token_data = decode::<AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid or expired token".to_string(),
        )
    })?;

    // Check token expiration
    let now = Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err((StatusCode::UNAUTHORIZED, "Token expired".to_string()));
    }

    // Store user info in request extensions
    let auth_user = AuthUser {
        user_id: token_data.claims.user_id,
        shop_id: token_data.claims.shop_id,
        shop_mother_id: token_data.claims.shop_mother_id,
        role_id: token_data.claims.role_id,
        shop_role_id: token_data.claims.shop_role_id,
        user_email: token_data.claims.user_email.clone(),
        sr_discount_type_id: token_data.claims.sr_discount_type_id,
        sr_discount: token_data.claims.sr_discount,
        password_version: token_data.claims.password_version,
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

/// Middleware to check refresh token validity
/// Equivalent to Go's CheckRefreshToken middleware
pub async fn check_refresh_token(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header".to_string(),
            )
        })?;

    let secret = env::var("JWT_RF_KEY").expect("JWT_RF_KEY must be set");
    let token_data = decode::<RefreshTokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid or expired refresh token".to_string(),
        )
    })?;

    let now = Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err((StatusCode::UNAUTHORIZED, "Refresh token expired".to_string()));
    }

    request.extensions_mut().insert(token_data.claims);

    Ok(next.run(request).await)
}

/// Middleware to check public API key
/// Equivalent to Go's CheckPublicKey middleware
pub async fn check_public_key(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let public_key = headers
        .get("X-API-Key")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing API key".to_string(),
            )
        })?;

    let expected_key = env::var("TK_PUBLIC_KEY").expect("TK_PUBLIC_KEY must be set");

    if public_key != expected_key {
        return Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string()));
    }

    Ok(next.run(request).await)
}

/// Middleware to check telemedicine public key
/// Equivalent to Go's CheckTelePublicKey middleware
pub async fn check_tele_public_key(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let public_key = headers
        .get("X-API-Key")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Missing telemedicine API key".to_string(),
            )
        })?;

    let expected_key = env::var("TK_TELE_PUBLIC_KEY")
        .expect("TK_TELE_PUBLIC_KEY must be set");

    if public_key != expected_key {
        return Err((StatusCode::UNAUTHORIZED, "Invalid telemedicine API key".to_string()));
    }

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_access_token() {
        std::env::set_var("JWT_AC_KEY", "test_secret_key");
        std::env::set_var("JWT_AC_EXPIRE", "90");

        let token = create_access_token(
            1,
            1,
            1,
            1,
            1,
            "test@example.com".to_string(),
            0,
            0.0,
            1,
        );

        assert!(token.is_ok());
    }
}
