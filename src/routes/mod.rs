// Routes module - API endpoint definitions
// Equivalent to Go's routes/ directory

use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use crate::configs::AppState;
use crate::controllers;
use crate::middlewares;

/// Create main router with all routes
/// Equivalent to Go's main.go router setup
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))

        // Auth routes (public)
        .nest("/auth", auth_routes())

        // User routes (protected)
        .nest("/user", user_routes())

        // Order routes (protected)
        .nest("/order", order_routes())

        // Add state
        .with_state(state)
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Auth routes
/// Equivalent to Go's SetRouterAuth
fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(controllers::login))
        .route("/logout", post(controllers::logout))
        .route(
            "/verify",
            get(controllers::verify_token)
                .layer(middleware::from_fn(middlewares::check_access_token))
        )
}

/// User routes
/// Equivalent to Go's SetRouterUser
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(controllers::get_current_user))
        .route("/:id", get(controllers::get_user_detail))
        .route("/", put(controllers::update_user))
        .route("/list", get(controllers::get_shop_users))
        .layer(middleware::from_fn(middlewares::check_access_token))
}

/// Order routes
/// Equivalent to Go's SetRouterOrders
fn order_routes() -> Router<AppState> {
    Router::new()
        .route("/search", post(controllers::search_orders))
        .route("/:id", get(controllers::get_order_detail))
        .route("/", post(controllers::create_order))
        .route("/:id", delete(controllers::delete_order))
        .layer(middleware::from_fn(middlewares::check_access_token))
}
