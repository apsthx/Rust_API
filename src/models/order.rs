use sqlx::{FromRow, MySql, Pool};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::NaiveDateTime;

/// Order database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i32,
    pub shop_id: i32,
    pub customer_id: i32,
    pub order_code: String,
    pub order_date: NaiveDateTime,
    pub order_total: f64,
    pub order_discount: f64,
    pub order_net: f64,
    pub order_status: i8,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Order model with database operations
pub struct OrderModel;

impl OrderModel {
    /// Get order by ID
    pub async fn get_order_by_id(
        db: &Pool<MySql>,
        order_id: i32,
        shop_id: i32,
    ) -> Result<Order> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            SELECT *
            FROM orders
            WHERE id = ? AND shop_id = ?
            "#,
        )
        .bind(order_id)
        .bind(shop_id)
        .fetch_one(db)
        .await?;

        Ok(order)
    }

    /// Search orders with filters
    pub async fn search_orders(
        db: &Pool<MySql>,
        shop_id: i32,
        customer_id: Option<i32>,
        status: Option<i8>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Order>> {
        let mut query = String::from(
            "SELECT * FROM orders WHERE shop_id = ?"
        );

        let mut bindings: Vec<Box<dyn sqlx::Encode<'_, MySql> + Send>> = vec![
            Box::new(shop_id)
        ];

        if let Some(cid) = customer_id {
            query.push_str(" AND customer_id = ?");
            bindings.push(Box::new(cid));
        }

        if let Some(s) = status {
            query.push_str(" AND order_status = ?");
            bindings.push(Box::new(s));
        }

        query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

        let orders = sqlx::query_as::<_, Order>(&query)
            .bind(shop_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(db)
            .await?;

        Ok(orders)
    }

    /// Create new order
    pub async fn create_order(
        db: &Pool<MySql>,
        shop_id: i32,
        customer_id: i32,
        order_code: &str,
        total: f64,
        discount: f64,
        net: f64,
    ) -> Result<i32> {
        let result = sqlx::query(
            r#"
            INSERT INTO orders
            (shop_id, customer_id, order_code, order_date, order_total, order_discount, order_net, order_status)
            VALUES (?, ?, ?, NOW(), ?, ?, ?, 1)
            "#,
        )
        .bind(shop_id)
        .bind(customer_id)
        .bind(order_code)
        .bind(total)
        .bind(discount)
        .bind(net)
        .execute(db)
        .await?;

        Ok(result.last_insert_id() as i32)
    }

    /// Update order
    pub async fn update_order(
        db: &Pool<MySql>,
        order_id: i32,
        total: f64,
        discount: f64,
        net: f64,
        status: i8,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE orders
            SET order_total = ?,
                order_discount = ?,
                order_net = ?,
                order_status = ?,
                updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(total)
        .bind(discount)
        .bind(net)
        .bind(status)
        .bind(order_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Delete order
    pub async fn delete_order(
        db: &Pool<MySql>,
        order_id: i32,
        shop_id: i32,
    ) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM orders
            WHERE id = ? AND shop_id = ?
            "#,
        )
        .bind(order_id)
        .bind(shop_id)
        .execute(db)
        .await?;

        Ok(())
    }
}
