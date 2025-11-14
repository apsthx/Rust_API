use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    Json,
};
use crate::configs::AppState;
use crate::structs::{OrderResponse, ApiResponse, CreateOrderRequest, OrderSearchRequest};
use crate::models::OrderModel;
use crate::middlewares::AuthUser;

/// Search orders
/// Equivalent to Go's OrdersSearch function
pub async fn search_orders(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<OrderSearchRequest>,
) -> Result<Json<ApiResponse<Vec<OrderResponse>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    let orders = OrderModel::search_orders(
        &state.db2,
        auth.shop_id,
        params.customer_id,
        params.status,
        limit,
        offset,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Search failed: {}", e))),
        )
    })?;

    let response: Vec<OrderResponse> = orders
        .into_iter()
        .map(|o| OrderResponse {
            id: o.id,
            shop_id: o.shop_id,
            customer_id: o.customer_id,
            order_code: o.order_code,
            order_total: o.order_total,
            order_discount: o.order_discount,
            order_net: o.order_net,
            order_status: o.order_status,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// Get order detail
/// Equivalent to Go's OrdersDetail function
pub async fn get_order_detail(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<i32>,
) -> Result<Json<ApiResponse<OrderResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let order = OrderModel::get_order_by_id(&state.db2, order_id, auth.shop_id)
        .await
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("Order not found".to_string())),
            )
        })?;

    let response = OrderResponse {
        id: order.id,
        shop_id: order.shop_id,
        customer_id: order.customer_id,
        order_code: order.order_code,
        order_total: order.order_total,
        order_discount: order.order_discount,
        order_net: order.order_net,
        order_status: order.order_status,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Create new order
/// Equivalent to Go's AddOrder function
pub async fn create_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<ApiResponse<OrderResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    // Generate order code
    let order_code = format!("ORD-{}-{}", auth.shop_id, chrono::Utc::now().timestamp());

    // Calculate totals
    let total: f64 = payload.items.iter().map(|item| item.price * item.quantity as f64).sum();
    let discount = 0.0; // Apply discount logic here
    let net = total - discount;

    // Create order
    let order_id = OrderModel::create_order(
        &state.db1,
        auth.shop_id,
        payload.customer_id,
        &order_code,
        total,
        discount,
        net,
    )
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(format!("Order creation failed: {}", e))),
        )
    })?;

    // TODO: Create order items in order_items table

    let response = OrderResponse {
        id: order_id,
        shop_id: auth.shop_id,
        customer_id: payload.customer_id,
        order_code,
        order_total: total,
        order_discount: discount,
        order_net: net,
        order_status: 1,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Delete order
/// Equivalent to Go's DelOrder function
pub async fn delete_order(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(order_id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    OrderModel::delete_order(&state.db1, order_id, auth.shop_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(format!("Delete failed: {}", e))),
            )
        })?;

    Ok(Json(ApiResponse::success_with_message(
        (),
        "Order deleted successfully".to_string(),
    )))
}
