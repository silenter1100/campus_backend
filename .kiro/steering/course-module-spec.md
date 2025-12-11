---
title: 课表模块实现规范
inclusion: manual
---

# 课表模块实现规范 (Course Module Specification)

## 项目背景

这是一个校园后端系统 (Campus Backend)，使用 Rust 构建，采用现代化的异步 Web 框架和数据库技术栈。

## 技术栈约束

### 核心框架

- **Web 框架**: Axum 0.7 (类似 Spring Boot)
- **异步运行时**: Tokio 1.x (full features)
- **序列化**: Serde 1.x (derive feature) - 仅用于内部数据处理
- **传输协议**: Protocol Buffers (Protobuf) - HTTP 接口使用 `application/x-protobuf` 传输
- **Protobuf 库**: prost 0.12 + prost-build 0.12
- **数据库**: SQLx 0.7 (MySQL, runtime-tokio, tls-native-tls)
- **日志**: Tracing 0.1 + tracing-subscriber 0.3
- **环境变量**: Dotenv 0.15
- **认证**: JWT (jsonwebtoken) - Bearer Token 认证
- **密码加密**: Bcrypt - 用户密码哈希
- **缓存**: Redis - 会话管理、分布式锁
- **中间件**: Tower + Tower-HTTP - CORS、日志、静态文件服务

### 通信协议架构

#### Protobuf 传输层

- **API 层**: RESTful API 使用 Protobuf 二进制格式传输
- **Content-Type**: `application/x-protobuf`
- **Proto 定义**: 所有 API 的请求/响应结构在 `proto/` 目录定义
- **代码生成**: 使用 `prost-build` 在编译时生成 Rust 代码

#### 数据处理流程

```
客户端 (iOS)
    ↓ HTTP POST/GET (Protobuf 二进制)
Controller Layer (Axum Handler)
    ↓ 1. 反序列化 Protobuf → Proto Struct
    ↓ 2. Proto Struct → Entity Struct (内部数据模型)
Service Layer (业务逻辑)
    ↓ 3. 数据库操作 (Entity ↔ Database)
Database (MySQL)
    ↑ 4. 查询结果 → Entity Struct
Service Layer
    ↑ 5. Entity Struct → Proto Struct
Controller Layer
    ↑ 6. 序列化 Proto Struct → Protobuf 二进制
客户端 (iOS)
```

### 数据库规范

- 使用 MySQL 作为数据库
- 使用 SQLx 进行数据库操作（异步、编译时检查）
- 使用 SQLx 的宏进行查询：`sqlx::query!`, `sqlx::query_as!`
- 连接池管理使用 `sqlx::MySqlPool`
- 复杂类型（如数组）在数据库中存储为 JSON 字符串
- **数据库表结构可能与 API 接口不完全一致**，根据业务需要设计

## 项目架构模式

### 目录结构

```
campus_backend/
├── src/
│   ├── main.rs              # 应用入口
│   ├── common/              # 公共模块
│   │   ├── db.rs           # 数据库连接池
│   │   ├── error.rs        # 统一错误处理
│   │   └── mod.rs          # 模块导出
│   ├── modules/             # 业务模块
│   │   ├── course/         # 课表模块
│   │   │   ├── entity.rs      # 数据实体定义 (数据库映射 + 内部 DTO)
│   │   │   ├── service.rs     # 业务逻辑层
│   │   │   ├── controller.rs  # 控制器/路由层 (Protobuf 处理)
│   │   │   └── mod.rs         # 模块导出
│   │   ├── user/           # 用户模块
│   │   ├── bbs/            # 论坛模块
│   │   ├── activity/       # 活动模块
│   │   └── mod.rs
│   └── ffi/                # UniFFI 跨语言调用 (如违禁词检测)
├── proto/                   # Protobuf 定义文件
│   ├── course.proto        # 课表模块 Proto 定义
│   └── ...
├── migrations/             # 数据库迁移文件
├── docs/                   # API 文档
├── build.rs                # Protobuf 编译配置
└── Cargo.toml             # 依赖配置
```

