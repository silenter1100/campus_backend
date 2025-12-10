use crate::common::auth::generate_token_for_user;
use crate::common::AppError;

/// 开发工具：打印常用测试用户的 JWT token
pub fn print_test_tokens() -> Result<(), AppError> {
    let test_users = vec![
        (1, "admin"),
        (2, "user1"), 
        (3, "user2"),
        (999, "test"),
    ];

    println!("=== 测试用 JWT Tokens ===");
    for (user_id, username) in test_users {
        let token = generate_token_for_user(user_id)?;
        println!("用户: {} (ID: {}) -> Token: {}", username, user_id, token);
    }
    println!("========================");

    Ok(())
}

/// 开发工具：生成单个用户的 token（便捷函数）
pub fn generate_test_token(user_id: i64) -> Result<String, AppError> {
    generate_token_for_user(user_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_test_token() {
        std::env::set_var("JWT_SECRET", "test_secret");
        std::env::set_var("JWT_EXPIRATION", "3600");
        
        let token = generate_test_token(123).unwrap();
        assert!(!token.is_empty());
    }
}