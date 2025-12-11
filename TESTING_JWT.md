# JWT 认证测试指南

## 快速开始

### 1. 测试 JWT Token 生成（无需服务器）

```bash
python test_jwt_only.py
```

这会测试 JWT token 的生成和解析，确保基础功能正常。

### 2. 测试完整 API（需要服务器）

```bash
# 启动服务器
cargo run

# 在另一个终端运行测试
python test_api.py
```

### 3. 使用不同用户测试

```bash
python test_api.py 1    # 用户 ID 1
python test_api.py 2    # 用户 ID 2
python test_api.py 999  # 用户 ID 999
```

## 测试内容

### 公开接口（无需认证）

- ✅ 获取学期列表
- ✅ 获取全校课程列表

### 需要认证的接口

- 🔐 获取用户课表
- 🔐 添加课表项
- 🔐 更新课表项
- 🔐 删除课表项

### 认证测试

- ❌ 无认证访问（应返回 401）
- ✅ 有效 Token 访问
- ❌ 无效 Token 访问（应返回 401）

## 手动测试命令

### 生成 Token

```bash
python generate_token.py
```

### 使用 curl 测试

```bash
# 获取用户课表（需要认证）
curl -H "Authorization: Bearer <your-token>" \
     "http://localhost:3000/api/v1/schedule?semester_id=1"

# 获取学期列表（公开接口）
curl "http://localhost:3000/api/v1/semesters"
```

## 常见问题

### Q: 401 Unauthorized 错误

A: 检查：

1. Token 是否正确生成
2. Authorization 头格式：`Bearer <token>`
3. Token 是否过期（默认 1 小时）

### Q: 如何查看 Token 内容？

A: 使用 `test_jwt_only.py` 或在线工具 jwt.io

### Q: 如何修改用户 ID？

A:

1. 命令行：`python test_api.py <user_id>`
2. 修改 `test_api.py` 中的 `TEST_USER_ID`
3. 使用 `generate_token.py` 生成特定用户的 token

## 测试流程

1. **基础测试**：`python test_jwt_only.py`
2. **启动服务器**：`cargo run`
3. **完整测试**：`python test_api.py`
4. **多用户测试**：`python test_api.py 2`

## 预期结果

### 成功的测试输出

```
🎓 课表模块 API 测试工具（JWT 认证版）
🔑 为用户 1 生成 JWT Token

============================================================
  测试：无认证访问（应该返回 401）
============================================================
状态码: 401
✅ 正确：未认证请求被拒绝

============================================================
  测试：获取学期列表（公开接口）
============================================================
状态码: 200
响应码: 200
消息: 成功
```

### 认证成功的标志

- 🔑 Token 生成成功
- ✅ 认证接口返回 200
- ❌ 无认证访问返回 401

## 调试技巧

1. **查看服务器日志**：观察认证中间件的日志输出
2. **检查 Token 格式**：确保是 `Bearer <token>` 格式
3. **验证环境变量**：确保 `.env` 中的 `JWT_SECRET` 正确
4. **测试 Token 有效性**：使用 `test_jwt_only.py` 验证
