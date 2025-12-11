// src/common/auth.rs
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::common::error::AppError;
use crate::common::state::AppState; // 引入刚才定义的 State

// ==========================================
// 1. 定义数据结构 (增加了 role)
// ==========================================

// JWT 里的载荷
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: String,  // 修改为String类型以支持UUID
    pub role: String,     // 角色字段 (student/admin)
    pub exp: usize,
    pub iat: usize,
}

// Controller 里直接拿到的用户信息对象
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,  // 修改为String类型以支持UUID
    pub role: String,     // 让 Controller 也能直接读取角色
}

// ==========================================
// 2. 实现 Axum 的提取器 (核心逻辑)
// ==========================================

#[async_trait]
impl FromRequestParts<AppState> for AuthUser
where
    AppState: Send + Sync,
{
    type Rejection = AppError;

    // ✨ 注意：这里第二个参数是 &AppState，不再是空
    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // 1. 提取 Header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Unauthorized("缺少认证头 (Missing Auth Header)".to_string()))?;

        // 2. 直接从内存(AppState)获取配置，不再读环境变量
        let config = &state.jwt_config;

        // 3. 解码验证
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(config.secret.as_bytes()),
            &Validation::default(),
        )
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Token 已过期 (Expired)".to_string())
                }
                _ => AppError::Unauthorized("Token 无效 (Invalid)".to_string()),
            })?;

        // 4. 返回封装好的用户对象
        Ok(AuthUser {
            user_id: token_data.claims.user_id,
            role: token_data.claims.role, // 传递角色
        })
    }
}

// ==========================================
// 3. 辅助函数 (生成 Token)
// ==========================================

/// 生成 Token 的通用函数
/// 注意：现在需要传入 role
pub fn generate_token(user_id: &str, role: &str, secret: &str, expiration_seconds: i64) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        user_id: user_id.to_string(),  // 转换为String
        role: role.to_owned(),         // 写入角色
        exp: now + expiration_seconds as usize,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
        .map_err(|e| AppError::InternalError(format!("Token creation failed: {}", e)))
}

// ==========================================
// 4. 便捷函数 (向后兼容)
// ==========================================

/// 便捷函数：为指定用户ID生成token（使用环境变量配置）
pub fn generate_token_for_user(user_id: &str) -> Result<String, AppError> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::InternalError("JWT_SECRET not set".to_string()))?;
    
    let expiration = std::env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "3600".to_string())
        .parse::<i64>()
        .map_err(|_| AppError::InternalError("Invalid JWT_EXPIRATION".to_string()))?;

    // 默认角色为student，实际使用时应该从用户数据中获取
    generate_token(user_id, "student", &secret, expiration)
}

/// 便捷函数：解析token获取用户ID
pub fn get_user_id_from_token(token: &str) -> Result<String, AppError> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::InternalError("JWT_SECRET not set".to_string()))?;
    
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AppError::Unauthorized("Token expired".to_string())
        }
        _ => AppError::Unauthorized("Invalid token".to_string()),
    })?;

    Ok(claims.user_id)
}

/// 便捷函数：验证token并获取完整的Claims
pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::InternalError("JWT_SECRET not set".to_string()))?;
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AppError::Unauthorized("Token expired".to_string())
        }
        _ => AppError::Unauthorized("Invalid token".to_string()),
    })
}