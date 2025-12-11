# 数据库迁移指南

## 什么是数据库迁移？

数据库迁移是一种版本控制系统，用于管理数据库 schema 的变更。它解决了你提到的问题：

- ✅ 代码和数据库结构同步
- ✅ 自动化表结构变更
- ✅ 支持回滚
- ✅ 团队协作时保持一致

## SQLx 迁移系统

### 安装 SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features mysql
```

### 配置

**文件**: `.sqlx/config.json`

```json
{
  "db": "mysql",
  "migrations": "./migrations"
}
```

## 迁移命令

### 1. 创建数据库

```bash
sqlx database create
```

这会根据 `DATABASE_URL` 创建数据库。

### 2. 删除数据库

```bash
sqlx database drop
```

### 3. 创建新迁移

```bash
sqlx migrate add <migration_name>
```

**示例**:

```bash
# 创建用户表迁移
sqlx migrate add create_users_table

# 添加索引
sqlx migrate add add_index_to_courses

# 修改列
sqlx migrate add modify_credits_column
```

这会在 `migrations/` 目录创建新文件：

```
migrations/20241210120000_create_users_table.sql
```

### 4. 运行迁移

```bash
# 运行所有待执行的迁移
sqlx migrate run

# 指定数据库 URL
sqlx migrate run --database-url mysql://user:pass@localhost/db
```

### 5. 回滚迁移

```bash
# 回滚最后一次迁移
sqlx migrate revert

# 回滚多次
sqlx migrate revert --target-version 20241210120000
```

### 6. 查看迁移状态

```bash
sqlx migrate info
```

输出示例：

```
Applied migrations:
  20240101000000_create_course_tables.sql (applied)
  20241210120000_create_users_table.sql (applied)

Pending migrations:
  20241210130000_add_index.sql (pending)
```

## 迁移文件格式

### 基本格式

```sql
-- migrations/20241210120000_create_users_table.sql

-- 创建用户表
CREATE TABLE users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_username (username),
    INDEX idx_email (email)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
```

### 可逆迁移（支持回滚）

SQLx 支持在同一文件中定义向上和向下迁移：

```sql
-- migrations/20241210120000_create_users_table.sql

-- 向上迁移（应用）
CREATE TABLE users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE
);

-- 向下迁移（回滚）
-- 使用特殊注释分隔
-- migrate:down
DROP TABLE users;
```

## 实际示例

### 示例 1：添加新表

```bash
sqlx migrate add create_comments_table
```

```sql
-- migrations/20241210120000_create_comments_table.sql

