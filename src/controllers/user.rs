use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use crate::configs::AppState;
use crate::structs::{UserResponse, ApiResponse, UpdateUserRequest};
use crate::models::UserModel;
use crate::middlewares::AuthUser;

/// Get user by ID
/// Equivalent to Go's UserDetail function
pub async fn get_user_detail(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(user_id): Path<i32>,
) -> Result<Json<ApiResponse<UserResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let user = UserModel::get_user_by_id(&state.db2, user_id, auth.shop_id)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("User not found".to_string())),
            )
        })?;

    let response = UserResponse {
        id: user.id,
        email: user.user_email,
        fname: user.user_fname,
        lname: user.user_lname,
        tel: user.user_tel,
        shop_id: user.shop_id,
        shop_name: user.shop_name,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Get current user info
pub async fn get_current_user(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ApiResponse<UserResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let user = UserModel::get_user_by_id(&state.db2, auth.user_id, auth.shop_id)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("User not found".to_string())),
            )
        })?;

    let response = UserResponse {
        id: user.id,
        email: user.user_email,
        fname: user.user_fname,
        lname: user.user_lname,
        tel: user.user_tel,
        shop_id: user.shop_id,
        shop_name: user.shop_name,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Update user
/// Equivalent to Go's UpdateUser function
pub async fn update_user(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    UserModel::update_user(
        &state.db1,
        auth.user_id,
        &payload.fname,
        &payload.lname,
        &payload.tel,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Update failed: {}", e))),
        )
    })?;

    Ok(Json(ApiResponse::success_with_message(
        (),
        "User updated successfully".to_string(),
    )))
}

/// Get all users in shop
pub async fn get_shop_users(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<UserResponse>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let users = UserModel::get_users_by_shop(&state.db2, auth.shop_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Failed to fetch users: {}", e))),
            )
        })?;

    let response: Vec<UserResponse> = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            email: u.user_email,
            fname: u.user_fname,
            lname: u.user_lname,
            tel: u.user_tel,
            shop_id: u.shop_id,
            shop_name: u.shop_name,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}
