use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

// 1. 定义 JWT 里的载荷 (Claims)
// 根据 RFC 7519，"exp" 是过期时间，"sub" 通常存用户ID
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户 ID
    pub exp: usize,  // 过期时间戳
    // 你可以在这里加更多字段，比如 role: String
}

// 2. JWT 密钥（生产环境应该从环境变量读取）
fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_key_123456".to_string())
}

// 3. 鉴权中间件
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // A. 尝试获取 Authorization header
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        // 如果没有 Header，视情况而定：
        // 1. 如果是公开接口（如登录、查看列表），可能允许通过（但 Extension 里没有 user_id）
        // 2. 如果是严格鉴权，直接返回 401
        // 这里为了简单，如果没 Token，就不注入 Extension，让 Controller 自己判断 Option<String>
        return Ok(next.run(req).await);
    };

    // B. 解析 "Bearer <token>"
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let token = &auth_header[7..];

    // C. 验证 Token
    let secret = get_jwt_secret();
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(data) => data,
        Err(_) => return Err(StatusCode::UNAUTHORIZED), // Token 无效或过期
    };

    // D. 将解析出的 User ID 注入到请求扩展中
    // 这样 Controller 里的 Extension(user_id) 就能拿到了
    req.extensions_mut().insert(token_data.claims.sub);

    Ok(next.run(req).await)
}