CREATE TABLE comments (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_user_id (user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
```

### 示例 2：添加列

```bash
sqlx migrate add add_avatar_to_users
```

```sql
-- migrations/20241210130000_add_avatar_to_users.sql

ALTER TABLE users
ADD COLUMN avatar_url VARCHAR(255) AFTER email;

-- migrate:down
ALTER TABLE users DROP COLUMN avatar_url;
```

### 示例 3：修改列类型

```bash
sqlx migrate add modify_credits_precision
```

```sql
-- migrations/20241210140000_modify_credits_precision.sql

ALTER TABLE public_courses
MODIFY COLUMN credits DECIMAL(4,2);

-- migrate:down
ALTER TABLE public_courses
MODIFY COLUMN credits FLOAT;
```

### 示例 4：添加索引

```bash
sqlx migrate add add_course_name_index
```

```sql
-- migrations/20241210150000_add_course_name_index.sql

CREATE INDEX idx_course_name ON public_courses(course_name);

-- migrate:down
DROP INDEX idx_course_name ON public_courses;
```

### 示例 5：数据迁移

```bash
sqlx migrate add migrate_old_data
```

```sql
-- migrations/20241210160000_migrate_old_data.sql

-- 迁移旧数据到新表
INSERT INTO new_schedule_items (user_id, course_name, semester_id)
SELECT user_id, course_name, semester_id
FROM old_schedule_items
WHERE deleted_at IS NULL;

-- 可选：删除旧表
-- DROP TABLE old_schedule_items;
```

## 工作流程

### 开发新功能

1. **创建迁移**

```bash
sqlx migrate add add_feature_x
```

2. **编写 SQL**

```sql
-- migrations/20241210120000_add_feature_x.sql
CREATE TABLE feature_x (...);
```

3. **运行迁移**

```bash
sqlx migrate run
```

4. **更新代码**

```rust
// 在 entity.rs 中添加对应的结构体
#[derive(sqlx::FromRow)]
pub struct FeatureX {
    pub id: i64,
    // ...
}
```

5. **测试**

```bash
cargo test
```

### 修改现有表

1. **创建迁移**

```bash
sqlx migrate add modify_existing_table
```

2. **编写 ALTER 语句**

```sql
ALTER TABLE courses ADD COLUMN new_field VARCHAR(100);
```

3. **运行迁移**

```bash
sqlx migrate run
```

4. **更新代码中的结构体**

### 回滚错误的迁移

```bash
# 回滚最后一次迁移
sqlx migrate revert

# 删除迁移文件
rm migrations/20241210120000_bad_migration.sql

# 重新创建正确的迁移
sqlx migrate add correct_migration
```

## 团队协作

### 场景：多人开发

**开发者 A**:

```bash
# 创建新迁移
sqlx migrate add add_feature_a
git add migrations/
git commit -m "Add feature A migration"
git push
```

**开发者 B**:

```bash
# 拉取最新代码
git pull

# 运行新迁移
sqlx migrate run

# 现在数据库已同步
```

### 场景：解决迁移冲突

如果两个开发者同时创建迁移，可能会有时间戳冲突：

```bash
# 重命名迁移文件，调整时间戳
mv migrations/20241210120000_feature_b.sql \
   migrations/20241210120001_feature_b.sql
```

## 生产环境部署

### 1. 备份数据库

```bash
mysqldump -u root -p campus_prod > backup_$(date +%Y%m%d).sql
```

### 2. 运行迁移

```bash
# 设置生产数据库 URL
export DATABASE_URL=mysql://user:pass@prod-server/campus_prod

# 查看待执行的迁移
sqlx migrate info

# 运行迁移
sqlx migrate run
```

### 3. 验证

```bash
# 检查表结构
mysql -u root -p campus_prod -e "SHOW TABLES;"

# 运行测试
cargo test
```

### 4. 回滚（如果需要）

```bash
sqlx migrate revert
mysql -u root -p campus_prod < backup_20241210.sql
```

## 最佳实践

### 1. 迁移文件命名

```bash
# ✅ 好的命名
sqlx migrate add create_users_table
sqlx migrate add add_email_index_to_users
sqlx migrate add modify_credits_precision

# ❌ 不好的命名
sqlx migrate add update
sqlx migrate add fix
sqlx migrate add temp
```

### 2. 小步迁移

```bash
# ✅ 好：分多个迁移
sqlx migrate add create_table
sqlx migrate add add_indexes
sqlx migrate add add_foreign_keys

# ❌ 不好：一个巨大的迁移
sqlx migrate add big_change
```

### 3. 始终测试回滚

```bash
# 运行迁移
sqlx migrate run

# 测试回滚
sqlx migrate revert

# 再次运行
sqlx migrate run
```

### 4. 不要修改已应用的迁移

```bash
# ❌ 不要修改已经运行的迁移文件
# 如果需要修改，创建新的迁移

sqlx migrate add fix_previous_migration
```

### 5. 使用事务

SQLx 自动将每个迁移包装在事务中，如果失败会自动回滚。

### 6. 添加注释

```sql
-- migrations/20241210120000_complex_change.sql

-- 目的：优化课程查询性能
-- 影响：public_courses 表
-- 预计执行时间：< 1 秒（小表）

CREATE INDEX idx_course_name_teacher
ON public_courses(course_name, teacher_name);
```

## 与 ORM 对比

| 特性     | SQLx 迁移 | SeaORM | Diesel |
| -------- | --------- | ------ | ------ |
| 学习曲线 | 低        | 中     | 中     |
| 灵活性   | 高        | 中     | 中     |
| 类型安全 | 编译时    | 编译时 | 编译时 |
| 自动生成 | 否        | 是     | 是     |
| 原生 SQL | 是        | 部分   | 部分   |
| 性能     | 最高      | 高     | 高     |

**为什么选择 SQLx**:

- ✅ 完全控制 SQL
- ✅ 最佳性能
- ✅ 简单直接
- ✅ 编译时检查
- ✅ 适合复杂查询

## 常见问题

### Q: 如何查看已应用的迁移？

```bash
sqlx migrate info
```

### Q: 如何跳过某个迁移？

不建议跳过。如果必须：

```sql
-- 手动插入迁移记录
INSERT INTO _sqlx_migrations (version, description, success)
VALUES (20241210120000, 'skipped', 1);
```

### Q: 迁移失败了怎么办？

```bash
# 1. 查看错误信息
sqlx migrate run

# 2. 修复 SQL
# 编辑迁移文件

# 3. 手动清理（如果需要）
mysql -u root -p -e "DELETE FROM _sqlx_migrations WHERE version = 20241210120000;"

# 4. 重新运行
sqlx migrate run
```

### Q: 如何在代码中运行迁移？

```rust
// 在应用启动时自动运行迁移
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

## 总结

SQLx 迁移系统提供了：

- ✅ 版本控制的数据库 schema
- ✅ 自动化的表结构变更
- ✅ 支持回滚
- ✅ 团队协作友好
- ✅ 生产环境安全

这完全解决了你提到的"代码和数据库解耦"的问题！
