# 快速启动指南

## 前置条件

- Rust 1.70+ 已安装
- MySQL 5.7+ 或 8.0+ 已安装并运行
- 基本的命令行操作知识

## 5 分钟快速启动

### 步骤 1: 创建数据库

```bash
# 登录 MySQL
mysql -u root -p

# 创建数据库
CREATE DATABASE campus_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

# 退出
exit
```

### 步骤 2: 执行数据库迁移

```bash
# 在项目根目录执行
mysql -u root -p campus_db < migrations/001_create_course_tables.sql
```

这将创建以下表并插入示例数据：

- `semesters` - 2 个学期
- `public_courses` - 5 门课程
- `schedule_items` - 用户课表项表（空）

### 步骤 3: 配置环境变量

```bash
# 复制示例配置
cp .env.example .env

# 编辑 .env 文件，修改数据库连接信息
# DATABASE_URL=mysql://root:your_password@localhost:3306/campus_db
```

**Windows 用户**:

```powershell
copy .env.example .env
notepad .env
```

### 步骤 4: 编译项目

```bash
# 首次编译会下载依赖，需要几分钟
cargo build --release
```

### 步骤 5: 运行服务器

```bash
# 开发模式（带日志）
cargo run

# 或使用编译好的二进制文件
./target/release/campus_backend
```

**Windows 用户**:

```powershell
.\target\release\campus_backend.exe
```

看到以下输出表示启动成功：

```
Campus Backend is starting...
Database connection pool created
Server listening on 0.0.0.0:3000
```

## 验证安装

### 方法 1: 使用 curl（需要 Protobuf 工具）

```bash
# 获取学期列表
curl -X GET http://localhost:3000/api/v1/semesters \
  -H "Accept: application/x-protobuf" \
  --output semesters.bin
```

### 方法 2: 使用 Postman

1. 打开 Postman
2. 创建新请求：GET `http://localhost:3000/api/v1/semesters`
3. 设置 Headers：`Accept: application/x-protobuf`
4. 发送请求

### 方法 3: 检查服务器日志

如果服务器正常运行，你应该能看到类似的日志：

```
2024-12-10T10:00:00.000Z INFO campus_backend: Campus Backend is starting...
2024-12-10T10:00:00.100Z INFO campus_backend: Database connection pool created
2024-12-10T10:00:00.200Z INFO campus_backend: Server listening on 0.0.0.0:3000
```

## API 端点列表

| 方法   | 路径                                               | 功能             |
| ------ | -------------------------------------------------- | ---------------- |
| GET    | `/api/v1/semesters`                                | 获取学期列表     |
| GET    | `/api/v1/courses?semester_id=1&page=1&pageSize=20` | 获取全校课程列表 |
| GET    | `/api/v1/schedule?semester_id=1`                   | 获取用户课表     |
| POST   | `/api/v1/schedule`                                 | 批量添加课表项   |
| PATCH  | `/api/v1/schedule?item_id=1`                       | 更新课表项       |
| DELETE | `/api/v1/schedule?item_id=1`                       | 删除课表项       |

## 常见问题

### Q1: 编译失败 - "DATABASE_URL must be set"

**解决方案**: 确保 `.env` 文件存在且包含正确的 `DATABASE_URL`

```bash
# 检查 .env 文件
cat .env

# 应该包含类似内容
DATABASE_URL=mysql://root:password@localhost:3306/campus_db
```

### Q2: 运行时错误 - "Failed to create database pool"

**可能原因**:

1. MySQL 服务未运行
2. 数据库连接信息错误
3. 数据库不存在

**解决方案**:

```bash
# 检查 MySQL 服务状态
# Linux/macOS
sudo systemctl status mysql

# Windows
# 打开服务管理器，查找 MySQL 服务

# 测试数据库连接
mysql -u root -p -e "SHOW DATABASES;"
```

### Q3: 编译警告 - "unused variables"

**说明**: 这些是正常的警告，不影响功能。主要是：

- 未使用的数据库字段（如 `created_at`, `updated_at`）
- 未使用的 Protobuf 消息类型

可以忽略这些警告。

### Q4: 端口 3000 已被占用

**解决方案**: 修改 `src/main.rs` 中的端口号

```rust
// 找到这一行
let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

// 改为其他端口，如 8080
let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
```

### Q5: Protobuf 响应如何解析？

**方法 1**: 使用 Postman 的 Protobuf 支持

**方法 2**: 使用 protoc 工具

```bash
# 安装 protoc
# macOS: brew install protobuf
# Linux: sudo apt install protobuf-compiler
# Windows: 下载 https://github.com/protocolbuffers/protobuf/releases

# 解码响应
protoc --decode=campus.course.GetSemestersResponse \
  proto/course.proto < semesters.bin
```

**方法 3**: 编写客户端代码（推荐）

## 下一步

1. **添加 JWT 认证**: 参考 `README_COURSE_MODULE.md` 的 JWT 部分
2. **编写测试**: 参考 Rust 测试文档
3. **部署到生产**: 使用 Docker 或直接部署二进制文件

## 开发模式

### 自动重新编译（推荐）

安装 cargo-watch:

```bash
cargo install cargo-watch

# 自动监听文件变化并重新编译运行
cargo watch -x run
```

### 查看详细日志

```bash
# 设置日志级别
RUST_LOG=debug cargo run
```

### 数据库管理

推荐工具：

- MySQL Workbench（GUI）
- DBeaver（跨平台 GUI）
- mycli（命令行，更友好）

```bash
# 安装 mycli
pip install mycli

# 连接数据库
mycli -u root -p campus_db
```

## 性能测试

使用 Apache Bench 进行简单的性能测试：

```bash
# 安装 ab
# macOS: brew install httpd
# Linux: sudo apt install apache2-utils

# 测试获取学期列表（1000 请求，并发 10）
ab -n 1000 -c 10 http://localhost:3000/api/v1/semesters
```

## 获取帮助

- 查看 `README_COURSE_MODULE.md` 了解详细文档
- 查看 `IMPLEMENTATION_SUMMARY.md` 了解实现细节
- 查看 `.kiro/steering/course-module-spec.md` 了解设计规范

## 许可证

MIT
