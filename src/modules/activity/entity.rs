// src/modules/activity/entity.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow; // 👈 新增

/// 统一的 API 响应包装，与文档中的 { code, message, data } 对应
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "ok".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            message: msg.into(),
            data: None,
        }
    }
}

/// 简单的当前用户信息（建议放到 common 模块，这里先放着）
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: i64,
    pub is_admin: bool,
    // 视需要扩展字段
}

/// 活动状态
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "activity_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityStatus {
    Draft,
    PendingReview,
    Published,
    Finished,
    Cancelled,
}

/// 可见范围
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "activity_visibility", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityVisibility {
    Public,
    OrganizationOnly,
    LinkOnly,
}

/// Activity 实体（既可映射 DB，也可作为对外 JSON）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Activity {
    pub id: i64,
    pub title: String,
    pub cover_url: Option<String>,
    pub summary: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub location: String,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub signup_start_time: Option<DateTime<Utc>>,
    pub signup_end_time: Option<DateTime<Utc>>,
    pub capacity: Option<i32>,
    pub signup_count: i32,
    pub organizer_id: i64,
    pub organizer_name: String,
    pub organizer_type: String,
    pub status: ActivityStatus,
    pub visibility: ActivityVisibility,
    pub can_comment: bool,
    pub is_official: bool,
    pub signup_required: bool,
    pub checkin_required: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
}

/// 报名状态
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "activity_signup_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivitySignupStatus {
    Applied,
    Cancelled,
    CheckedIn,
}

/// 报名记录
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ActivitySignup {
    pub id: i64,
    pub user_id: i64,
    pub activity_id: i64,
    pub status: ActivitySignupStatus,
    pub checkin_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


/// 签到记录（如需要单独返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityCheckin {
    pub id: i64,
    pub activity_id: i64,
    pub user_id: i64,
    pub checkin_time: DateTime<Utc>,
    pub method: String,
    pub device_info: Option<String>,
}

/// 活动列表 item（可以直接复用 Activity，也可以做精简版）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActivityListItem {
    pub id: i64,
    pub title: String,
    pub cover_url: Option<String>,
    pub summary: String,
    pub category: String,
    pub tags: Vec<String>,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub signup_end_time: Option<DateTime<Utc>>,
    pub capacity: Option<i32>,
    pub signup_count: i32,
    pub status: ActivityStatus,
    pub visibility: ActivityVisibility,
    pub organizer_name: String,
    pub is_official: bool,
}


/// 分页返回
#[derive(Debug, Serialize)]
pub struct Paged<T> {
    pub total: i64,
    pub list: Vec<T>,
}

/// ---------- 请求 DTO ----------

#[derive(Debug, Deserialize)]
pub struct ListActivitiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub keyword: Option<String>,
    pub category: Option<String>,
    pub status: Option<String>,
    pub only_joined: Option<bool>,
    pub start_time_from: Option<DateTime<Utc>>,
    pub start_time_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ListMyActivitiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub filter: Option<String>, // upcoming / history
}

#[derive(Debug, Deserialize)]
pub struct SignupActivityBody {
    pub mobile: Option<String>,
    pub student_id: Option<String>,
    pub department: Option<String>,
    pub extra_form: Option<String>, // 可以放 JSON 字符串
}

#[derive(Debug, Deserialize)]
pub struct CreateActivityBody {
    pub title: String,
    pub cover_url: Option<String>,
    pub summary: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub signup_start_time: Option<DateTime<Utc>>,
    pub signup_end_time: Option<DateTime<Utc>>,
    pub capacity: Option<i32>,
    pub visibility: ActivityVisibility,
    pub signup_required: Option<bool>,
    pub checkin_required: Option<bool>,
    pub can_comment: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateActivityBody {
    pub title: Option<String>,
    pub cover_url: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub location: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub signup_start_time: Option<DateTime<Utc>>,
    pub signup_end_time: Option<DateTime<Utc>>,
    pub capacity: Option<i32>,
    pub visibility: Option<ActivityVisibility>,
    pub signup_required: Option<bool>,
    pub checkin_required: Option<bool>,
    pub can_comment: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewActivityBody {
    pub action: String, // APPROVE / REJECT
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckinBody {
    pub checkin_token: String,
}
