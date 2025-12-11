# API 测试指南

## 问题说明

Postman 可以发送 Protobuf 请求，但**无法正确解析 Protobuf 响应**。这是因为 Postman 对 Protobuf 的支持还不够完善。

## 解决方案

### 方案 1：Python 测试脚本 (推荐 ⭐)

**优点**：

- ✅ 完美支持 Protobuf 请求和响应
- ✅ 可以看到格式化的响应数据
- ✅ 支持自动化测试
- ✅ 可以自定义测试逻辑

**使用步骤**：

1. **安装依赖**

```bash
pip install requests protobuf
```

2. **编译 Proto 文件**

```bash
# Windows
protoc --python_out=. proto/course.proto

# 如果没有 protoc，下载：
# https://github.com/protocolbuffers/protobuf/releases
```

3. **运行测试**

```bash
python test_api.py
```

4. **自定义测试**
   编辑 `test_api.py` 中的 `main()` 函数，启用或禁用特定测试。

---

### 方案 2：命令行工具 (curl + protoc)

**适合快速测试单个接口**

#### GET 请求（获取学期列表）

```bash
# 发送请求并保存响应
curl http://localhost:3000/api/v1/semesters \
  -H "Accept: application/x-protobuf" \
  --output response.bin

# 解析响应
protoc --decode=campus.course.GetSemestersResponse proto/course.proto < response.bin
```

#### POST 请求（添加课表）

```bash
# 1. 创建 JSON 格式的请求数据
cat > request.json << EOF
{
  "semester_id": 1,
  "items": [
    {
      "source_id": 101,
      "course_name": "数据结构",
      "teacher_name": "张三",
      "location": "A101",
      "day_of_week": 1,
      "start_section": 1,
      "end_section": 2,
      "weeks": "1-16",
      "type": "必修",
      "credits": 3.0,
      "is_custom": false
    }
  ]
}
EOF

# 2. 编码为 Protobuf 二进制
cat request.json | protoc --encode=campus.course.AddScheduleItemsRequest proto/course.proto > request.bin

# 3. 发送请求
curl -X POST http://localhost:3000/api/v1/schedule \
  -H "Content-Type: application/x-protobuf" \
  --data-binary @request.bin \
  --output response.bin

# 4. 解析响应
protoc --decode=campus.course.AddScheduleItemsResponse proto/course.proto < response.bin
```

---

### 方案 3：Insomnia (图形界面工具)

**下载**：https://insomnia.rest/

**配置步骤**：

1. 安装 Insomnia
2. 创建新请求
3. Body 类型选择 "Protobuf"
4. 点击 "Select Proto File" 导入 `proto/course.proto`
5. 选择对应的 Message 类型
6. 填写字段值
7. 发送请求

**优点**：

- ✅ 原生支持 Protobuf
- ✅ 可以解析响应
- ✅ 图形界面友好

---

### 方案 4：Kreya (专业 API 测试工具)

**下载**：https://kreya.app/

**特点**：

- 专为 gRPC 和 Protobuf 设计
- 支持 HTTP REST + Protobuf
- 免费版功能完整

---

## 推荐方案对比

| 工具          | 易用性     | 响应解析  | 自动化      | 推荐度     |
| ------------- | ---------- | --------- | ----------- | ---------- |
| Python 脚本   | ⭐⭐⭐⭐   | ✅ 完美   | ✅ 支持     | ⭐⭐⭐⭐⭐ |
| Insomnia      | ⭐⭐⭐⭐⭐ | ✅ 完美   | ❌ 不支持   | ⭐⭐⭐⭐   |
| curl + protoc | ⭐⭐       | ✅ 完美   | ⭐⭐ 脚本化 | ⭐⭐⭐     |
| Postman       | ⭐⭐⭐⭐⭐ | ❌ 不支持 | ⭐⭐⭐      | ⭐⭐       |
| Kreya         | ⭐⭐⭐⭐   | ✅ 完美   | ❌ 不支持   | ⭐⭐⭐⭐   |

---

## 快速开始

**最简单的方式**：

1. 确保服务器运行：

```bash
cargo run
```

2. 运行 Python 测试脚本：

```bash
pip install requests protobuf
protoc --python_out=. proto/course.proto
python test_api.py
```

3. 查看格式化的响应输出 ✅

---

## 常见问题

### Q: protoc 命令找不到？

A: 下载并安装 Protocol Buffers 编译器：

- Windows: https://github.com/protocolbuffers/protobuf/releases
- 下载 `protoc-xx.x-win64.zip`
- 解压并添加到 PATH

### Q: Python 脚本报错 "找不到 course_pb2"？

A: 需要先编译 proto 文件：

```bash
protoc --python_out=. proto/course.proto
```

### Q: 为什么 Postman 不能解析响应？

A: Postman 的 Protobuf 支持还不完善，建议使用 Insomnia 或 Python 脚本。

### Q: 如何测试特定的接口？

A: 编辑 `test_api.py`，在 `main()` 函数中注释掉不需要的测试。

---

## 接口列表

| 方法   | 路径                | 说明           |
| ------ | ------------------- | -------------- |
| GET    | `/api/v1/semesters` | 获取学期列表   |
| GET    | `/api/v1/courses`   | 获取全校课程   |
| GET    | `/api/v1/schedule`  | 获取用户课表   |
| POST   | `/api/v1/schedule`  | 批量添加课表项 |
| PATCH  | `/api/v1/schedule`  | 更新课表项     |
| DELETE | `/api/v1/schedule`  | 删除课表项     |

所有接口都使用 `application/x-protobuf` 格式。
