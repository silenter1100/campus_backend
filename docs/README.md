# 校园助手 App - 服务端

## 项目简介

校园助手 App 的 Rust 后端服务，为 iOS 客户端提供 RESTful API，支持课表管理、校园论坛、活动发布等核心功能。

## 技术架构

### 核心技术栈

- **Web 框架**: Axum 0.7 - 高性能异步 Web 框架
- **异步运行时**: Tokio - Rust 异步生态核心
- **数据库访问**: SQLx - 异步 SQL 工具包，编译时检查 SQL 语句
- **数据库**: MySQL - 使用 SQLx 异步连接
- **序列化**: Serde + Serde JSON - JSON 数据处理
- **认证**: JWT (jsonwebtoken) - Token 认证
- **密码加密**: Bcrypt - 用户密码哈希
- **缓存**: Redis - 会话管理、分布式锁
- **日志**: Tracing + Tracing-subscriber - 结构化日志
- **中间件**: Tower + Tower-HTTP - CORS、日志、静态文件服务

### 通信协议

- **传输协议**: Protocol Buffers (Protobuf) - 高效的二进制序列化协议
- **API 层**: RESTful API (Protobuf 二进制) - HTTP 接口使用 `application/x-protobuf` 传输
- **跨语言调用**: UniFFI - iOS 客户端通过 UniFFI 直接调用 Rust 函数(如违禁词检测)

## 功能模块

### 1. 用户模块 (User)

- 用户注册/登录/登出
- 个人资料管理
- 密码修改
- 管理员批量导入用户
- 角色权限管理(学生/管理员)

### 2. 课表模块 (Course)

- 获取全校课程列表(支持筛选、分页)
- 学期管理
- 个人课表管理(增删改查)
- 自定义课程支持
- 课程颜色标记

### 3. 论坛模块 (BBS)

- 板块管理(预设/动态板块)
- 帖子发布/查看/删除
- 评论系统(支持楼中楼)
- 点赞/收藏功能
- 内容举报与审核
- 敏感词过滤

### 4. 活动模块 (Activity)

- 活动发布(管理员)
- 活动列表/详情查看
- 活动报名/取消报名
- 活动收藏
- 报名信息管理

### 5. 文件存储

- 图片/文档上传
- 自动生成缩略图
- 支持格式: PNG, JPG, PDF

## 项目结构

```
campus_backend/
├── src/
│   ├── main.rs              # 应用入口
│   ├── common/              # 公共模块
│   │   ├── db.rs           # 数据库连接
│   │   ├── error.rs        # 错误处理
│   │   └── mod.rs
│   └── modules/             # 业务模块
│       ├── user/           # 用户模块
│       ├── course/         # 课表模块
│       ├── bbs/            # 论坛模块
│       └── activity/       # 活动模块
├── docs/                    # API 文档
│   ├── API_个人模块.md
│   ├── API_课表模块.md
│   ├── API_论坛模块.md
│   ├── API_活动模块.md
│   └── 客户端需求.md
├── Cargo.toml              # 依赖配置
└── .env                    # 环境变量(需自行创建)
```

## 快速开始

### 🚀 5 分钟快速启动

#### Windows 用户（推荐）

# 启动服务

cargo run

````

#### 手动安装

1. **克隆项目**

```bash
git clone <repository-url>
cd campus_backend
````

2. **创建数据库**

```bash
mysql -u root -p -e "CREATE DATABASE campus_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
```

3. **运行数据库迁移**

```bash
mysql -u root -p campus_db < migrations/001_init_schema.sql
mysql -u root -p campus_db < migrations/002_seed_data.sql
```

4. **配置环境变量**

```bash
cp .env.example .env
# 编辑 .env 文件，填入数据库密码
```

5. **编译并运行**

```bash
cargo build
cargo run
```

### 📖 详细文档

- [快速开始指南](./快速开始.md) - 5 分钟快速启动
- [架构优化方案](./架构优化方案.md) - 完整架构分析
- [课程表模块开发指南](./开发指南_课程表模块.md) - 详细开发流程
- [项目总结](./项目总结.md) - 项目成果总结

## API 文档

详细 API 文档请查看 `docs/` 目录:

- [个人模块 API](./API_个人模块.md)
- [课表模块 API](./API_课表模块.md)
- [论坛模块 API](./API_论坛模块.md)
- [活动模块 API](./API_活动模块.md)

## 开发说明

### 数据库迁移

使用 SQLx CLI 进行数据库迁移管理:

```bash
# 安装 SQLx CLI
cargo install sqlx-cli --no-default-features --features mysql

# 创建迁移
sqlx migrate add <migration_name>

# 运行迁移
sqlx migrate run
```

### 认证机制

- 使用 JWT Bearer Token 认证
- Token 在 Authorization Header 中传递
- 格式: `Authorization: Bearer <token>`

### 错误处理

统一的错误响应格式:

```json
{
  "code": 400,
  "message": "错误描述",
  "data": null
}
```

### 日志

使用 Tracing 进行结构化日志记录，支持不同日志级别

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交变更
4. 推送到分支
5. 创建 Pull Request
