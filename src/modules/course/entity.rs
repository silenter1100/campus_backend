use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

// ==================== 数据库实体 ====================

/// 学期数据库实体
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SemesterDb {
    pub id: i64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub is_current: bool,
}

/// 全校课程数据库实体
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PublicCourseDb {
    pub id: i64,
    pub semester_id: i64,
    pub course_name: String,
    pub teacher_name: String,
    pub teacher_id: Option<String>,
    pub location: String,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks_range: Json<Vec<i32>>,
    pub r#type: String,
    pub credits: Option<i32>,
    pub description: Option<String>,
}

/// 用户课表项数据库实体
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ScheduleItemDb {
    pub id: i64,
    pub user_id: String,
    pub semester_id: i64,
    pub source_id: Option<i64>,
    pub course_name: String,
    pub teacher_name: Option<String>,
    pub location: Option<String>,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks_range: Json<Vec<i32>>,
    pub r#type: Option<String>,
    pub credits: Option<i32>,
    pub description: Option<String>,
    pub color_hex: String,
    pub is_custom: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ==================== API 响应实体 ====================

/// 学期信息
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Semester {
    pub id: i64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub is_current: bool,
}

impl From<SemesterDb> for Semester {
    fn from(db: SemesterDb) -> Self {
        Self {
            id: db.id,
            name: db.name,
            start_date: db.start_date,
            end_date: db.end_date,
            is_current: db.is_current,
        }
    }
}

/// 全校课程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicCourse {
    pub id: i64,
    pub course_name: String,
    pub teacher_name: String,
    pub teacher_id: Option<String>,
    pub location: String,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks_range: Vec<i32>,
    pub r#type: String,
    pub credits: Option<i32>,
    pub description: Option<String>,
}

impl From<PublicCourseDb> for PublicCourse {
    fn from(db: PublicCourseDb) -> Self {
        Self {
            id: db.id,
            course_name: db.course_name,
            teacher_name: db.teacher_name,
            teacher_id: db.teacher_id,
            location: db.location,
            day_of_week: db.day_of_week,
            start_section: db.start_section,
            end_section: db.end_section,
            weeks_range: db.weeks_range.0,
            r#type: db.r#type,
            credits: db.credits,
            description: db.description,
        }
    }
}

/// 用户课表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleItem {
    pub id: i64,
    pub source_id: Option<i64>,
    pub course_name: String,
    pub teacher_name: Option<String>,
    pub location: Option<String>,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks_range: Vec<i32>,
    pub r#type: Option<String>,
    pub credits: Option<i32>,
    pub description: Option<String>,
    pub color_hex: String,
    pub is_custom: bool,
}

impl From<ScheduleItemDb> for ScheduleItem {
    fn from(db: ScheduleItemDb) -> Self {
        Self {
            id: db.id,
            source_id: db.source_id,
            course_name: db.course_name,
            teacher_name: db.teacher_name,
            location: db.location,
            day_of_week: db.day_of_week,
            start_section: db.start_section,
            end_section: db.end_section,
            weeks_range: db.weeks_range.0,
            r#type: db.r#type,
            credits: db.credits,
            description: db.description,
            color_hex: db.color_hex,
            is_custom: db.is_custom,
        }
    }
}

// ==================== 内部 DTO ====================

/// 添加课表项输入
#[derive(Debug, Clone)]
pub struct ScheduleItemInput {
    pub source_id: Option<i64>,
    pub course_name: String,
    pub teacher_name: Option<String>,
    pub location: Option<String>,
    pub day_of_week: i32,
    pub start_section: i32,
    pub end_section: i32,
    pub weeks: Vec<i32>,
    pub r#type: Option<String>,
    pub credits: Option<i32>,
    pub description: Option<String>,
    pub color_hex: String,
    pub is_custom: bool,
}

/// 更新课表项输入
#[derive(Debug, Clone)]
pub struct UpdateScheduleItemInput {
    pub course_name: Option<String>,
    pub teacher_name: Option<String>,
    pub location: Option<String>,
    pub day_of_week: Option<i32>,
    pub start_section: Option<i32>,
    pub end_section: Option<i32>,
    pub weeks: Option<Vec<i32>>,
    pub r#type: Option<String>,
    pub credits: Option<i32>,
    pub description: Option<String>,
    pub color_hex: Option<String>,
}

/// 获取全校课程查询参数
#[derive(Debug, Clone)]
pub struct GetCoursesParams {
    pub semester_id: Option<i64>,
    pub name: Option<String>,
    pub teacher: Option<String>,
    pub page: i32,
    pub page_size: i32,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub pages: i32,
}

/// 批量添加失败项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedItem {
    pub course_name: String,
    pub error_message: String,
}

/// 批量添加结果
#[derive(Debug, Clone)]
pub struct BatchAddResult {
    pub successful_items: Vec<ScheduleItem>,
    pub failed_items: Vec<FailedItem>,
}
