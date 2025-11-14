use sqlx::{FromRow, MySql, Pool};
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// User database model
/// Equivalent to Go's User struct in models/user.go
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub user_email: String,
    pub user_password: String,
    pub user_fname: String,
    pub user_lname: String,
    pub user_tel: String,
    pub user_is_active: i8,
    pub user_otp_url: Option<String>,
    pub password_version: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// User with shop information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserWithShop {
    pub id: i32,
    pub user_email: String,
    pub user_fname: String,
    pub user_lname: String,
    pub user_tel: String,
    pub shop_id: i32,
    pub shop_name: String,
    pub shop_role_id: i32,
    pub shop_role_name: String,
    pub role_id: i32,
}

/// User model with database operations
pub struct UserModel;

impl UserModel {
    /// Get user by ID with shop information
    /// Equivalent to Go's GetUserById function
    pub async fn get_user_by_id(
        db: &Pool<MySql>,
        user_id: i32,
        shop_id: i32,
    ) -> Result<UserWithShop> {
        let user = sqlx::query_as::<_, UserWithShop>(
            r#"
            SELECT
                users.id,
                users.user_email,
                users.user_fname,
                users.user_lname,
                users.user_tel,
                user_shops.shop_id,
                shops.shop_name,
                user_shops.shop_role_id,
                shop_roles.shop_role_name,
                shop_roles.role_id
            FROM users
            JOIN user_shops ON user_shops.user_id = users.id
            JOIN shops ON user_shops.shop_id = shops.id
            JOIN shop_roles ON user_shops.shop_role_id = shop_roles.id
            WHERE user_shops.us_invite = 2
                AND users.id = ?
                AND user_shops.shop_id = ?
                AND users.user_is_active = 1
            "#,
        )
        .bind(user_id)
        .bind(shop_id)
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    /// Get user for login by username (email) and password hash
    /// Equivalent to Go's GetUserForLogin function
    pub async fn get_user_for_login(
        db: &Pool<MySql>,
        username: &str,
        password_hash: &str,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE user_email = ?
                AND user_password = ?
                AND user_is_active = 1
            "#,
        )
        .bind(username)
        .bind(password_hash)
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    /// Get user by email
    pub async fn get_user_by_email(
        db: &Pool<MySql>,
        email: &str,
    ) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE user_email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    /// Create new user
    /// Equivalent to Go's AddUser function
    pub async fn create_user(
        db: &Pool<MySql>,
        email: &str,
        password_hash: &str,
        fname: &str,
        lname: &str,
        tel: &str,
    ) -> Result<i32> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (user_email, user_password, user_fname, user_lname, user_tel, user_is_active, password_version)
            VALUES (?, ?, ?, ?, ?, 1, 1)
            "#,
        )
        .bind(email)
        .bind(password_hash)
        .bind(fname)
        .bind(lname)
        .bind(tel)
        .execute(db)
        .await?;

        Ok(result.last_insert_id() as i32)
    }

    /// Update user information
    /// Equivalent to Go's UpdateUser function
    pub async fn update_user(
        db: &Pool<MySql>,
        user_id: i32,
        fname: &str,
        lname: &str,
        tel: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET user_fname = ?, user_lname = ?, user_tel = ?, updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(fname)
        .bind(lname)
        .bind(tel)
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Update user password and increment password_version
    /// Password version is used to invalidate existing JWT tokens
    pub async fn update_password(
        db: &Pool<MySql>,
        user_id: i32,
        new_password_hash: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET user_password = ?,
                password_version = password_version + 1,
                updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(new_password_hash)
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Deactivate user
    pub async fn deactivate_user(
        db: &Pool<MySql>,
        user_id: i32,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET user_is_active = 0, updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(())
    }

    /// Get all users for a shop
    pub async fn get_users_by_shop(
        db: &Pool<MySql>,
        shop_id: i32,
    ) -> Result<Vec<UserWithShop>> {
        let users = sqlx::query_as::<_, UserWithShop>(
            r#"
            SELECT
                users.id,
                users.user_email,
                users.user_fname,
                users.user_lname,
                users.user_tel,
                user_shops.shop_id,
                shops.shop_name,
                user_shops.shop_role_id,
                shop_roles.shop_role_name,
                shop_roles.role_id
            FROM users
            JOIN user_shops ON user_shops.user_id = users.id
            JOIN shops ON user_shops.shop_id = shops.id
            JOIN shop_roles ON user_shops.shop_role_id = shop_roles.id
            WHERE user_shops.shop_id = ?
                AND user_shops.us_invite = 2
                AND users.user_is_active = 1
            ORDER BY users.user_fname ASC
            "#,
        )
        .bind(shop_id)
        .fetch_all(db)
        .await?;

        Ok(users)
    }

    /// Update user OTP URL for 2FA
    pub async fn update_otp_url(
        db: &Pool<MySql>,
        user_id: i32,
        otp_url: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET user_otp_url = ?, updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(otp_url)
        .bind(user_id)
        .execute(db)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_serialization() {
        let user = User {
            id: 1,
            user_email: "test@example.com".to_string(),
            user_password: "hashed_password".to_string(),
            user_fname: "John".to_string(),
            user_lname: "Doe".to_string(),
            user_tel: "0812345678".to_string(),
            user_is_active: 1,
            user_otp_url: None,
            password_version: 1,
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("test@example.com"));
    }
}