### 分层架构 (四层架构 + Protobuf)

#### 1. Proto Layer (proto/\*.proto)

- 定义所有 API 的请求/响应消息结构
- 使用 Protocol Buffers 语法 (proto3)
- 生成的 Rust 代码用于网络传输
- 参考文件：`proto/course.proto`
- **Proto 结构与数据库表结构可能不同**

#### 2. Entity Layer (entity.rs)

- **数据库实体**: 使用 `#[derive(Debug, Clone, sqlx::FromRow)]` 映射数据库表
- **内部 DTO**: 用于 Service 层业务逻辑的数据传递
- 字段命名使用 snake_case
- 可选字段使用 `Option<T>`
- **不直接用于网络传输**，仅用于内部数据处理
- **数据库表可能包含额外字段**（如 created_at, updated_at, user_id 等）
- **数据库表可能拆分或合并 API 实体**（如分离主表和关联表）

#### 3. Service Layer (service.rs)

- 包含所有业务逻辑
- 数据库操作在此层执行
- 接收 `&MySqlPool` 作为数据库连接
- 返回 `Result<T, AppError>` 类型
- 处理数据验证、业务规则、事务管理
- **输入/输出使用内部 Entity 结构**，不直接处理 Protobuf
- **注意 Rust 所有权**：避免不必要的 clone，使用引用传递
- 函数签名示例：
  ```rust
  pub async fn get_courses(
      pool: &MySqlPool,
      params: GetCoursesParams,
  ) -> Result<CourseListResponse, AppError>
  ```

#### 4. Controller Layer (controller.rs)

- 定义 Axum 路由处理函数
- **负责 Protobuf 序列化/反序列化**
- 使用 Axum 的 extractors: `Body`, `State`, `Extension`
- 数据流程：
  1. 接收 Protobuf 二进制数据 (`application/x-protobuf`)
  2. 使用 `prost::Message::decode()` 反序列化为 Proto 消息
  3. 转换 Proto 消息为内部 Entity 结构
  4. 调用 Service 层函数
  5. 将 Service 返回的 Entity 转换为 Proto 消息
  6. 使用 `encode_to_vec()` 序列化为 Protobuf 二进制返回
- 路由函数示例：

  ```rust
  pub async fn get_courses_handler(
      State(pool): State<MySqlPool>,
      body: Bytes,
  ) -> Result<impl IntoResponse, AppError> {
      use prost::Message;

      // 1. 反序列化 Protobuf 请求
      let proto_req = GetPublicCoursesRequest::decode(body)
          .map_err(|e| AppError::BadRequest(format!("Invalid protobuf: {}", e)))?;

      // 2. 转换为内部结构
      let params = proto_to_entity(proto_req);

      // 3. 调用 Service
      let result = service::get_courses(&pool, params).await?;

      // 4. 转换为 Proto 响应
      let proto_resp = entity_to_proto(result);

      // 5. 序列化返回
      let bytes = proto_resp.encode_to_vec();
      Ok((
          StatusCode::OK,
          [(header::CONTENT_TYPE, "application/x-protobuf")],
          bytes
      ))
  }
  ```

## Protobuf 集成规范

### Cargo 依赖配置

```toml
[dependencies]
prost = "0.12"           # Protobuf 运行时
prost-types = "0.12"     # Protobuf 标准类型
bytes = "1"              # 字节处理

[build-dependencies]
prost-build = "0.12"     # Protobuf 代码生成
```

### build.rs 配置

```rust
fn main() {
    prost_build::compile_protos(
        &["proto/course.proto"],
        &["proto/"],
    ).unwrap();
}
```

### Protobuf 消息转换

#### Proto → Entity (Controller → Service)

