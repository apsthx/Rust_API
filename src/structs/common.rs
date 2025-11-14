use serde::{Deserialize, Serialize};

/// Standard API response wrapper
/// Equivalent to Go's response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: true,
            data: Some(data),
            message: None,
            error: None,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            status: true,
            data: Some(data),
            message: Some(message),
            error: None,
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn error(error: String) -> Self {
        Self {
            status: false,
            data: None,
            message: None,
            error: Some(error),
        }
    }
}

/// Pagination request
#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
        }
    }
}

impl PaginationRequest {
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(20);
        (page - 1) * limit
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20)
    }
}

/// Pagination response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: i64, limit: i64, total: i64) -> Self {
        let total_pages = (total as f64 / limit as f64).ceil() as i64;
        Self {
            data,
            page,
            limit,
            total,
            total_pages,
        }
    }
}
