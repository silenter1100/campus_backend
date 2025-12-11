use std::sync::Arc;
use sqlx::MySqlPool;
use axum::extract::FromRef;
use crate::modules::upload::UploadService;

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
    pub pool: MySqlPool,
    pub upload_service: Option<Arc<UploadService>>,
}

// 为 MySqlPool 实现 FromRef
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

// 为 UploadService 实现 FromRef
impl FromRef<AppState> for Arc<UploadService> {
    fn from_ref(state: &AppState) -> Self {
        state.upload_service.clone().expect("UploadService not initialized")
    }
}