```rust
// 在 controller.rs 中
fn proto_to_entity(proto: GetPublicCoursesRequest) -> GetCoursesParams {
    GetCoursesParams {
        semester_id: proto.semester_id,
        name: proto.name,
        teacher: proto.teacher,
        page: proto.page,
        page_size: proto.page_size,
    }
}
```

#### Entity → Proto (Service → Controller)

```rust
// 在 controller.rs 中
fn entity_to_proto(entity: CourseListResponse) -> GetPublicCoursesResponse {
    GetPublicCoursesResponse {
        code: 200,
        message: "Success".to_string(),
        data: Some(GetPublicCoursesData {
            list: entity.courses.into_iter().map(|c| PublicCourse {
                id: c.id,
                course_name: c.course_name,
                // ... 其他字段转换
            }).collect(),
            pagination: Some(Pagination {
                total: entity.total,
                page: entity.page,
                page_size: entity.page_size,
                pages: entity.pages,
            }),
        }),
    }
}
```

### API 响应格式规范

所有响应都遵循 Protobuf 定义的消息结构：

```protobuf
message GetPublicCoursesResponse {
  int32 code = 1;
  string message = 2;
  GetPublicCoursesData data = 3;
}
```

### 成功响应

- `code`: 200
- `message`: 描述性消息
- `data`: 实际数据（Protobuf 消息）

### 错误响应

- `code`: HTTP 状态码 (400, 401, 404, 500)
- `message`: 错误描述
- `data`: 空消息或 null

## 错误处理规范

### 自定义错误类型 (common/error.rs)

```rust
#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
}

// 实现 IntoResponse for AppError
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 返回统一的 JSON 错误响应
    }
}
```

## 数据库操作规范

### 查询示例

```rust
// 使用 query_as! 宏进行类型安全查询
let courses = sqlx::query_as!(
    PublicCourse,
    r#"
    SELECT id, course_name, teacher_name, location, day_of_week,
           start_section, end_section, weeks_range, type, credits, description
    FROM public_courses
    WHERE semester_id = ?
    "#,
    semester_id
)
.fetch_all(pool)
.await?;
```

### 插入示例

```rust
let result = sqlx::query!(
    r#"
    INSERT INTO schedule_items
    (user_id, source_id, course_name, day_of_week, start_section, end_section)
    VALUES (?, ?, ?, ?, ?, ?)
    "#,
    user_id, source_id, course_name, day_of_week, start_section, end_section
)
.execute(pool)
.await?;

let item_id = result.last_insert_id();
```

## 课表模块具体需求

### 数据模型

参考 `#[[file:proto/course.proto]]` 和 `#[[file:docs/课表模块.md]]`

#### 核心实体

1. **Semester** - 学期信息
2. **PublicCourse** - 全校课程
3. **ScheduleItem** - 用户课表项

### API 端点

1. **GET /api/v1/semesters** - 获取学期列表
2. **GET /api/v1/courses** - 获取全校课程列表（支持分页、筛选）
3. **GET /api/v1/schedule** - 获取用户课表
4. **POST /api/v1/schedule** - 批量添加课表项
5. **PATCH /api/v1/schedule** - 更新课表项
6. **DELETE /api/v1/schedule** - 删除课表项

### 业务规则

1. **时间冲突检测**: 添加/更新课表项时检查同一用户的时间冲突
2. **批量操作**: POST 支持批量添加，返回成功和失败列表
3. **自定义课程**: `is_custom` 和 `source_id` 互斥
4. **分页**: 使用 `page` 和 `pageSize` 参数
5. **周次范围**: 使用 JSON 数组存储，如 `[1,2,3,4,5]`

## 测试规范

### 单元测试要求

1. **测试位置**: 在每个文件底部使用 `#[cfg(test)]` 模块
2. **测试框架**: 使用 Rust 内置的 `#[test]` 和 `#[tokio::test]`
3. **Mock 数据库**: 可以使用 SQLx 的测试功能或 mock 对象

