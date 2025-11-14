use sqlx::{FromRow, MySql, Pool};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Shop {
    pub id: i32,
    pub shop_name: String,
    pub shop_address: Option<String>,
    pub shop_tel: Option<String>,
}

pub struct ShopModel;

impl ShopModel {
    pub async fn get_shop_by_id(
        db: &Pool<MySql>,
        shop_id: i32,
    ) -> Result<Shop> {
        let shop = sqlx::query_as::<_, Shop>(
            "SELECT * FROM shops WHERE id = ?",
        )
        .bind(shop_id)
        .fetch_one(db)
        .await?;
        Ok(shop)
    }
}
