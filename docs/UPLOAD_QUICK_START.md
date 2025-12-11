# Upload 模块快速启动指南

## 快速开始（3 分钟）

### 1. 应用数据库迁移

```bash
# 方法1: 使用sqlx migrate
sqlx migrate run --database-url "mysql://app_user:AppPass123!@localhost:3306/campus_db"

# 方法2: 直接执行SQL
mysql -u app_user -pAppPass123! -h localhost campus_db < migrations/20240104000000_create_upload_files_table.sql
```

### 2. 启动服务器

```bash
cargo run
```

服务器将在 `http://localhost:3000` 启动

### 3. 测试 API

```bash
# 运行测试脚本
python test_upload_api.py
```

## 详细测试步骤

### 步骤 1: 创建测试用户（如果还没有）

```bash
curl -X POST http://localhost:3000/api/v1/user/register \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user_001",
    "password": "test123",
    "name": "测试用户",
    "role": "student"
  }'
```

### 步骤 2: 登录获取 Token

```bash
curl -X POST http://localhost:3000/api/v1/user/login \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "test_user_001",
    "password": "test123"
  }'
```

保存返回的 token，例如：

```json
{
  "code": 200,
  "message": "登录成功",
  "data": {
    "token": "eyJ0eXAiOiJKV1QiLCJhbGc..."
  }
}
```

### 步骤 3: 上传文件

创建一个测试图片文件 `test.png`，然后：

```bash
curl -X POST http://localhost:3000/api/v1/storage/upload \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -F "file=@test.png"
```

响应示例：

```json
{
  "code": 200,
  "message": "上传成功",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "url": "https://mock-bucket.oss-cn-hangzhou.aliyuncs.com/images/2024/12/11/xxx.png",
    "thumbnail_url": "https://mock-bucket.oss-cn-hangzhou.aliyuncs.com/images/2024/12/11/xxx.png?x-oss-process=image/resize,w_200,h_200",
    "size": 12345
  }
}
```

### 步骤 4: 查看我的文件列表

```bash
curl http://localhost:3000/api/v1/storage/files \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

### 步骤 5: 获取文件信息

```bash
curl http://localhost:3000/api/v1/storage/files/FILE_ID_HERE
```

### 步骤 6: 删除文件

```bash
curl -X DELETE http://localhost:3000/api/v1/storage/files/FILE_ID_HERE \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

## 使用 Python 测试脚本

最简单的测试方法：

```bash
# 1. 确保服务器正在运行
cargo run

# 2. 在另一个终端运行测试脚本
python test_upload_api.py
```

测试脚本会自动：

1. 登录获取 token
2. 测试未认证上传（应该失败）
3. 上传测试图片
4. 查询文件信息
5. 查看文件列表
6. 删除文件
7. 验证删除成功

## 常见问题

### Q1: 编译失败

```bash
# 清理并重新编译
cargo clean
cargo build
```

### Q2: 数据库连接失败

检查 `.env` 文件中的 `DATABASE_URL` 是否正确：

```env
DATABASE_URL=mysql://app_user:AppPass123!@localhost:3306/campus_db
```

### Q3: Token 无效

Token 有效期为 1 小时（3600 秒），过期后需要重新登录。

### Q4: 文件上传失败

检查：

1. 是否携带了正确的 Authorization header
2. 文件大小是否超过 10MB
3. 文件类型是否支持（PNG, JPEG, PDF）

### Q5: Mock 模式下文件在哪里？

Mock 模式下文件不会真正保存，只有数据库记录。要真正上传到 OSS，需要：

1. 配置真实的 OSS 凭证
2. 修改 `service.rs` 取消 Mock 代码

## 支持的文件类型

当前支持：

- ✅ PNG 图片
- ✅ JPEG/JPG 图片
- ✅ PDF 文档

待扩展：

- ⏳ GIF 图片
- ⏳ WebP 图片
- ⏳ Word 文档
- ⏳ Excel 文档

## 文件大小限制

- 最大文件大小：10MB
- 可在 `controller.rs` 中修改 `MAX_FILE_SIZE` 常量

## 下一步

1. **开发环境**: 继续使用 Mock 模式开发其他功能
2. **测试环境**: 配置测试 OSS 账号
3. **生产环境**: 配置生产 OSS 账号和 CDN

## 相关文档

- [功能总结](./UPLOAD_MODULE_SUMMARY.md)
- [实现完成报告](./UPLOAD_IMPLEMENTATION_COMPLETE.md)
- [API 文档](./UPLOAD_MODULE_SUMMARY.md#api文档)

## 技术支持

如有问题，请查看：

1. 服务器日志输出
2. 数据库表结构
3. 环境变量配置
4. JWT token 是否有效
