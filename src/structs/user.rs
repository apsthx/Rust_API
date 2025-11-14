use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub fname: String,
    pub lname: String,
    pub tel: String,
    pub shop_id: i32,
    pub shop_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub fname: String,
    pub lname: String,
    pub tel: String,
}
