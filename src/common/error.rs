use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 1. 根据错误的类型，匹配出 HTTP 状态码、错误码和错误信息
        let (status, code_num, message) = match self {
            AppError::Database(e) => {
                // 记录详细的数据库错误日志，但只给前端返回 "Internal Server Error"
                tracing::error!("Database Error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, 500, "Internal Server Error".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, 400, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, 401, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, 403, msg),
            AppError::Internal(msg) => {
                tracing::error!("Internal Error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, 500, msg)
            }
        };

        // 2. 构建 JSON 响应体
        let body = Json(json!({
            "code": code_num,
            "message": message,
            "data": null 
        }));

        // 3. 转换为 Axum 的 Response
        (status, body).into_response()
    }
}