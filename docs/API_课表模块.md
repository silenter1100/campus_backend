---
title: 课表/日历模块
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

# 课表/日历模块

Base URLs:

# Authentication

- HTTP Authentication, scheme: bearer

# Default

## GET 获取全校课程列表-v1

GET /api/v1/courses

根据筛选条件获取全校课表，给学生选择自己课程放入课程表

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|semester_id|query|integer| 否 |学期id，可选|
|name|query|string| 否 |课程名称模糊搜索，可选|
|teacher|query|string| 否 |教师姓名模糊搜索，可选|
|page|query|integer| 否 |none|
|pageSize|query|integer| 否 |none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "list": [
      {
        "id": 0,
        "course_name": "string",
        "teacher_name": "string",
        "teacher_id": 0,
        "location": "string",
        "day_of_week": 0,
        "start_section": 0,
        "end_section": 0,
        "weeks_range": [
          0
        ],
        "type": "string",
        "credits": 0,
        "description": "string"
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
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|none|None|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|None|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|None|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» list|[[PublicCourse](#schemapubliccourse)]|true|none||none|
|»»» 全校课程|[PublicCourse](#schemapubliccourse)|false|none|全校课程|none|
|»»»» id|integer|true|none|课程ID|none|
|»»»» course_name|string|true|none|课程名称|none|
|»»»» teacher_name|string|true|none|教师姓名|none|
|»»»» teacher_id|integer¦null|true|none|教师工号|none|
|»»»» location|string|true|none|楼教室|none|
|»»»» day_of_week|integer|true|none|星期几|1-7 (1代表周一)|
|»»»» start_section|integer|true|none|开始节次|如: 1 (第1节)|
|»»»» end_section|integer|true|none|结束节次|none|
|»»»» weeks_range|[integer]|true|none|周次范围|如:[1,2,...16]|
|»»»» type|string|true|none|课程类型|compulsory(必修)/elective(选修)|
|»»»» credits|integer|true|none|学分|none|
|»»»» description|string¦null|false|none|课程描述|none|
|»» pagination|object|true|none||none|
|»»» total|integer|true|none|总记录数|none|
|»»» page|integer|true|none|当前页码|none|
|»»» pageSize|integer|true|none|每页限制数|none|
|»»» pages|integer|true|none|总页数|none|

## GET 获取学期列表

GET /api/v1/semesters

获取学期列表

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "semesters": {
      "id": 0,
      "name": "string",
      "start_date": "string",
      "end_date": "string",
      "is_current": true
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|none|None|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|None|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|None|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» semesters|[Semester](#schemasemester)|true|none|学期|none|
|»»» id|integer|true|none|学期唯一id|none|
|»»» name|string|true|none|学期名|none|
|»»» start_date|string|true|none|学期开始日期|标准格式，如”YYYY-MM-DD“|
|»»» end_date|string|true|none|学期结束日期|标准格式，如”YYYY-MM-DD“|
|»»» is_current|boolean|true|none|是否当前学期|none|

## GET 获取用户课程表项

GET /api/v1/schedule

获取用户课表项，通过Authorization 中的Bearer token确认唯一用户

### 请求参数

|名称|位置|类型|必选|说明|
|---|---|---|---|---|
|semester_id|query|string| 是 |选择学期课表|
|week|query|integer| 否 |返回制定周课表项|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "items": {
      "id": 0,
      "source_id": 0,
      "course_name": "string",
      "teacher_name": "string",
      "location": "string",
      "day_of_week": 0,
      "start_section": 0,
      "end_section": 0,
      "weeks_range": [
        0
      ],
      "type": "string",
      "credits": 0,
      "description": "string",
      "color_hex": "#9E9E9E",
      "is_custom": true
    }
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|none|None|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|None|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|None|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» items|[ScheduleItem](#schemascheduleitem)|true|none|用户课表项|none|
|»»» id|integer|true|none|用户课表项唯一id|用户课表项的唯一ID|
|»»» source_id|integer¦null|false|none|PublicCourse.id|如果是非自定义课程，则为PublicCourse.id|
|»»» course_name|string|true|none|课程名称|none|
|»»» teacher_name|string¦null|false|none|教师名称|none|
|»»» location|string¦null|false|none|楼教室|none|
|»»» day_of_week|integer|true|none||none|
|»»» start_section|integer|true|none||none|
|»»» end_section|integer|true|none||none|
|»»» weeks_range|[integer]|true|none||none|
|»»» type|string¦null|false|none|公共/选修|compulsory/elective|
|»»» credits|integer¦null|false|none|学分|none|
|»»» description|string¦null|false|none||none|
|»»» color_hex|string|true|none|#RRGGBB，有默认值|none|
|»»» is_custom|boolean|true|none|是否用户自定义|none|

## POST 新增课表项

POST /api/v1/schedule

新增课表项，支持批量导出课程或单个课程添加
注意：is_custom和source_id应该互斥

> Body 请求参数

```json
{
  "items": [
    {
      "source_id": 0,
      "course_name": "string",
      "teacher_name": "string",
      "location": "string",
      "day_of_week": 0,
      "start_section": 0,
      "end_section": 0,
      "weeks": [
        0
      ],
      "type": "string",
      "credits": 0,
      "description": "string",
      "color_hex": "#9E9E9E",
      "is_custom": true
    }
  ]
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 是 ||none|
|» items|body|[object]| 是 ||none|
|»» source_id|body|integer¦null| 否 | PublicCourse.id|如果是非自定义课程，则为PublicCourse.id|
|»» course_name|body|string| 是 | 课程名称|none|
|»» teacher_name|body|string¦null| 否 | 教师名称|none|
|»» location|body|string¦null| 否 | 楼教室|none|
|»» day_of_week|body|integer| 是 ||none|
|»» start_section|body|integer| 是 ||none|
|»» end_section|body|integer| 是 ||none|
|»» weeks|body|[integer]| 是 ||none|
|»» type|body|string¦null| 否 | 公共/选修|compulsory/elective|
|»» credits|body|integer¦null| 否 | 学分|none|
|»» description|body|string¦null| 否 ||none|
|»» color_hex|body|string| 是 | #RRGGBB，有默认值|none|
|»» is_custom|body|boolean| 是 | 是否用户自定义|none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "successful_items": [
      {
        "id": 0,
        "source_id": 0,
        "course_name": "string",
        "teacher_name": "string",
        "location": "string",
        "day_of_week": 0,
        "start_section": 0,
        "end_section": 0,
        "weeks_range": [
          0
        ],
        "type": "string",
        "credits": 0,
        "description": "string",
        "color_hex": "#9E9E9E",
        "is_custom": true
      }
    ],
    "failed_items": [
      {
        "course_name": "string",
        "error_message": "string"
      }
    ]
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|none|Inline|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|None|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none|业务状态码|none|
|» message|string|true|none|消息提示|例如：导入处理完成，其中 2 项成功，1 项因数据校验失败。|
|» data|object|true|none||none|
|»» successful_items|[[ScheduleItem](#schemascheduleitem)]|true|none|成功课表项|none|
|»»» 用户课表项|[ScheduleItem](#schemascheduleitem)|false|none|用户课表项|none|
|»»»» id|integer|true|none|用户课表项唯一id|用户课表项的唯一ID|
|»»»» source_id|integer¦null|false|none|PublicCourse.id|如果是非自定义课程，则为PublicCourse.id|
|»»»» course_name|string|true|none|课程名称|none|
|»»»» teacher_name|string¦null|false|none|教师名称|none|
|»»»» location|string¦null|false|none|楼教室|none|
|»»»» day_of_week|integer|true|none||none|
|»»»» start_section|integer|true|none||none|
|»»»» end_section|integer|true|none||none|
|»»»» weeks_range|[integer]|true|none||none|
|»»»» type|string¦null|false|none|公共/选修|compulsory/elective|
|»»»» credits|integer¦null|false|none|学分|none|
|»»»» description|string¦null|false|none||none|
|»»»» color_hex|string|true|none|#RRGGBB，有默认值|none|
|»»»» is_custom|boolean|true|none|是否用户自定义|none|
|»» failed_items|[[Course_error_message](#schemacourse_error_message)]|true|none|失败列表|none|
|»»» 课程上传失败消息|[Course_error_message](#schemacourse_error_message)|false|none|课程上传失败消息|none|
|»»»» course_name|string|true|none|课程名称|none|
|»»»» error_message|string|true|none|失败原因|none|

状态码 **400**

*400 Bad Request 响应*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||HTTP 状态码|
|» message|string|true|none||空请求体/ items 非数组 / 互斥字段source_id 和 is_custom相互冲突|
|» data|object¦null|false|none||错误时 data 字段通常为 null。|

## DELETE 删除课程表项

DELETE /api/v1/schedule

删除指定课程

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|item_id|query|integer| 是 ||none|

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
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|none|None|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|None|
|500|[Internal Server Error](https://tools.ietf.org/html/rfc7231#section-6.6.1)|none|None|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|string¦null|false|none|为了返回结构一致，默认null|none|

## PATCH 更新课程表项

PATCH /api/v1/schedule

对课程表项（即一节课）信息进行局部更新

> Body 请求参数

```json
{
  "source_id": 0,
  "semester_id": "string",
  "course_name": "string",
  "teacher_name": "string",
  "location": "string",
  "day_of_week": 0,
  "start_section": 0,
  "end_section": 0,
  "weeks": [
    0
  ],
  "type": "string",
  "credits": 0,
  "description": "string",
  "color_hex": "#9E9E9E",
  "is_custom": true
}
```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|item_id|query|integer| 是 ||none|
|body|body|object| 是 ||none|
|» source_id|body|integer| 否 | PublicCourse.id|如果是非自定义课程，则为PublicCourse.id|
|» semester_id|body|string| 是 | 学期|none|
|» course_name|body|string| 否 | 课程名称|none|
|» teacher_name|body|string| 否 | 教师名称|none|
|» location|body|string¦null| 否 | 楼教室|none|
|» day_of_week|body|integer| 否 ||none|
|» start_section|body|integer| 否 ||none|
|» end_section|body|integer| 否 ||none|
|» weeks|body|[integer]| 否 ||none|
|» type|body|string¦null| 否 | 公共/选修|compulsory/elective|
|» credits|body|integer¦null| 否 | 学分|none|
|» description|body|string¦null| 否 ||none|
|» color_hex|body|string| 否 | #RRGGBB，有默认值|none|
|» is_custom|body|boolean| 否 | 是否用户自定义|none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "id": 0,
    "source_id": 0,
    "course_name": "string",
    "teacher_name": "string",
    "location": "string",
    "day_of_week": 0,
    "start_section": 0,
    "end_section": 0,
    "weeks_range": [
      0
    ],
    "type": "string",
    "credits": 0,
    "description": "string",
    "color_hex": "#9E9E9E",
    "is_custom": true
  }
}
```

### 返回结果

|状态码|状态码含义|说明|数据模型|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|none|Inline|
|400|[Bad Request](https://tools.ietf.org/html/rfc7231#section-6.5.1)|none|Inline|
|401|[Unauthorized](https://tools.ietf.org/html/rfc7235#section-3.1)|none|None|
|404|[Not Found](https://tools.ietf.org/html/rfc7231#section-6.5.4)|none|Inline|

### 返回数据结构

状态码 **200**

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||none|
|» message|string|true|none||none|
|» data|object|true|none||none|
|»» id|integer|true|none|用户课表项唯一id|用户课表项的唯一ID|
|»» source_id|integer¦null|false|none|PublicCourse.id|如果是非自定义课程，则为PublicCourse.id|
|»» course_name|string|true|none|课程名称|none|
|»» teacher_name|string¦null|false|none|教师名称|none|
|»» location|string¦null|false|none|楼教室|none|
|»» day_of_week|integer|true|none||none|
|»» start_section|integer|true|none||none|
|»» end_section|integer|true|none||none|
|»» weeks_range|[integer]|true|none||none|
|»» type|string¦null|false|none|公共/选修|compulsory/elective|
|»» credits|integer¦null|false|none|学分|none|
|»» description|string¦null|false|none||none|
|»» color_hex|string|true|none|#RRGGBB，有默认值|none|
|»» is_custom|boolean|true|none|是否用户自定义|none|

状态码 **400**

*400 Bad Request 响应*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||HTTP 状态码|
|» message|string|true|none||参数值越界类似开始大于结束<br />业务冲突：课程时间冲突|
|» data|object¦null|false|none||错误时 data 字段通常为 null。|

状态码 **404**

*404 Not Found 响应*

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|» code|integer|true|none||HTTP 状态码|
|» message|string|true|none||资源不存在：指定的课程表项 item_id 不存在或已被删除<br />ID校验失败：item_id 缺失或格式错误|
|» data|object¦null|false|none||错误时 data 字段通常为 null。|

## POST 上传图片/文件

POST /api/v1/storage/upload

通用文件上传接口，支持图片、文档,暂时约定支持png jpg pdf

> Body 请求参数

```yaml
file: ""
file_type: ""

```

### 请求参数

|名称|位置|类型|必选|中文名|说明|
|---|---|---|---|---|---|
|body|body|object| 是 ||none|
|» file|body|string(binary)| 否 ||none|
|» file_type|body|string| 否 ||none|

> 返回示例

> 200 Response

```json
{
  "code": 0,
  "message": "string",
  "data": {
    "url": "string",
    "thumbnail_url": "string",
    "filename": "string",
    "size": 0
  }
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
|» data|object¦null|false|none|业务数据主体|none|
|»» url|string|false|none||文件在对象存储中的永久访问链接 (OSS/S3 URL)|
|»» thumbnail_url|string¦null|false|none||如果是图片，返回自动生成的缩略图 URL (可选)|
|»» filename|string|false|none||服务器保存的文件名|
|»» size|integer|false|none||文件大小，单位：字节 (Bytes)|

# 数据模型

<h2 id="tocS_Semester">Semester</h2>

<a id="schemasemester"></a>
<a id="schema_Semester"></a>
<a id="tocSsemester"></a>
<a id="tocssemester"></a>

```json
{
  "id": 0,
  "name": "string",
  "start_date": "string",
  "end_date": "string",
  "is_current": true
}

```

学期

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|integer|true|none|学期唯一id|none|
|name|string|true|none|学期名|none|
|start_date|string|true|none|学期开始日期|标准格式，如”YYYY-MM-DD“|
|end_date|string|true|none|学期结束日期|标准格式，如”YYYY-MM-DD“|
|is_current|boolean|true|none|是否当前学期|none|

<h2 id="tocS_PublicCourse">PublicCourse</h2>

<a id="schemapubliccourse"></a>
<a id="schema_PublicCourse"></a>
<a id="tocSpubliccourse"></a>
<a id="tocspubliccourse"></a>

```json
{
  "id": 0,
  "course_name": "string",
  "teacher_name": "string",
  "teacher_id": 0,
  "location": "string",
  "day_of_week": 0,
  "start_section": 0,
  "end_section": 0,
  "weeks_range": [
    0
  ],
  "type": "string",
  "credits": 0,
  "description": "string"
}

```

全校课程

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|integer|true|none|课程ID|none|
|course_name|string|true|none|课程名称|none|
|teacher_name|string|true|none|教师姓名|none|
|teacher_id|integer¦null|true|none|教师工号|none|
|location|string|true|none|楼教室|none|
|day_of_week|integer|true|none|星期几|1-7 (1代表周一)|
|start_section|integer|true|none|开始节次|如: 1 (第1节)|
|end_section|integer|true|none|结束节次|none|
|weeks_range|[integer]|true|none|周次范围|如:[1,2,...16]|
|type|string|true|none|课程类型|compulsory(必修)/elective(选修)|
|credits|integer|true|none|学分|none|
|description|string¦null|false|none|课程描述|none|

<h2 id="tocS_ScheduleItem">ScheduleItem</h2>

<a id="schemascheduleitem"></a>
<a id="schema_ScheduleItem"></a>
<a id="tocSscheduleitem"></a>
<a id="tocsscheduleitem"></a>

```json
{
  "id": 0,
  "source_id": 0,
  "course_name": "string",
  "teacher_name": "string",
  "location": "string",
  "day_of_week": 0,
  "start_section": 0,
  "end_section": 0,
  "weeks_range": [
    0
  ],
  "type": "string",
  "credits": 0,
  "description": "string",
  "color_hex": "#9E9E9E",
  "is_custom": true
}

```

用户课表项

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|id|integer|true|none|用户课表项唯一id|用户课表项的唯一ID|
|source_id|integer¦null|false|none|PublicCourse.id|如果是非自定义课程，则为PublicCourse.id|
|course_name|string|true|none|课程名称|none|
|teacher_name|string¦null|false|none|教师名称|none|
|location|string¦null|false|none|楼教室|none|
|day_of_week|integer|true|none||none|
|start_section|integer|true|none||none|
|end_section|integer|true|none||none|
|weeks_range|[integer]|true|none||none|
|type|string¦null|false|none|公共/选修|compulsory/elective|
|credits|integer¦null|false|none|学分|none|
|description|string¦null|false|none||none|
|color_hex|string|true|none|#RRGGBB，有默认值|none|
|is_custom|boolean|true|none|是否用户自定义|none|

<h2 id="tocS_pagination">pagination</h2>

<a id="schemapagination"></a>
<a id="schema_pagination"></a>
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

<h2 id="tocS_Course_error_message">Course_error_message</h2>

<a id="schemacourse_error_message"></a>
<a id="schema_Course_error_message"></a>
<a id="tocScourse_error_message"></a>
<a id="tocscourse_error_message"></a>

```json
{
  "course_name": "string",
  "error_message": "string"
}

```

课程上传失败消息

### 属性

|名称|类型|必选|约束|中文名|说明|
|---|---|---|---|---|---|
|course_name|string|true|none|课程名称|none|
|error_message|string|true|none|失败原因|none|

