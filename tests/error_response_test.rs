#[cfg(test)]
mod tests {
    use campus_backend::common::error::{AppError, ErrorResponse};
    use axum::{response::IntoResponse, body::to_bytes};
    use prost::Message;

    #[tokio::test]
    async fn test_error_response_format() {
        // 创建一个错误
        let error = AppError::NotFound("课程未找到".to_string());
        
        // 转换为响应
        let response = error.into_response();
        
        // 检查状态码
        assert_eq!(response.status(), 404);
        
        // 检查 Content-Type
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/x-protobuf"
        );
        
        // 检查响应体是否为有效的 protobuf
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response = ErrorResponse::decode(&body[..]).unwrap();
        
        assert_eq!(error_response.code, 404);
        assert_eq!(error_response.message, "课程未找到");
    }

    #[tokio::test]
    async fn test_database_error_response() {
        // 模拟数据库错误
        let db_error = sqlx::Error::RowNotFound;
        let error = AppError::from(db_error);
        
        let response = error.into_response();
        
        assert_eq!(response.status(), 500);
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response = ErrorResponse::decode(&body[..]).unwrap();
        
        assert_eq!(error_response.code, 500);
        assert_eq!(error_response.message, "数据库错误");
    }
}