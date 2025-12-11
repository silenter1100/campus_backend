---
title: 活动模块
language_tabs:
  - shell: Shell
  - http: HTTP
  - javascript: JavaScript
  - ruby: Ruby
  - python: Python
  - php: PHP
  - java: Java
  - go: Go
toc_footers: []
includes: []
search: true
code_clipboard: true
highlight_theme: darkula
headingLevel: 2
generator: "@tarslib/widdershins v4.0.30"

---

# 活动模块

Base URLs:

# Authentication

- HTTP Authentication, scheme: bearer

# Default

## POST 管理员发布活动

POST /api/v1/activities

管理员发布活动

> Body 请求参数

```json
{
  "title": "string",
  "content": "string",
  "location": "string",
  "organizer": "string",
  "start_time": "2019-08-24T14:15:22Z",
  "end_time": "2019-08-24T14:15:22Z"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 是 | CreateActivityRequest|none|
|» title|body|string| 是 | 活动标题|none|
|» content|body|string| 是 | 活动主要内容|none|
|» location|body|string| 是 | 活动地点|none|
|» organizer|body|string| 是 | 主办单位|none|
|» start_time|body|string(date-time)| 是 | 活动开始时间|ISO 8601格式|
|» end_time|body|string(date-time)| 是 | 活动结束时间|ISO 8601格式|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": [
    {
      "id": "string",
      "title": "string",
      "content": "string",
      "cover_url": "string",
      "activity_type": 0,
      "location": "string",
      "organizer": "string",
      "start_time": "2019-08-24T14:15:22Z",
      "end_time": "2019-08-24T14:15:22Z",
      "quota": 0,
      "current_enrollments": 0,
      "need_sign_in": true,
      "status": 1,
      "created_at": "2019-08-24T14:15:22Z"
    }
  ]
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*BaseResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|业务状态码|通常200表示成功|
|» message|string|true|none|状态信息|对code的简要描述|
|» data|[[Activity](#schemaactivity)]|true|none||none|
|»» Activity|[Activity](#schemaactivity)|false|none|Activity|none|
|»»» id|string|true|none|活动唯一标识符|none|
|»»» title|string|true|none|活动标题|none|
|»»» content|string|true|none|活动主要内容/介绍|none|
|»»» cover_url|string|true|none|活动封面图URL|none|
|»»» activity_type|integer|true|none|活动类型|1:讲座, 2:社团, 3:竞赛|
|»»» location|string|true|none|活动地点|none|
|»»» organizer|string|true|none|主办单位/主办方信息|none|
|»»» start_time|string(date-time)|true|none|活动开始时间|none|
|»»» end_time|string(date-time)|true|none|活动结束时间|none|
|»»» quota|integer|true|none|报名人数限额|none|
|»»» current_enrollments|integer|true|none|当前报名人数|服务端实时统计|
|»»» need_sign_in|boolean|true|none|是否需要签到|仅作信息展示|
|»»» status|integer|true|none|活动状态|1: 已发布/进行中, 2: 已结束, 3: 已撤销|
|»»» created_at|string(date-time)|true|none|创建时间|none|

#### 枚举值

|属性|值|
|---|---|
|status|1|
|status|2|
|status|3|

## GET 获取活动列表

GET /api/v1/activities

获取活动列表供前端主页显示
由于只显示部分信息，返回部分信息
提供给学生和管理员

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|keyword|query|string| 否 ||none|
|activity_type|query|integer| 否 ||none|
|page|query|integer| 否 ||none|
|pageSize|query|integer| 否 ||none|

#### 枚举值

|属性|值|
|---|---|
|activity_type|1|
|activity_type|2|
|activity_type|3|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "list": [
      {
        "id": "string",
        "title": "string",
        "cover_url": "string",
        "location": "string",
        "start_time": "2019-08-24T14:15:22Z",
        "quota": 0,
        "current_enrollments": 0
      }
    ],
    "pagination": {
      "total": 0,
      "page": 0,
      "pageSize": 0,
      "pages": 0
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*GetActivitiesListResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» list|[object]|true|none|Activity 列表|none|
|»»» id|string|true|none||none|
|»»» title|string|true|none||none|
|»»» cover_url|string|true|none||none|
|»»» location|string|true|none||none|
|»»» start_time|string(date-time)|true|none||none|
|»»» quota|integer|true|none|上限|none|
|»»» current_enrollments|integer|true|none|当前报名人数|none|
|»» pagination|object|true|none||none|
|»»» total|integer|true|none|总记录数|none|
|»»» page|integer|true|none|当前页码|none|
|»»» pageSize|integer|true|none|每页限制数|none|
|»»» pages|integer|true|none|总页数|none|

## GET 管理员查看活动报名参与信息

GET /api/v1/activities/{activity_id}/enrollments

获取指定活动的所有报名学生信息和参与统计。

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|activity_id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "total_enrolled": 0,
    "enrollment_list": [
      {
        "user_id": "string",
        "user_name": "string",
        "student_id": "string",
        "major": "string",
        "phone_number": "string",
        "activity_id": "string",
        "enroll_time": "2019-08-24T14:15:22Z",
        "attendance_status": 1
      }
    ]
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*GetEnrollmentsResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» total_enrolled|integer|true|none|报名总数|none|
|»» enrollment_list|[object]|true|none|报名学生列表 (EnrollmentRecord)|none|
|»»» user_id|string|true|none||none|
|»»» user_name|string|true|none||none|
|»»» student_id|string|true|none||none|
|»»» major|string|true|none||none|
|»»» phone_number|string¦null|false|none||none|
|»»» activity_id|string|true|none||none|
|»»» enroll_time|string(date-time)|true|none||none|
|»»» attendance_status|integer|true|none||none|

#### 枚举值

|属性|值|
|---|---|
|attendance_status|1|
|attendance_status|2|

## PATCH 修改/撤销活动

PATCH /api/v1/activities/{activity_id}

修改撤销活动

> Body 请求参数

```json
{
  "title": "string",
  "content": "string",
  "cover_url": "string",
  "activity_type": 1,
  "location": "string",
  "organizer": "string",
  "start_time": "2019-08-24T14:15:22Z",
  "end_time": "2019-08-24T14:15:22Z",
  "quota": 0,
  "need_sign_in": true,
  "status": 1
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|activity_id|path|string| 是 ||none|
|body|body|object| 是 | PatchActivityRequest|none|
|» title|body|string| 否 ||none|
|» content|body|string| 否 ||none|
|» cover_url|body|string| 否 ||none|
|» activity_type|body|integer| 否 ||none|
|» location|body|string| 否 ||none|
|» organizer|body|string| 否 ||none|
|» start_time|body|string(date-time)| 否 ||none|
|» end_time|body|string(date-time)| 否 ||none|
|» quota|body|integer| 否 ||none|
|» need_sign_in|body|boolean| 否 ||none|
|» status|body|integer| 否 ||none|

#### 枚举值

|属性|值|
|---|---|
|» activity_type|1|
|» activity_type|2|
|» activity_type|3|
|» status|1|
|» status|2|
|» status|3|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": "null"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*SimpleSuccessResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object¦null|false|none||none|

## GET 学生获取活动详情

GET /api/v1/activities/{activity_id}

获取活动详情

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|activity_id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "id": "string",
    "title": "string",
    "content": "string",
    "cover_url": "string",
    "activity_type": 0,
    "location": "string",
    "organizer": "string",
    "start_time": "2019-08-24T14:15:22Z",
    "end_time": "2019-08-24T14:15:22Z",
    "quota": 0,
    "current_enrollments": 0,
    "need_sign_in": true,
    "status": 0,
    "created_at": "2019-08-24T14:15:22Z",
    "is_enrolled": true,
    "is_collected": true
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*GetActivityDetailResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none|完整的活动详情 (Activity)|none|
|»» id|string|true|none||none|
|»» title|string|true|none||none|
|»» content|string|true|none||none|
|»» cover_url|string|true|none||none|
|»» activity_type|integer|true|none||none|
|»» location|string|true|none||none|
|»» organizer|string|true|none||none|
|»» start_time|string(date-time)|true|none||none|
|»» end_time|string(date-time)|true|none||none|
|»» quota|integer|true|none||none|
|»» current_enrollments|integer|true|none||none|
|»» need_sign_in|boolean|true|none||none|
|»» status|integer|true|none||none|
|»» created_at|string(date-time)|true|none||none|
|»» is_enrolled|boolean|true|none|用户是否已报名|根据Token判断|
|»» is_collected|boolean|true|none|用户是否已收藏|根据Token判断|

## POST 学生报名参加活动

POST /api/v1/activities/{activity_id}/enroll

报名参加活动

> Body 请求参数

```json
{
  "user_name": "string",
  "student_id": "string",
  "major": "string",
  "phone_number": "string"
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|activity_id|path|string| 是 ||none|
|body|body|object| 是 | EnrollActivityRequest|none|
|» user_name|body|string| 是 | 姓名|可自动填充|
|» student_id|body|string| 是 | 学号|可自动填充|
|» major|body|string| 是 | 学院/专业|可自动填充|
|» phone_number|body|string¦null| 否 | 手机号|可选|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": "null"
}
```

> 404 Response

```json
{
  "code": 404,
  "message": "指定的活动不存在。",
  "data": null
}
```

```json
{
  "code": 4091,
  "message": "活动报名人数已满，无法报名。",
  "data": null
}
```

```json
{
  "code": 4092,
  "message": "您已报名该活动，请勿重复操作。",
  "data": null
}
```

```json
{
  "code": 4093,
  "message": "活动报名已截止或已开始，无法报名。",
  "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|none|Inline|
|409|[Conflict](https://tools.ietf.org/html/rfc7231#section-6.5.8)|none|Inline|

### 返回数据结构

状态码 **200**

*SimpleSuccessResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object¦null|false|none||none|

状态码 **404**

*404 Not Found 响应*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||HTTP 状态码|
|» message|string|true|none||错误提示，指示资源不存在。|
|» data|object¦null|false|none||错误时 data 字段通常为 null。|

状态码 **409**

*409 资源冲突响应*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object¦null|false|none||none|

## DELETE 学生取消报名

DELETE /api/v1/activities/{activity_id}/enroll

取消报名

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|activity_id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": "null"
}
```

> 400 Response

```json
{
  "code": 400,
  "message": "活动取消已截止，无法取消报名。",
  "data": null
}
```

> 404 Response

```json
{
  "code": 404,
  "message": "您尚未报名该活动，无法取消。",
  "data": null
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|none|Inline|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|none|Inline|

### 返回数据结构

状态码 **200**

*SimpleSuccessResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object¦null|false|none||none|

状态码 **400**

*404 Not Found 响应*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||HTTP 状态码|
|» message|string|true|none||参数错误|
|» data|object¦null|false|none||错误时 data 字段通常为 null。|

状态码 **404**

*404 Not Found 响应*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||HTTP 状态码|
|» message|string|true|none||错误提示，指示资源不存在。|
|» data|object¦null|false|none||错误时 data 字段通常为 null。|

## POST 学生收藏活动

POST /api/v1/activities/{activity_id}/collect

收藏活动

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|activity_id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": "null"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*SimpleSuccessResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object¦null|false|none||none|

## DELETE 学生取消收藏

DELETE /api/v1/activities/{activity_id}/collect

取消收藏

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|activity_id|path|string| 是 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": "null"
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*SimpleSuccessResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object¦null|false|none||none|

## GET 我的活动{报名 + 收藏 }

GET /api/v1/my/activities

我的活动，包含报名和收藏

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|include_enrollments|query|boolean| 否 ||报名列表|
|include_collections|query|boolean| 否 ||收藏列表|
|page|query|integer| 否 ||当前页码|
|pageSize|query|integer| 否 ||每页条数|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "enrolled_data": {
      "pagination": {
        "total": 0,
        "page": 0,
        "pageSize": 0,
        "pages": 0
      },
      "list": [
        {
          "activity_id": "string",
          "title": "string",
          "cover_url": "string",
          "start_time": "2019-08-24T14:15:22Z",
          "end_time": "2019-08-24T14:15:22Z",
          "my_status": "["
        }
      ]
    },
    "collected_data": {
      "pagination": {
        "total": 0,
        "page": 0,
        "pageSize": 0,
        "pages": 0
      },
      "list": [
        {
          "activity_id": "string",
          "title": "string",
          "cover_url": "string",
          "start_time": "2019-08-24T14:15:22Z",
          "end_time": "2019-08-24T14:15:22Z"
        }
      ]
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|

### 返回数据结构

状态码 **200**

*GetMyActivitiesDecoupledResponse*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|false|none||none|
|»» enrolled_data|object|true|none|我的报名活动数据|none|
|»»» pagination|object|true|none||none|
|»»»» total|integer|true|none|总记录数|none|
|»»»» page|integer|true|none|当前页码|none|
|»»»» pageSize|integer|true|none|每页限制数|none|
|»»»» pages|integer|true|none|总页数|none|
|»»» list|[[EnrollmentResponse](#schemaenrollmentresponse)]|true|none|我的报名活动列表|none|
|»»»» activity_id|string|true|none||none|
|»»»» title|string|true|none||none|
|»»»» cover_url|string|true|none||none|
|»»»» start_time|string(date-time)|true|none||none|
|»»»» end_time|string(date-time)|true|none||none|
|»»»» my_status|integer|true|none||1:已报名, 2:已取消报名|
|»» collected_data|object|true|none|我的收藏活动数据|none|
|»»» pagination|object|true|none||none|
|»»»» total|integer|true|none|总记录数|none|
|»»»» page|integer|true|none|当前页码|none|
|»»»» pageSize|integer|true|none|每页限制数|none|
|»»»» pages|integer|true|none|总页数|none|
|»»» list|[[CollectionResponse](#schemacollectionresponse)]|true|none|我的收藏活动列表|none|
|»»»» activity_id|string|true|none||none|
|»»»» title|string|true|none||none|
|»»»» cover_url|string|true|none||none|
|»»»» start_time|string(date-time)|true|none||none|
|»»»» end_time|string(date-time)|true|none||none|

#### 枚举值

|属性|值|
|---|---|
|my_status|1|
|my_status|2|

# 数据模型

<h2 id="tocS_Activity">Activity</h2>

<a id="schemaactivity"></a>
<a id="schema_Activity"></a>
<a id="tocSactivity"></a>
<a id="tocsactivity"></a>

```json
{
  "id": "string",
  "title": "string",
  "content": "string",
  "cover_url": "string",
  "activity_type": 0,
  "location": "string",
  "organizer": "string",
  "start_time": "2019-08-24T14:15:22Z",
  "end_time": "2019-08-24T14:15:22Z",
  "quota": 0,
  "current_enrollments": 0,
  "need_sign_in": true,
  "status": 1,
  "created_at": "2019-08-24T14:15:22Z"
}

```

Activity

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|string|true|none|活动唯一标识符|none|
|title|string|true|none|活动标题|none|
|content|string|true|none|活动主要内容/介绍|none|
|cover_url|string|true|none|活动封面图URL|none|
|activity_type|integer|true|none|活动类型|1:讲座, 2:社团, 3:竞赛|
|location|string|true|none|活动地点|none|
|organizer|string|true|none|主办单位/主办方信息|none|
|start_time|string(date-time)|true|none|活动开始时间|none|
|end_time|string(date-time)|true|none|活动结束时间|none|
|quota|integer|true|none|报名人数限额|none|
|current_enrollments|integer|true|none|当前报名人数|服务端实时统计|
|need_sign_in|boolean|true|none|是否需要签到|仅作信息展示|
|status|integer|true|none|活动状态|1: 已发布/进行中, 2: 已结束, 3: 已撤销|
|created_at|string(date-time)|true|none|创建时间|none|

#### 枚举值

|属性|值|
|---|---|
|status|1|
|status|2|
|status|3|

<h2 id="tocS_Pagination">Pagination</h2>

<a id="schemapagination"></a>
<a id="schema_Pagination"></a>
<a id="tocSpagination"></a>
<a id="tocspagination"></a>

```json
{
  "total": 0,
  "page": 0,
  "pageSize": 0,
  "pages": 0
}

```

分页信息

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|total|integer|true|none|总记录数|none|
|page|integer|true|none|当前页码|none|
|pageSize|integer|true|none|每页限制数|none|
|pages|integer|true|none|总页数|none|

<h2 id="tocS_EnrollmentResponse">EnrollmentResponse</h2>

<a id="schemaenrollmentresponse"></a>
<a id="schema_EnrollmentResponse"></a>
<a id="tocSenrollmentresponse"></a>
<a id="tocsenrollmentresponse"></a>

```json
{
  "activity_id": "string",
  "title": "string",
  "cover_url": "string",
  "start_time": "2019-08-24T14:15:22Z",
  "end_time": "2019-08-24T14:15:22Z",
  "my_status": 1
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|activity_id|string|true|none||none|
|title|string|true|none||none|
|cover_url|string|true|none||none|
|start_time|string(date-time)|true|none||none|
|end_time|string(date-time)|true|none||none|
|my_status|integer|true|none||1:已报名, 2:已取消报名|

#### 枚举值

|属性|值|
|---|---|
|my_status|1|
|my_status|2|

<h2 id="tocS_CollectionResponse">CollectionResponse</h2>

<a id="schemacollectionresponse"></a>
<a id="schema_CollectionResponse"></a>
<a id="tocScollectionresponse"></a>
<a id="tocscollectionresponse"></a>

```json
{
  "activity_id": "string",
  "title": "string",
  "cover_url": "string",
  "start_time": "2019-08-24T14:15:22Z",
  "end_time": "2019-08-24T14:15:22Z"
}

```

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|activity_id|string|true|none||none|
|title|string|true|none||none|
|cover_url|string|true|none||none|
|start_time|string(date-time)|true|none||none|
|end_time|string(date-time)|true|none||none|

