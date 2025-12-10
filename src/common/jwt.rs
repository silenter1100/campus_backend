use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub exp: usize, // 过期时间
    pub iat: usize, // 签发时间
}

pub struct JwtConfig {
    pub secret: String,
    pub expiration_days: i64,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file");
        let expiration_days = env::var("JWT_EXPIRATION_DAYS")
            .unwrap_or_else(|_| "7".to_string())
            .parse::<i64>()
            .expect("JWT_EXPIRATION_DAYS must be a number");
        JwtConfig {
            secret,
            expiration_days,
        }
    }
}

pub fn generate_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let config = JwtConfig::from_env();
    let now = Utc::now();
    let exp = now + Duration::days(config.expiration_days);
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )?;
    
    Ok(token)
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config = JwtConfig::from_env();
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

/// 检查JWT token是否已过期
/// 返回 true 表示token已过期，false 表示未过期或验证失败（如签名无效）
pub fn is_token_expired(token: &str) -> bool {
    match validate_token(token) {
        Ok(_) => false,
        Err(err) => {
            match err.kind() {
                ErrorKind::ExpiredSignature => true,
                _ => false,
            }
        }
    }
}

/// 验证JWT token但不检查过期时间（用于刷新token等场景）
pub fn validate_token_without_expiry(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config = JwtConfig::from_env();
    let mut validation = Validation::default();
    validation.validate_exp = false;
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}
