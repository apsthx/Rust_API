use sqlx::{FromRow, MySql, Pool};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub shop_id: i32,
    pub product_name: String,
    pub product_price: f64,
    pub product_stock: i32,
}

pub struct ProductModel;

impl ProductModel {
    pub async fn get_product_by_id(
        db: &Pool<MySql>,
        product_id: i32,
        shop_id: i32,
    ) -> Result<Product> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE id = ? AND shop_id = ?",
        )
        .bind(product_id)
        .bind(shop_id)
        .fetch_one(db)
        .await?;
        Ok(product)
    }
}