### 测试覆盖范围

#### Service 层测试

- 测试每个公共函数
- 测试正常流程和错误流程
- 测试边界条件
- 测试业务规则（如时间冲突检测）

#### Entity 层测试

- 测试序列化/反序列化
- 测试数据验证逻辑

### 测试示例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_schedule_item_success() {
        // Arrange
        let pool = setup_test_db().await;
        let item = ScheduleItemInput { /* ... */ };

        // Act
        let result = add_schedule_item(&pool, 1, item).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_time_conflict_detection() {
        // 测试时间冲突检测逻辑
    }
}
```

## Rust 所有权与借用规范

### 所有权原则

1. **避免不必要的 clone**:

   ```rust
   // ❌ 不好：不必要的 clone
   fn process_course(course: PublicCourse) -> String {
       course.course_name.clone()
   }

   // ✅ 好：使用引用
   fn process_course(course: &PublicCourse) -> &str {
       &course.course_name
   }
   ```

2. **函数参数使用引用**:

   ```rust
   // Service 层函数接收引用
   pub async fn get_courses(
       pool: &MySqlPool,  // 引用，不获取所有权
       params: &GetCoursesParams,  // 引用
   ) -> Result<CourseListResponse, AppError>
   ```

3. **返回值的所有权**:

   ```rust
   // 返回新创建的数据，转移所有权
   pub async fn create_schedule_item(
       pool: &MySqlPool,
       item: ScheduleItemInput,  // 获取所有权，因为会消费
   ) -> Result<ScheduleItem, AppError>  // 返回所有权
   ```

4. **集合迭代使用引用**:

   ```rust
   // ❌ 不好：消费集合
   for item in items {
       process(item);
   }

   // ✅ 好：借用集合
   for item in &items {
       process(item);
   }

   // ✅ 好：需要所有权时使用 into_iter
   for item in items.into_iter() {
       consume(item);
   }
   ```

5. **字符串处理**:

   ```rust
   // ❌ 不好：不必要的 String 分配
   fn get_message() -> String {
       "Success".to_string()
   }

   // ✅ 好：使用 &str
   fn get_message() -> &'static str {
       "Success"
   }

   // ✅ 需要动态字符串时才用 String
   fn format_message(name: &str) -> String {
       format!("Hello, {}", name)
   }
   ```

### 生命周期注意事项

1. **结构体中的引用需要生命周期标注**:

   ```rust
   // 如果 Entity 包含引用
   pub struct CourseRef<'a> {
       pub name: &'a str,
       pub teacher: &'a str,
   }
   ```

2. **避免在跨 await 点使用引用**:

   ```rust
   // ❌ 不好：引用跨越 await
   let item = &items[0];
   some_async_fn().await;
   process(item);  // 可能导致生命周期问题

   // ✅ 好：clone 或重新获取
   let item = items[0].clone();
   some_async_fn().await;
   process(&item);
   ```

## 数据库设计规范

### 数据库表与 API 实体的关系

**重要原则**: 数据库表结构不必与 API 接口的 Protobuf 定义完全一致，应根据业务需求和性能优化进行设计。

### 常见设计模式

#### 1. 表结构包含额外字段

API 实体可能只暴露部分字段，数据库表包含更多元数据：

```sql
-- 数据库表
CREATE TABLE schedule_items (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL,              -- API 不暴露，通过 JWT 获取
    semester_id BIGINT NOT NULL,          -- API 不暴露，通过查询参数获取
    source_id BIGINT,                     -- API 暴露
    course_name VARCHAR(255) NOT NULL,    -- API 暴露
    -- ... 其他 API 字段
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,  -- 元数据，API 不暴露
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    is_deleted BOOLEAN DEFAULT FALSE,     -- 软删除标记
    INDEX idx_user_semester (user_id, semester_id),
    INDEX idx_source (source_id)
);
```

对应的 Entity:

```rust
// 数据库完整实体
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ScheduleItemDb {
    pub id: i64,
    pub user_id: i64,
    pub semester_id: i64,
    pub source_id: Option<i64>,
    pub course_name: String,
    // ... 其他字段
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub is_deleted: bool,
}

