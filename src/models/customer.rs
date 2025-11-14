use sqlx::{FromRow, MySql, Pool};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: i32,
    pub shop_id: i32,
    pub customer_fname: String,
    pub customer_lname: String,
    pub customer_tel: String,
    pub customer_email: Option<String>,
}

pub struct CustomerModel;

impl CustomerModel {
    pub async fn get_customer_by_id(
        db: &Pool<MySql>,
        customer_id: i32,
        shop_id: i32,
    ) -> Result<Customer> {
        let customer = sqlx::query_as::<_, Customer>(
            "SELECT * FROM customers WHERE id = ? AND shop_id = ?",
        )
        .bind(customer_id)
        .bind(shop_id)
        .fetch_one(db)
        .await?;
        Ok(customer)
    }
}
