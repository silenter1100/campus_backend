use std::sync::Arc;
use sqlx::MySqlPool; // ✨ 修正：改成 MySqlPool
use axum::extract::FromRef;

#[derive(Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            expiration: std::env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .expect("JWT_EXPIRATION must be a number"),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub jwt_config: Arc<JwtConfig>,
    pub pool: MySqlPool, // ✨ 修正：改成 MySqlPool
}

// ✨ 修正：为 MySqlPool 实现 FromRef
impl FromRef<AppState> for MySqlPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Arc<JwtConfig> {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_config.clone()
    }
}