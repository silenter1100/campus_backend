use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

/// 应用统一错误类型
#[derive(Debug)]
pub enum AppError {
    /// 数据库错误
    DatabaseError(sqlx::Error),
    /// 资源未找到
    NotFound(String),
    /// 请求参数错误
    BadRequest(String),
    /// 未授权 (Token 错误)
    Unauthorized(String),

    // 用于权限不足的情况
    /// 禁止访问 (权限不足)
    Forbidden(String),

    /// 内部服务器错误
    #[allow(dead_code)]
    InternalError(String),
    /// Protobuf 解析错误
    ProtobufError(prost::DecodeError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg), // ✨ 对应的格式化
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::ProtobufError(e) => write!(f, "Protobuf error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

/// 从 sqlx::Error 转换
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

/// 从 prost::DecodeError 转换
impl From<prost::DecodeError> for AppError {
    fn from(err: prost::DecodeError) -> Self {
        AppError::ProtobufError(err)
    }
}

/// 实现 IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseError(e) => {
                // tracing::error!("Database error: {:?}", e); // 确保你有 tracing 依赖，如果没有先注释掉
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),

            // ✨ 【改动3】对应的 HTTP 403 状态码
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),

            AppError::InternalError(msg) => {
                // tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::ProtobufError(e) => {
                // tracing::error!("Protobuf error: {:?}", e);
                (StatusCode::BAD_REQUEST, format!("Protobuf 解析错误: {}", e))
            }
        };

        // 返回简单的文本响应
        (status, message).into_response()
    }
}