// API 响应实体（不包含内部字段）
#[derive(Debug, Clone)]
pub struct ScheduleItem {
    pub id: i64,
    pub source_id: Option<i64>,
    pub course_name: String,
    // ... 只包含 API 需要的字段
}

// 转换函数
impl From<ScheduleItemDb> for ScheduleItem {
    fn from(db: ScheduleItemDb) -> Self {
        Self {
            id: db.id,
            source_id: db.source_id,
            course_name: db.course_name,
            // 不包含 user_id, created_at 等内部字段
        }
    }
}
```

#### 2. 表结构拆分（一对多关系）

API 可能返回嵌套结构，但数据库使用关联表：

```sql
-- 主表
CREATE TABLE public_courses (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    semester_id BIGINT NOT NULL,
    course_name VARCHAR(255) NOT NULL,
    teacher_name VARCHAR(100),
    location VARCHAR(100),
    day_of_week INT,
    start_section INT,
    end_section INT,
    type VARCHAR(20),
    credits INT,
    description TEXT
);

-- 周次范围单独存储（一对多）
CREATE TABLE course_weeks (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    course_id BIGINT NOT NULL,
    week_number INT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES public_courses(id),
    INDEX idx_course (course_id)
);
```

或者使用 JSON 字段（MySQL 5.7+）：

```sql
CREATE TABLE public_courses (
    -- ... 其他字段
    weeks_range JSON NOT NULL,  -- 存储为 JSON 数组 [1,2,3,4,5]
);
```

#### 3. 表结构合并（减少 JOIN）

API 可能分离的数据，数据库为性能考虑合并存储：

```sql
-- 合并存储用户课表和课程信息（避免频繁 JOIN）
CREATE TABLE schedule_items (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    source_id BIGINT,  -- 关联 public_courses，但冗余存储常用字段
    -- 冗余字段，避免每次查询都 JOIN
    course_name VARCHAR(255) NOT NULL,
    teacher_name VARCHAR(100),
    location VARCHAR(100),
    -- ... 其他冗余字段
);
```

### 数据库设计最佳实践

1. **索引设计**:

   - 为常用查询条件添加索引
   - 复合索引考虑查询顺序
   - 避免过多索引影响写入性能

2. **JSON 字段使用**:

   - 适合存储数组、动态结构
   - MySQL 5.7+ 支持 JSON 类型和查询
   - 使用 `sqlx::types::Json<Vec<i32>>` 映射

3. **软删除**:

   - 使用 `is_deleted` 标记而非物理删除
   - 查询时添加 `WHERE is_deleted = FALSE`

4. **时间戳**:

   - `created_at`: 记录创建时间
   - `updated_at`: 记录更新时间
   - 使用 `chrono::NaiveDateTime` 映射

5. **外键约束**:
   - 开发环境使用外键保证数据一致性
   - 生产环境根据性能需求决定是否使用

## 代码风格规范

1. **命名约定**:

   - 函数: snake_case
   - 结构体: PascalCase
   - 常量: SCREAMING_SNAKE_CASE
   - 生命周期: 单个小写字母 `'a`, `'b`

2. **异步函数**: 所有 I/O 操作使用 `async/await`

3. **错误处理**:

   - 使用 `?` 操作符传播错误
   - 不使用 `unwrap()` 或 `expect()`（除非在测试中）
   - 使用 `map_err()` 转换错误类型

4. **文档注释**:

   - 为公共 API 添加 `///` 文档注释
   - 为模块添加 `//!` 模块级注释

