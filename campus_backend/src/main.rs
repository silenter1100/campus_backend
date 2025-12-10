// 声明一级模块
mod common;
mod modules;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
// ✨ 修正1：确保引用路径正确。如果 Any 报错，请检查 Cargo.toml 的 tower-http 是否开启了 cors feature
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 引入我们定义的 State
use crate::common::state::{AppState, JwtConfig};

#[tokio::main]
async fn main() {
    // 1. 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                // 如果环境变量没设置，默认打印 debug 级别的日志
                .unwrap_or_else(|_| "campus_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Campus Backend is starting...");

    // ✨ 修正2：你的 Cargo.toml 里添加的是 dotenvy，所以这里要用 dotenvy
    // 如果你同事的代码是 dotenv::dotenv()，请改为 dotenvy
    dotenvy::dotenv().ok();

    // 3. 获取数据库连接
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // ✨ 修正3：调用 common::db::create_pool
    // 如果这里报错，请检查 src/common/db.rs 里的 create_pool 函数前面有没有 pub
    let pool = common::db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection pool created");

    // 4. 初始化 AppState (融合了数据库和JWT配置)
    let jwt_config = JwtConfig::from_env();
    let state = AppState {
        pool,
        jwt_config: Arc::new(jwt_config),
    };

    // 5. 配置 CORS
    let cors = CorsLayer::new()
        // 这里允许所有来源、方法和头。
        // 如果 Any 报错，去 Cargo.toml 确认 tower-http = { ..., features = ["cors"] }
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 6. 构建路由
    let app = Router::new()
        // 通常建议叫 router()。如果你的代码里是 routes()，请把下面这行改成 .merge(modules::course::controller::routes())
        .merge(modules::course::controller::router())
        .layer(cors)

        .with_state(state);

    // 7. 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}