use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::time::Duration;

/// 创建数据库连接池 (MySQL)
pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}