5. **代码组织**:
   - 先 imports (标准库 → 外部 crate → 本地模块)
   - 然后 constants
   - 然后 types/structs
   - 最后 functions
   - 测试放在文件底部 `#[cfg(test)]`

## 实现步骤

### 准备阶段

1. **配置 Protobuf**:
   - 添加 `prost` 相关依赖到 `Cargo.toml`
   - 创建 `build.rs` 配置 Protobuf 编译
   - 确认 `proto/course.proto` 定义完整

### 核心实现

2. **第一步**: 实现 `common/error.rs` - 统一错误处理
3. **第二步**: 实现 `common/db.rs` - 数据库连接池
4. **第三步**: 实现 `common/proto.rs` - Protobuf 工具函数
5. **第四步**: 实现 `modules/course/entity.rs` - 数据实体（数据库映射）
6. **第五步**: 实现 `modules/course/service.rs` - 业务逻辑
7. **第六步**: 实现 `modules/course/controller.rs` - 路由处理 + Protobuf 转换
8. **第七步**: 更新 `modules/course/mod.rs` - 模块导出
9. **第八步**: 更新 `main.rs` - 注册路由

### 测试阶段

10. **第九步**: 添加单元测试（Service 层）
11. **第十步**: 添加集成测试（Controller 层 + Protobuf）

## 注意事项

### 数据库相关

1. 所有数据库操作必须是异步的
2. 使用连接池而不是单个连接
3. 周次范围 `weeks_range` 在数据库中存储为 JSON 字符串
4. 使用 SQLx 的编译时检查功能

### Protobuf 相关

5. **Controller 层负责所有 Protobuf 序列化/反序列化**
6. **Service 层不直接处理 Protobuf**，使用内部 Entity 结构
7. Proto 消息和 Entity 结构需要明确的转换函数
8. Content-Type 必须设置为 `application/x-protobuf`
9. 使用 `prost::Message` trait 的 `decode()` 和 `encode_to_vec()` 方法

### 代码质量

