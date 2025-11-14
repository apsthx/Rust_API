use sqlx::{MySql, Pool, MySqlPool};
use std::env;
use anyhow::Result;

/// Application state containing all database connections
/// Equivalent to Go's configs/database.go with DB1, DB2, DBL1, DBL2
#[derive(Clone)]
pub struct AppState {
    pub db1: Pool<MySql>,   // Main write database
    pub db2: Pool<MySql>,   // Main read replica
    pub dbl1: Pool<MySql>,  // Logging write database
    pub dbl2: Pool<MySql>,  // Logging read replica
}

/// Database configuration structure
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    /// Create database URL string for MySQL connection
    fn to_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}?charset=utf8mb4&parseTime=true",
            self.user, self.password, self.host, self.port, self.name
        )
    }

    /// Load configuration from environment variables
    pub fn from_env(prefix: &str) -> Result<Self> {
        Ok(Self {
            host: env::var(format!("{}_HOST", prefix))?,
            port: env::var(format!("{}_PORT", prefix))?.parse()?,
            user: env::var(format!("{}_USER", prefix))?,
            password: env::var(format!("{}_PWD", prefix))?,
            name: env::var(format!("{}_NAME", prefix))?,
        })
    }
}

/// Initialize all database connections
/// Equivalent to Go's ConnectDatabase() function
pub async fn init_databases() -> Result<AppState> {
    tracing::info!("Initializing database connections...");

    // Load configurations from environment
    let db1_config = DatabaseConfig::from_env("DB")?;
    let db2_config = DatabaseConfig::from_env("DBR")?;
    let dbl1_config = DatabaseConfig::from_env("DBL")?;
    let dbl2_config = DatabaseConfig::from_env("DBLR")?;

    // Create connection pools
    let db1 = create_pool(&db1_config, "DB1 (Main Write)").await?;
    let db2 = create_pool(&db2_config, "DB2 (Main Read)").await?;
    let dbl1 = create_pool(&dbl1_config, "DBL1 (Log Write)").await?;
    let dbl2 = create_pool(&dbl2_config, "DBL2 (Log Read)").await?;

    tracing::info!("All database connections established successfully");

    Ok(AppState {
        db1,
        db2,
        dbl1,
        dbl2,
    })
}

/// Create a MySQL connection pool with proper configuration
async fn create_pool(config: &DatabaseConfig, name: &str) -> Result<Pool<MySql>> {
    tracing::info!("Connecting to {}: {}:{}", name, config.host, config.port);

    let pool = MySqlPool::connect(&config.to_url()).await?;

    tracing::info!("Successfully connected to {}", name);

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_url_format() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "password".to_string(),
            name: "clinic".to_string(),
        };

        let url = config.to_url();
        assert!(url.contains("mysql://"));
        assert!(url.contains("localhost:3306"));
        assert!(url.contains("charset=utf8mb4"));
    }
}
