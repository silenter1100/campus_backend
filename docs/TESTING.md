# 测试指南

## 概述

本项目采用完整的测试体系，包括：

- ✅ 单元测试（Service 层）
- ✅ 集成测试（API 层）
- ✅ Mock 测试（可选）
- ✅ 数据库迁移管理

## 测试架构

```
tests/
├── common/
│   └── mod.rs              # 测试辅助函数
├── course_service_test.rs  # Service 层单元测试
├── course_api_test.rs      # API 集成测试
└── course_service_mock_test.rs  # Mock 测试示例

src/modules/course/
└── repository.rs           # Repository 抽象层（支持 Mock）
```

## 快速开始

### 1. 安装 SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features mysql
```

### 2. 配置测试数据库

编辑 `.env.test` 文件：

```env
DATABASE_URL=mysql://root:your_password@localhost:3306/campus_test
RUST_LOG=debug
```

### 3. 创建测试数据库并运行迁移

**手动执行:**

```bash
# 创建数据库
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS campus_test;"

# 运行迁移
set DATABASE_URL=mysql://root:your_password@localhost:3306/campus_test
sqlx migrate run
```

### 4. 运行测试

**手动执行:**

```bash
cargo test
```

**运行特定测试:**

```bash
# 只运行 Service 层测试
cargo test course_service_test

# 只运行 API 测试
cargo test course_api_test

# 运行单个测试
cargo test test_get_semesters

# 显示详细输出
cargo test -- --nocapture
```

## 测试类型详解

### 1. Service 层单元测试

**文件**: `tests/course_service_test.rs`

**特点**:

- 测试业务逻辑
- 使用真实数据库
- 自动清理测试数据
- 串行执行（避免冲突）

**示例**:

```rust
#[tokio::test]
#[serial]
async fn test_get_semesters() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;

    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;

    // 执行测试
    let result = service::get_semesters(&pool).await;

    assert!(result.is_ok());
    let semesters = result.unwrap();
    assert!(!semesters.is_empty());

    // 清理
    common::cleanup_test_data(&pool).await;
}
```

**覆盖的测试**:

- ✅ 获取学期列表
- ✅ 获取全校课程（无过滤）
- ✅ 获取全校课程（按学期过滤）
- ✅ 获取全校课程（按课程名过滤）
- ✅ 分页功能
- ✅ 获取用户课表
- ✅ 添加课表项
- ✅ 更新课表项
- ✅ 删除课表项
- ✅ 删除不存在的课表项（错误处理）

### 2. API 集成测试

**文件**: `tests/course_api_test.rs`

**特点**:

- 测试完整的 HTTP 请求/响应
- 测试 Protobuf 序列化/反序列化
- 测试路由和中间件
- 端到端测试

**示例**:

```rust
#[tokio::test]
#[serial]
async fn test_api_get_semesters() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;

    // 创建测试数据
    common::create_test_semester(&pool).await;

    // 创建测试应用
    let app = create_test_app().await;

    // 发送请求
    let request = Request::builder()
        .uri("/api/v1/semesters")
        .header("Accept", "application/x-protobuf")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // 验证响应
    assert_eq!(response.status(), StatusCode::OK);

    // 解析 Protobuf 响应
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    let proto_response = proto::GetSemestersResponse::decode(body_bytes).unwrap();
    assert_eq!(proto_response.code, 200);

    // 清理
    common::cleanup_test_data(&pool).await;
}
```

**覆盖的测试**:

- ✅ GET /api/v1/semesters
- ✅ GET /api/v1/courses
- ✅ GET /api/v1/schedule
- ✅ POST /api/v1/schedule
- ✅ PATCH /api/v1/schedule
- ✅ DELETE /api/v1/schedule

### 3. Mock 测试（可选）

**文件**: `tests/course_service_mock_test.rs`

**特点**:

- 不依赖真实数据库
- 快速执行
- 隔离测试
- 适合纯单元测试

**使用场景**:

- 测试复杂的业务逻辑
- 测试错误处理
- 测试边界条件
- CI/CD 环境

**示例**:

```rust
use mockall::predicate::*;

#[tokio::test]
async fn test_with_mock() {
    let mut mock_repo = MockCourseRepository::new();

    // 设置 Mock 期望
    mock_repo
        .expect_get_semesters()
        .times(1)
        .returning(|| {
            Ok(vec![Semester {
                id: 1,
                name: "Mock学期".to_string(),
                start_date: "2024-09-01".to_string(),
                end_date: "2025-01-15".to_string(),
                is_current: true,
            }])
        });

    // 使用 Mock 执行测试
    let result = service::get_semesters_with_repo(&mock_repo).await;
    assert!(result.is_ok());
}
```

## 测试辅助函数

**文件**: `tests/common/mod.rs`

提供的辅助函数：

```rust
// 初始化测试环境
init_test_env()