10. 正确处理 Option 和 Result 类型
11. 遵循 Rust 的所有权和借用规则
12. 使用 `#[derive(Debug)]` 便于调试
13. 颜色值 `color_hex` 需要验证格式 (#RRGGBB)
14. 分页计算: `pages = (total + pageSize - 1) / pageSize`

### 数据转换

15. Proto 的 `repeated` 字段对应 Rust 的 `Vec<T>`
16. Proto 的 `optional` 字段对应 Rust 的 `Option<T>`
17. Proto 的 `int32` 对应 Rust 的 `i32`
18. Proto 的 `int64` 对应 Rust 的 `i64`
19. Proto 的 `string` 对应 Rust 的 `String`
20. Proto 的 `bool` 对应 Rust 的 `bool`

## 数据库操作示例

### JSON 字段处理

```rust
use sqlx::types::Json;

// 查询包含 JSON 字段
let courses = sqlx::query!(
    r#"
    SELECT id, course_name, weeks_range as "weeks_range: Json<Vec<i32>>"
    FROM public_courses
    WHERE semester_id = ?
    "#,
    semester_id
)
.fetch_all(pool)
.await?;

// 插入 JSON 字段
let weeks_json = Json(vec![1, 2, 3, 4, 5]);
sqlx::query!(
    r#"
    INSERT INTO public_courses (course_name, weeks_range)
    VALUES (?, ?)
    "#,
    course_name,
    weeks_json
)
.execute(pool)
.await?;
```

### 事务处理

```rust
use sqlx::Transaction;

pub async fn add_schedule_items_batch(
    pool: &MySqlPool,
    user_id: i64,
    items: Vec<ScheduleItemInput>,
) -> Result<Vec<ScheduleItem>, AppError> {
    // 开启事务
    let mut tx: Transaction<'_, sqlx::MySql> = pool.begin().await?;

    let mut results = Vec::new();

    for item in items {
        // 在事务中执行操作
        let result = sqlx::query!(
            r#"
            INSERT INTO schedule_items (user_id, course_name, day_of_week)
            VALUES (?, ?, ?)
            "#,
            user_id, item.course_name, item.day_of_week
        )
        .execute(&mut *tx)
        .await?;

        results.push(ScheduleItem {
            id: result.last_insert_id() as i64,
            course_name: item.course_name,
            day_of_week: item.day_of_week,
        });
    }

    // 提交事务
    tx.commit().await?;

    Ok(results)
}
```

### 时间冲突检测

```rust
pub async fn check_time_conflict(
    pool: &MySqlPool,
    user_id: i64,
    semester_id: i64,
    day_of_week: i32,
    start_section: i32,
    end_section: i32,
    weeks: &[i32],
) -> Result<bool, AppError> {
    // 查询可能冲突的课程
    let conflicts = sqlx::query!(
        r#"
        SELECT id, weeks_range as "weeks_range: Json<Vec<i32>>",
               start_section, end_section
        FROM schedule_items
        WHERE user_id = ?
          AND semester_id = ?
          AND day_of_week = ?
          AND is_deleted = FALSE
          AND (
              (start_section <= ? AND end_section >= ?)
              OR (start_section <= ? AND end_section >= ?)
              OR (start_section >= ? AND end_section <= ?)
          )
        "#,
        user_id,
        semester_id,
        day_of_week,
        start_section, start_section,
        end_section, end_section,
        start_section, end_section
    )
    .fetch_all(pool)
    .await?;

    // 检查周次是否重叠
    for conflict in conflicts {
        let conflict_weeks: Vec<i32> = conflict.weeks_range.0;
        if weeks.iter().any(|w| conflict_weeks.contains(w)) {
            return Ok(true);  // 存在冲突
        }
    }

    Ok(false)  // 无冲突
}
```

## 完整示例：课表项添加流程

### 1. Proto 定义 (proto/course.proto)

```protobuf
message AddScheduleItemsRequest {
  int64 semester_id = 1;
  repeated ScheduleItemInput items = 2;
}

message ScheduleItemInput {
  optional int64 source_id = 1;
  string course_name = 2;
  int32 day_of_week = 3;
  int32 start_section = 4;
  int32 end_section = 5;
  repeated int32 weeks = 6;
  bool is_custom = 7;
}
```

### 2. Entity 定义 (entity.rs)

```rust
// 数据库实体
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ScheduleItemDb {
    pub id: i64,
    pub user_id: i64,
    pub semester_id: i64,
    pub source_id: Option<i64>,
    pub course_name: String,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks_range: sqlx::types::Json<Vec<i32>>,
    pub is_custom: bool,
    pub created_at: chrono::NaiveDateTime,
}

// API 响应实体
#[derive(Debug, Clone)]
pub struct ScheduleItem {
    pub id: i64,
    pub source_id: Option<i64>,
    pub course_name: String,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks_range: Vec<i32>,
    pub is_custom: bool,
}

// 内部输入 DTO
#[derive(Debug, Clone)]
pub struct ScheduleItemInput {
    pub source_id: Option<i64>,
    pub course_name: String,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks: Vec<i32>,
    pub is_custom: bool,
}
```

### 3. Service 实现 (service.rs)

```rust
use crate::common::AppError;
use super::entity::*;
use sqlx::{MySqlPool, types::Json};

pub async fn add_schedule_items(
    pool: &MySqlPool,
    user_id: i64,
    semester_id: i64,
    items: Vec<ScheduleItemInput>,
) -> Result<Vec<ScheduleItem>, AppError> {
    let mut results = Vec::new();

    for item in items {
        // 验证业务规则
        if item.is_custom && item.source_id.is_some() {
            return Err(AppError::BadRequest(
                "自定义课程不能有 source_id".to_string()
            ));
        }

        // 检查时间冲突
        let has_conflict = check_time_conflict(
            pool, user_id, semester_id,
            item.day_of_week, item.start_section, item.end_section,
            &item.weeks
        ).await?;

        if has_conflict {
            return Err(AppError::BadRequest(
                format!("课程 {} 存在时间冲突", item.course_name)
            ));
        }

        // 插入数据库
        let weeks_json = Json(item.weeks.clone());
        let result = sqlx::query!(
            r#"
            INSERT INTO schedule_items
            (user_id, semester_id, source_id, course_name, day_of_week,
             start_section, end_section, weeks_range, is_custom)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            user_id, semester_id, item.source_id, item.course_name,
            item.day_of_week, item.start_section, item.end_section,
            weeks_json, item.is_custom
        )
        .execute(pool)
        .await?;

        results.push(ScheduleItem {
            id: result.last_insert_id() as i64,
            source_id: item.source_id,
            course_name: item.course_name,
            day_of_week: item.day_of_week,
            start_section: item.start_section,
            end_section: item.end_section,
            weeks_range: item.weeks,
            is_custom: item.is_custom,
        });
    }

    Ok(results)
}
```

### 4. Controller 实现 (controller.rs)

```rust
use axum::{
    body::Bytes,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
};
use prost::Message;
use crate::common::AppError;
use super::{service, entity};

// 引入生成的 Proto 代码
mod proto {
    include!(concat!(env!("OUT_DIR"), "/campus.course.rs"));
}

pub async fn add_schedule_items_handler(
    State(pool): State<sqlx::MySqlPool>,
    // TODO: 从 JWT 中提取 user_id
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    // 1. 反序列化 Protobuf 请求
    let proto_req = proto::AddScheduleItemsRequest::decode(body)
        .map_err(|e| AppError::BadRequest(format!("Invalid protobuf: {}", e)))?;

    // 2. 转换为内部结构
    let items: Vec<entity::ScheduleItemInput> = proto_req.items
        .into_iter()
        .map(|item| entity::ScheduleItemInput {
            source_id: item.source_id,
            course_name: item.course_name,
            day_of_week: item.day_of_week,
            start_section: item.start_section,
            end_section: item.end_section,
            weeks: item.weeks,
            is_custom: item.is_custom,
        })
        .collect();

    // 3. 调用 Service（假设 user_id = 1）
    let user_id = 1; // TODO: 从 JWT 获取
    let results = service::add_schedule_items(
        &pool,
        user_id,
        proto_req.semester_id,
        items
    ).await?;

    // 4. 转换为 Proto 响应
    let proto_items: Vec<proto::ScheduleItem> = results
        .into_iter()
        .map(|item| proto::ScheduleItem {
            id: item.id,
            source_id: item.source_id,
            course_name: item.course_name,
            day_of_week: item.day_of_week,
            start_section: item.start_section,
            end_section: item.end_section,
            weeks: item.weeks_range,
            is_custom: item.is_custom,
            // ... 其他字段
        })
        .collect();

    let proto_resp = proto::AddScheduleItemsResponse {
        code: 200,
        message: "添加成功".to_string(),
        data: Some(proto::AddScheduleItemsData {
            successful_items: proto_items,
            failed_items: vec![],
        }),
    };

    // 5. 序列化返回
    let bytes = proto_resp.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes
    ))
}
```

## 关键要点总结

1. **Protobuf 只在 Controller 层处理**，Service 层使用内部 Entity
2. **数据库表结构可以与 API 不同**，根据业务需求设计
3. **注意 Rust 所有权**，避免不必要的 clone
4. **使用引用传递参数**，返回值转移所有权
5. **JSON 字段存储复杂类型**，使用 `sqlx::types::Json<T>`
6. **事务保证数据一致性**，批量操作使用事务
7. **业务规则在 Service 层验证**，如时间冲突检测
8. **错误处理使用 `?` 操作符**，统一返回 `Result<T, AppError>`
