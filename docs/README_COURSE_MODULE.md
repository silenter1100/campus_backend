# 课程表模块实现说明

## 项目概述

这是校园后端系统的课程表模块实现，使用 Rust + Axum + SQLx + Protobuf 技术栈。

## 技术栈

- **Web 框架**: Axum 0.7
- **数据库**: MySQL + SQLx 0.7
- **传输协议**: Protocol Buffers (Protobuf)
- **异步运行时**: Tokio 1.x

## 项目结构

```
campus_backend/
├── src/
│   ├── common/              # 公共模块
│   │   ├── db.rs           # 数据库连接池
│   │   ├── error.rs        # 统一错误处理
│   │   └── mod.rs
│   ├── modules/
│   │   └── course/         # 课程表模块
│   │       ├── entity.rs      # 数据实体定义
│   │       ├── service.rs     # 业务逻辑层
│   │       ├── controller.rs  # 控制器/路由层
│   │       └── mod.rs
│   └── main.rs             # 应用入口
├── proto/
│   └── course.proto        # Protobuf 定义
├── migrations/
│   └── 001_create_course_tables.sql  # 数据库迁移
├── build.rs                # Protobuf 编译配置
└── Cargo.toml             # 依赖配置
```

## 快速开始

### 1. 环境准备

确保已安装：

- Rust 1.70+
- MySQL 5.7+ 或 8.0+
- Protocol Buffers 编译器（可选，prost-build 会自动处理）

### 2. 配置数据库

1. 创建数据库：

```sql
CREATE DATABASE campus_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
```

2. 执行迁移脚本：

```bash
mysql -u your_username -p campus_db < migrations/001_create_course_tables.sql
```

### 3. 配置环境变量

复制 `.env.example` 为 `.env` 并修改配置：

```bash
cp .env.example .env
```

编辑 `.env` 文件：

```
DATABASE_URL=mysql://your_username:your_password@localhost:3306/campus_db
```

### 4. 编译和运行

```bash
# 编译项目（会自动生成 Protobuf 代码）
cargo build

# 运行项目
cargo run
```

服务器将在 `http://0.0.0.0:3000` 启动。

## API 端点

所有 API 使用 Protobuf 二进制格式传输（Content-Type: `application/x-protobuf`）

### 1. 获取学期列表

- **GET** `/api/v1/semesters`
- 返回所有学期信息

### 2. 获取全校课程列表

- **GET** `/api/v1/courses`
- 查询参数：
  - `semester_id` (可选): 学期 ID
  - `name` (可选): 课程名称模糊搜索
  - `teacher` (可选): 教师姓名模糊搜索
  - `page` (可选): 页码，默认 1
  - `pageSize` (可选): 每页数量，默认 20

### 3. 获取用户课表

- **GET** `/api/v1/schedule`
- 查询参数：
  - `semester_id` (必需): 学期 ID
  - `week` (可选): 筛选指定周

### 4. 批量添加课表项

- **POST** `/api/v1/schedule`
- 请求体: `AddScheduleItemsRequest` (Protobuf)
- 支持批量添加，返回成功和失败列表

### 5. 更新课表项

- **PATCH** `/api/v1/schedule`
- 查询参数：
  - `item_id` (必需): 课表项 ID
- 请求体: `UpdateScheduleItemRequest` (Protobuf)

### 6. 删除课表项

- **DELETE** `/api/v1/schedule`
- 查询参数：
  - `item_id` (必需): 课表项 ID

## 核心功能

### 1. 时间冲突检测

添加或更新课表项时，系统会自动检测：

- 同一用户
- 同一学期
- 同一星期几
- 时间段重叠
- 周次重叠

### 2. 批量操作

POST 接口支持批量添加课表项，每个项独立验证：

- 成功的项会被添加
- 失败的项会返回错误信息
- 不会因为部分失败而回滚整个操作

### 3. 数据验证

- `is_custom` 和 `source_id` 互斥验证
- 时间范围验证（start_section <= end_section）
- 星期几范围验证（1-7）
- 颜色格式验证（#RRGGBB）

## 数据库设计

### 核心表

1. **semesters** - 学期信息表
2. **public_courses** - 全校课程表
3. **schedule_items** - 用户课表项表

### 特殊字段

- `weeks_range`: JSON 数组，存储周次范围，如 `[1,2,3,4,5]`
- `color_hex`: 课程颜色，格式 `#RRGGBB`
- `is_custom`: 区分自定义课程和全校课程

## 开发注意事项

### 1. Protobuf 代码生成

编译时会自动生成 Protobuf 代码到 `target/debug/build/.../out/campus.course.rs`

如果修改了 `proto/course.proto`，需要重新编译：

```bash
cargo clean
cargo build
```

### 2. 数据库连接

使用 SQLx 的编译时检查功能，需要：

- 数据库必须在编译时可访问
- 或者设置 `SQLX_OFFLINE=true` 使用离线模式

### 3. JWT 认证（TODO）

当前代码中 `user_id` 使用固定值 1，实际应该：

- 从 JWT token 中提取 user_id
- 添加认证中间件
- 处理未授权情况

### 4. 错误处理

所有错误都通过 `AppError` 统一处理，自动转换为合适的 HTTP 状态码。

## 测试

### 使用 curl 测试（需要 Protobuf 工具）

由于使用 Protobuf 二进制格式，建议使用：

- Postman（支持 Protobuf）
- grpcurl
- 自定义测试客户端

### 示例：获取学期列表

```bash
curl -X GET http://localhost:3000/api/v1/semesters \
  -H "Accept: application/x-protobuf" \
  --output semesters.bin

# 需要 protoc 工具解码
protoc --decode=campus.course.GetSemestersResponse \
  proto/course.proto < semesters.bin
```

## 性能优化建议

1. **索引优化**: 已为常用查询字段添加索引
2. **连接池**: 使用 SQLx 连接池，最大连接数 10
3. **分页查询**: 避免一次性加载大量数据
4. **JSON 字段**: 使用 MySQL JSON 类型存储数组数据

## 后续改进

- [ ] 添加 JWT 认证
- [ ] 添加 Redis 缓存
- [ ] 添加单元测试
- [ ] 添加集成测试
- [ ] 添加 API 文档生成
- [ ] 添加日志记录
- [ ] 添加性能监控

## 许可证

MIT
