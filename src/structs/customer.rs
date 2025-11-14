use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerResponse {
    pub id: i32,
    pub shop_id: i32,
    pub fname: String,
    pub lname: String,
    pub tel: String,
    pub email: Option<String>,
}
