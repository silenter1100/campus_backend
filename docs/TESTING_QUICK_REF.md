# 测试快速参考

## 🚀 快速开始

### 1. 安装 SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features mysql
```

### 2. 配置测试数据库

```bash
# Windows
setup_test_db.bat

# Linux/Mac
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS campus_test;"
export DATABASE_URL=mysql://root:password@localhost:3306/campus_test
sqlx migrate run
```

### 3. 运行测试

```bash
# Windows
run_tests.bat

# Linux/Mac
cargo test
```

## 📝 常用命令

### 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_get_semesters

# 显示输出
cargo test -- --nocapture

# 只编译不运行
cargo test --no-run
```

### 迁移

```bash
# 创建新迁移
sqlx migrate add migration_name

# 运行迁移
sqlx migrate run

# 回滚迁移
sqlx migrate revert

# 查看状态
sqlx migrate info
```

### 覆盖率

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## 📂 文件位置

- 测试文件: `tests/`
- 迁移文件: `migrations/`
- 测试配置: `.env.test`
- 文档: `docs/TESTING.md`

## 🔧 故障排除

### 测试失败：连接数据库

1. 检查 MySQL 是否运行
2. 检查 `.env.test` 中的密码
3. 确保测试数据库已创建

### 迁移失败

1. 检查 SQL 语法
2. 确保数据库连接正常
3. 查看 `sqlx migrate info`

## 📚 完整文档

- [测试指南](docs/TESTING.md)
- [迁移指南](docs/MIGRATIONS.md)
- [升级总结](docs/UPGRADE_SUMMARY.md)
