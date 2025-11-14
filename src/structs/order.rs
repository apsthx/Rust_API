use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: i32,
    pub shop_id: i32,
    pub customer_id: i32,
    pub order_code: String,
    pub order_total: f64,
    pub order_discount: f64,
    pub order_net: f64,
    pub order_status: i8,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub customer_id: i32,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderItem {
    pub product_id: i32,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct OrderSearchRequest {
    pub customer_id: Option<i32>,
    pub status: Option<i8>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
