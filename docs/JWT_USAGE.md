# JWT 认证使用指南

## 概述

本项目实现了简化的 JWT 认证系统，专为课程模块设计。无需依赖用户模块，可以独立开发和测试。

## 核心特性

- ✅ 标准 JWT token 生成和验证
- ✅ 从 token 中提取用户 ID
- ✅ 与现有课程 API 无缝集成
- ✅ 后续可轻松集成用户模块

## 快速开始

### 1. 环境配置

确保 `.env` 文件包含以下配置：

```env
JWT_SECRET=your_secret_key_here
JWT_EXPIRATION=3600
DEV_MODE=true
```

### 2. 生成测试 Token

使用 Python 脚本生成测试 token：

```bash
python generate_token.py
```

这会输出常用测试用户的 token 和对应的 curl 命令。

### 3. 测试 API

## 代码集成

### 在控制器中使用认证

```rust
use crate::common::auth::AuthUser;

async fn my_handler(
    auth_user: AuthUser, // 自动从 JWT 提取用户信息
    // ... 其他参数
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth_user.user_id; // 获取用户 ID
    // ... 业务逻辑
}
```

### 生成 Token

```rust
use crate::common::auth::generate_token_for_user;

// 为用户生成 token
let token = generate_token_for_user(user_id)?;
```

### 解析 Token

```rust
use crate::common::auth::get_user_id_from_token;

// 从 token 获取用户 ID
let user_id = get_user_id_from_token(&token)?;
```

## 架构说明

### JWT Claims 结构

```json
{
  "user_id": 1,
  "iat": 1640995200, // 签发时间
  "exp": 1640998800 // 过期时间
}
```

### 认证流程

1. 客户端在请求头中发送 `Authorization: Bearer <token>`
2. Axum 提取器自动验证 token
3. 验证成功后，将 `AuthUser` 注入到处理函数
4. 处理函数可直接使用 `auth_user.user_id`

### 错误处理

- 缺少 token：返回 401 Unauthorized
- token 无效：返回 401 Unauthorized
- token 过期：返回 401 Unauthorized

## 测试工具

### 1. Token 生成器

```bash
python generate_token.py
```

### 2. 认证测试

```bash
python test_jwt.py
```

### 3. 手动测试

```bash
# 生成用户 ID 为 123 的 token
python -c "from generate_token import generate_jwt_token; print(generate_jwt_token(123))"
```

## 与用户模块集成

当用户模块完成后，只需要：

1. 在用户登录成功后调用 `generate_token_for_user(user_id)`
2. 可选：添加 token 黑名单机制
3. 可选：添加 refresh token 机制

现有的课程模块代码无需修改。

## 安全注意事项

1. **生产环境**：确保 `JWT_SECRET` 足够复杂且保密
2. **HTTPS**：生产环境必须使用 HTTPS 传输 token
3. **过期时间**：根据安全需求调整 token 过期时间
4. **存储**：客户端应安全存储 token（如 HttpOnly Cookie）

## 常见问题

### Q: 如何自定义 token 过期时间？

A: 修改 `.env` 中的 `JWT_EXPIRATION`（单位：秒）

### Q: 如何添加更多 Claims？

A: 修改 `src/common/auth.rs` 中的 `Claims` 结构体

### Q: 如何实现登出？

A: JWT 是无状态的，客户端删除 token 即可。如需服务端登出，可实现 token 黑名单。

### Q: 如何处理 token 刷新？

A: 可以实现 refresh token 机制，或在 token 即将过期时自动刷新。
