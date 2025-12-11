# 测试环境快速配置

## 一键配置（Windows）

### 1. 安装 SQLx CLI

```bash
cargo install sqlx-cli --no-default-features --features mysql
```

### 2. 配置测试数据库

双击运行：

```
setup_test_db.bat
```

或手动执行：

```bash
# 创建测试数据库
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS campus_test;"

# 配置环境变量
set DATABASE_URL=mysql://root:your_password@localhost:3306/campus_test

# 运行迁移
sqlx migrate run
```

### 3. 运行测试

双击运行：

```
run_tests.bat
```

或手动执行：

```bash
cargo test
```

## 测试类型

### 单元测试（Service 层）

```bash
cargo test course_service_test
```

测试内容：

- ✅ 业务逻辑
- ✅ 数据验证
- ✅ 错误处理
- ✅ 边界条件

### 集成测试（API 层）

```bash
cargo test course_api_test
```

测试内容：

- ✅ HTTP 请求/响应
- ✅ Protobuf 序列化
- ✅ 路由功能
- ✅ 端到端流程

### 运行所有测试

```bash
cargo test --verbose
```

### 查看测试输出

```bash
cargo test -- --nocapture
```

## 测试覆盖率

```bash
# 安装工具
cargo install cargo-tarpaulin

# 生成报告
cargo tarpaulin --out Html
```

## 常见问题

### 测试失败：连接数据库

1. 确保 MySQL 正在运行
2. 检查 `.env.test` 中的密码
3. 确保测试数据库已创建

### 测试冲突

测试使用 `#[serial]` 标记，会串行执行，避免冲突。

### 清理测试数据

测试会自动清理数据，无需手动操作。

## 下一步

查看完整文档：

- [测试指南](./TESTING.md)
- [迁移指南](./MIGRATIONS.md)
- [API 测试指南](./TESTING_GUIDE.md)
