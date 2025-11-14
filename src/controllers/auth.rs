use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use crate::configs::AppState;
use crate::structs::{LoginRequest, LoginResponse, ApiResponse, ShopAccount};
use crate::models::UserModel;
use crate::middlewares::{hash_password, verify_password, create_access_token, create_refresh_token};
use validator::Validate;

/// Login handler
/// Equivalent to Go's Login function in controllers/auth.go
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Validate request
    if let Err(errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(format!("Validation error: {}", errors))),
        ));
    }

    // Hash password for comparison
    let password_hash = hash_password(&payload.password)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Password hashing failed: {}", e))),
            )
        })?;

    // Get user from database
    let user = UserModel::get_user_for_login(&state.db1, &payload.username, &password_hash)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error("Invalid username or password".to_string())),
            )
        })?;

    // Check if user is active
    if user.user_is_active == 0 {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error("User account is deactivated".to_string())),
        ));
    }

    // Check 2FA/OTP if enabled
    if let Some(otp_url) = &user.user_otp_url {
        if !otp_url.is_empty() {
            if let Some(otp_code) = &payload.otp_code {
                // TODO: Verify OTP code using totp-rs
                // For now, just placeholder
                tracing::info!("OTP verification needed for user {}", user.id);
            } else {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::error("OTP code required".to_string())),
                ));
            }
        }
    }

    // Get shop accounts for user
    // For now, return empty vec - in real implementation, query user_shops table
    let shops: Vec<ShopAccount> = vec![];

    // Generate tokens
    let access_token = create_access_token(
        user.id,
        1, // Default shop_id, should come from first shop or selected shop
        1, // shop_mother_id
        1, // role_id
        1, // shop_role_id
        user.user_email.clone(),
        0, // sr_discount_type_id
        0.0, // sr_discount
        user.password_version,
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Token generation failed: {}", e))),
        )
    })?;

    let refresh_token = create_refresh_token(
        user.id,
        1,
        1,
        1,
        1,
        user.user_email.clone(),
        0,
        0.0,
        user.password_version,
        1, // user_type
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Token generation failed: {}", e))),
        )
    })?;

    // Prepare response
    let response = LoginResponse {
        user_id: user.id,
        email: user.user_email.clone(),
        fname: user.user_fname,
        lname: user.user_lname,
        access_token,
        refresh_token,
        shops,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Logout handler
pub async fn logout() -> Json<ApiResponse<()>> {
    // In JWT-based auth, logout is typically handled client-side
    // Server can maintain a blacklist if needed
    Json(ApiResponse::success_with_message(
        (),
        "Logged out successfully".to_string(),
    ))
}

/// Verify token handler
pub async fn verify_token() -> Json<ApiResponse<()>> {
    // This handler is called after JWT middleware validation
    // If we reach here, token is valid
    Json(ApiResponse::success_with_message(
        (),
        "Token is valid".to_string(),
    ))
}
