use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::time::Duration;

// 定义 DBPool 类型别名，方便并在项目中使用
pub type DBPool = Pool<MySql>;

pub async fn init_db_pool() -> DBPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    MySqlPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}