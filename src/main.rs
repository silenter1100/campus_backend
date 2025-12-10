// 声明一级模块
pub mod common;
pub mod modules;

use std::sync::Arc;
use dotenv::dotenv;

use common::db::{DbPool, init_db_pool};
use modules::user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载环境变量
    dotenv().ok();
    
    // 初始化日志
    tracing_subscriber::fmt::init();
    tracing::info!("Campus Backend is starting...");
    
    // 初始化数据库连接池
    let db_pool = Arc::new(init_db_pool().await?);
    tracing::info!("Database connection pool initialized.");
    
    // 构建路由
    let app = axum::Router::new()
        // 用户模块路由
        .nest("/api", user::router(Arc::clone(&db_pool)))
        // 健康检查端点
        .route("/health", axum::routing::get(|| async { "OK" }));
    
    // 获取服务器配置
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("Available endpoints:");
    tracing::info!("  POST   http://{}/api/users/login", addr);
    tracing::info!("  POST   http://{}/api/users/register", addr);
    tracing::info!("  GET    http://{}/api/users/{{id}}", addr);
    tracing::info!("  PUT    http://{}/api/users/{{id}}/profile", addr);
    tracing::info!("  POST   http://{}/api/users/{{id}}/logout", addr);
    tracing::info!("  PUT    http://{}/api/users/{{id}}/password", addr);
    tracing::info!("  GET    http://{}/health", addr);
    
    // 启动服务器
    axum::serve(listener, app).await?;
    
    Ok(())
}