// src/common/dev_tools.rs
use crate::common::{auth, error::AppError, state::JwtConfig};

/// 开发工具：打印测试用的 Token
pub fn print_test_tokens() -> Result<(), AppError> {
    // 临时读取一下配置（只在测试工具里这么做没关系）
    let config = JwtConfig::from_env();

    let test_users = vec![
        (1, "admin", "admin"),      // ID, 用户名, 角色
        (2, "student_a", "student"),
        (3, "teacher_b", "teacher"),
    ];

    println!("\n====== 🛠️ 开发测试 Token (有效期 {}秒) ======", config.expiration);
    for (uid, name, role) in test_users {
        let token = auth::generate_token(uid, role, &config.secret, config.expiration)?;
        println!("User: {:<10} | Role: {:<8} | Token: {}", name, role, token);
    }
    println!("====================================================\n");

    Ok(())
}