// 创建测试数据库连接池
create_test_pool() -> Pool<MySql>

// 清理测试数据
cleanup_test_data(pool: &Pool<MySql>)

// 创建测试学期
create_test_semester(pool: &Pool<MySql>) -> i64

// 创建测试课程
create_test_course(pool: &Pool<MySql>, semester_id: i64) -> i64

// 创建测试课表项
create_test_schedule_item(pool: &Pool<MySql>, user_id: i64, semester_id: i64) -> i64
```

## SQLx 迁移管理

### 创建新迁移

```bash
sqlx migrate add create_new_table
```

这会在 `migrations/` 目录创建新文件：

```
migrations/20241210120000_create_new_table.sql
```

### 运行迁移

```bash
# 运行所有待执行的迁移
sqlx migrate run

# 回滚最后一次迁移
sqlx migrate revert

# 查看迁移状态
sqlx migrate info
```

### 迁移文件格式

```sql
-- 20241210120000_create_new_table.sql

-- 向上迁移（创建）
CREATE TABLE new_table (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL
);

-- 向下迁移（回滚）
-- 在文件末尾添加：
-- migrate:down
-- DROP TABLE new_table;
```

## 测试最佳实践

### 1. 使用 `#[serial]` 避免并发冲突

```rust
use serial_test::serial;

#[tokio::test]
#[serial]  // 串行执行，避免数据库冲突
async fn test_something() {
    // ...
}
```

### 2. 始终清理测试数据

```rust
#[tokio::test]
#[serial]
async fn test_something() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;  // 测试前清理

    // 执行测试...

    common::cleanup_test_data(&pool).await;  // 测试后清理
}
```

### 3. 测试边界条件

```rust
// 测试空结果
// 测试不存在的 ID
// 测试无效输入
// 测试权限检查
```

### 4. 使用有意义的断言

```rust
// ❌ 不好
assert!(result.is_ok());

// ✅ 好
assert!(result.is_ok(), "获取学期列表失败: {:?}", result.err());

// ✅ 更好
let semesters = result.expect("获取学期列表失败");
assert_eq!(semesters.len(), 1, "应该返回 1 个学期");
assert_eq!(semesters[0].name, "2024-2025学年第一学期");
```

### 5. 测试错误情况

```rust
#[tokio::test]
#[serial]
async fn test_delete_nonexistent_item() {
    let pool = common::create_test_pool().await;

    let result = service::delete_schedule_item(&pool, 1, 99999).await;

    // 应该返回错误
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
}
```

## 测试覆盖率

### 安装 tarpaulin

```bash
cargo install cargo-tarpaulin
```

### 生成覆盖率报告

```bash
# HTML 报告
cargo tarpaulin --out Html

# 终端输出
cargo tarpaulin --out Stdout

# 排除测试文件
cargo tarpaulin --exclude-files tests/*
```

## CI/CD 集成

### GitHub Actions 示例

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      mysql:
        image: mysql:8.0
        env:
          MYSQL_ROOT_PASSWORD: test_password
          MYSQL_DATABASE: campus_test
        ports:
          - 3306:3306

    steps:
      - uses: actions/checkout@v2

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install SQLx CLI
        run: cargo install sqlx-cli --no-default-features --features mysql

      - name: Run migrations
        run: sqlx migrate run
        env:
          DATABASE_URL: mysql://root:test_password@localhost:3306/campus_test

      - name: Run tests
        run: cargo test --verbose
        env:
          DATABASE_URL: mysql://root:test_password@localhost:3306/campus_test
```

## 常见问题

### Q: 测试失败：连接数据库超时

A: 确保 MySQL 正在运行，并且 `.env.test` 中的连接信息正确。

### Q: 测试冲突：数据已存在

A: 使用 `#[serial]` 标记测试，并在测试前后调用 `cleanup_test_data()`。

### Q: SQLx 迁移失败

A: 检查迁移文件的 SQL 语法，确保数据库连接正常。

### Q: 如何跳过某些测试？

A: 使用 `#[ignore]` 标记：

```rust
#[tokio::test]
#[ignore]
async fn test_slow_operation() {
    // ...
}
```

运行被忽略的测试：

```bash
cargo test -- --ignored
```

## 下一步

- [ ] 添加性能测试（Benchmark）
- [ ] 添加压力测试
- [ ] 集成 Swagger/OpenAPI 测试
- [ ] 添加端到端测试（E2E）
- [ ] 配置持续集成（CI）

## 参考资源

- [Rust 测试文档](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [SQLx 文档](https://github.com/launchbadge/sqlx)
- [Axum 测试指南](https://docs.rs/axum/latest/axum/testing/index.html)
- [Mockall 文档](https://docs.rs/mockall/latest/mockall/)
