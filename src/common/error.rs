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
    /// 未授权
    #[allow(dead_code)]
    Unauthorized(String),
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

/// 实现 IntoResponse，使 AppError 可以作为 Axum 的响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::ProtobufError(e) => {
                tracing::error!("Protobuf error: {:?}", e);
                (StatusCode::BAD_REQUEST, format!("Protobuf 解析错误: {}", e))
            }
        };

        // 返回简单的文本响应
        // 注意：实际应该返回 Protobuf 格式的错误响应
        (status, message).into_response()
    }
}
