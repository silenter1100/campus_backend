use campus_backend::common::error::{AppError, ErrorResponse};
use axum::{response::IntoResponse, body::to_bytes};
use prost::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 错误响应格式示例 ===\n");

    // 示例 1: NotFound 错误
    let error = AppError::NotFound("课程未找到".to_string());
    let response = error.into_response();
    
    println!("1. NotFound 错误:");
    println!("   HTTP 状态码: {}", response.status());
    println!("   Content-Type: {:?}", response.headers().get("content-type"));
    
    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let error_response = ErrorResponse::decode(&body[..])?;
    println!("   Protobuf 响应:");
    println!("     code: {}", error_response.code);
    println!("     message: {}", error_response.message);
    println!();

    // 示例 2: BadRequest 错误
    let error = AppError::BadRequest("请求参数无效".to_string());
    let response = error.into_response();
    
    println!("2. BadRequest 错误:");
    println!("   HTTP 状态码: {}", response.status());
    
    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let error_response = ErrorResponse::decode(&body[..])?;
    println!("   Protobuf 响应:");
    println!("     code: {}", error_response.code);
    println!("     message: {}", error_response.message);
    println!();

    // 示例 3: 数据库错误
    let db_error = sqlx::Error::RowNotFound;
    let error = AppError::from(db_error);
    let response = error.into_response();
    
    println!("3. 数据库错误:");
    println!("   HTTP 状态码: {}", response.status());
    
    let body = to_bytes(response.into_body(), usize::MAX).await?;
    let error_response = ErrorResponse::decode(&body[..])?;
    println!("   Protobuf 响应:");
    println!("     code: {}", error_response.code);
    println!("     message: {}", error_response.message);

    Ok(())
}