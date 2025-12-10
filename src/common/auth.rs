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

use super::AppError;

// JWT Claims 结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: i64,
    pub exp: usize, // 过期时间
    pub iat: usize, // 签发时间
}

// 用户认证信息提取器
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
}

// JWT 配置
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64, // 秒
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::InternalError("JWT_SECRET not set".to_string()))?;
        
        let expiration = std::env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "3600".to_string()) // 默认1小时
            .parse::<i64>()
            .map_err(|_| AppError::InternalError("Invalid JWT_EXPIRATION".to_string()))?;

        Ok(Self { secret, expiration })
    }

    /// 生成 JWT token
    pub fn generate_token(&self, user_id: i64) -> Result<String, AppError> {
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            user_id,
            exp: now + self.expiration as usize,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))
    }

    /// 验证 JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
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
}

// 从请求中提取认证用户信息
#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从请求头中提取 Authorization
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
            .await
            .map_err(|_| AppError::Unauthorized("Missing authorization header".to_string()))?;

        // 验证 token
        let jwt_config = JwtConfig::from_env()?;
        let claims = jwt_config.verify_token(bearer.token())?;

        Ok(AuthUser {
            user_id: claims.user_id,
        })
    }
}

/// 便捷函数：为指定用户ID生成token
pub fn generate_token_for_user(user_id: i64) -> Result<String, AppError> {
    let jwt_config = JwtConfig::from_env()?;
    jwt_config.generate_token(user_id)
}

/// 便捷函数：解析token获取用户ID
pub fn get_user_id_from_token(token: &str) -> Result<i64, AppError> {
    let jwt_config = JwtConfig::from_env()?;
    let claims = jwt_config.verify_token(token)?;
    Ok(claims.user_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        std::env::set_var("JWT_SECRET", "test_secret");
        std::env::set_var("JWT_EXPIRATION", "3600");
        
        let user_id = 123;
        let token = generate_token_for_user(user_id).unwrap();
        let extracted_user_id = get_user_id_from_token(&token).unwrap();

        assert_eq!(extracted_user_id, user_id);
    }

    #[test]
    fn test_invalid_token() {
        std::env::set_var("JWT_SECRET", "test_secret");
        
        let result = get_user_id_from_token("invalid_token");
        assert!(result.is_err());
    }
}