use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("认证失败: {0}")]
    AuthError(String),
    
    #[error("未找到资源: {0}")]
    NotFound(String),
    
    #[error("请求参数错误: {0}")]
    BadRequest(String),
    
    #[error("内部服务器错误: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(e) => {
                tracing::error!("数据库错误: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("数据库错误: {}", e))
            }
            AppError::AuthError(msg) => {
                tracing::warn!("认证失败: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
            AppError::NotFound(msg) => {
                tracing::info!("未找到资源: {}", msg);
                (StatusCode::NOT_FOUND, msg)
            }
            AppError::BadRequest(msg) => {
                tracing::info!("请求参数错误: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::InternalServerError(msg) => {
                tracing::error!("内部服务器错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
        };

        let body = Json(json!({
            "code": status.as_u16(),
            "message": error_message,
            "data": null
        }));

        (status, body).into_response()
    }
}

// 方便从其他错误类型转换
impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::InternalServerError(err)
    }
}
