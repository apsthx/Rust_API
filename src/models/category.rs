use sqlx::{FromRow, MySql, Pool};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub shop_id: i32,
    pub category_type_id: i32,
    pub category_name: String,
}

pub struct CategoryModel;

impl CategoryModel {
    pub async fn get_categories_by_shop(
        db: &Pool<MySql>,
        shop_id: i32,
    ) -> Result<Vec<Category>> {
        let categories = sqlx::query_as::<_, Category>(
            "SELECT * FROM categories WHERE shop_id = ?",
        )
        .bind(shop_id)
        .fetch_all(db)
        .await?;
        Ok(categories)
    }
}
