use sqlx::mysql::MySqlPool;
use dotenv::dotenv;
use std::env;

pub type DbPool = MySqlPool;

/// 初始化数据库连接池
pub async fn init_db_pool() -> Result<DbPool, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    MySqlPool::connect(&database_url).await
}

/// 获取数据库连接池（用于依赖注入）
pub async fn get_db_pool() -> DbPool {
    // 这里可以使用once_cell或lazy_static来缓存连接池
    // 暂时每次调用都创建新连接，实际生产环境应该缓存
    init_db_pool().await.expect("Failed to create database pool")
}
