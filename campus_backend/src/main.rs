mod common;
mod modules;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::common::state::{AppState, JwtConfig};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "campus_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Campus Backend is starting...");

    // 使用 dotenvy 加载环境变量
    dotenvy::dotenv().ok();

    // 读取数据库地址
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 创建数据库连接池
    let pool = common::db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection pool created");

    // 初始化 JWT 配置
    let jwt_config = JwtConfig::from_env();
    let state = AppState {
        pool,
        jwt_config: Arc::new(jwt_config),
    };

    // 设置 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 注册路由（course + user + forum）
    let app = Router::new()
        // 课程模块
        .merge(modules::course::router())
        // 用户模块
        .merge(modules::user::router())
        // 论坛模块（原 bbs）
        .merge(modules::forum::router())

        .layer(cors)
        .with_state(state);

    // 启动服务
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
