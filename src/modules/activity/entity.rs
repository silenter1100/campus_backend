// src/modules/activity/entity.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ====================
//   数据库实体
// ====================

/// 活动数据库实体
#[derive(Debug, Clone, FromRow)]
pub struct ActivityDb {
    pub id: String,
    pub title: String,
    pub content: String,
    pub cover_url: Option<String>,
    pub activity_type: i32,
    pub location: String,
    pub organizer: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub quota: i32,
    pub current_enrollments: i32,
    pub need_sign_in: i8, // MySQL TINYINT(1) 映射为 i8
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 报名记录数据库实体
#[derive(Debug, Clone, FromRow)]
pub struct EnrollmentDb {
    pub id: i64,
    pub user_id: String,
    pub activity_id: String,
    pub user_name: String,
    pub student_id: String,
    pub major: String,
    pub phone_number: Option<String>,
    pub enroll_time: DateTime<Utc>,
    pub attendance_status: i32,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 收藏记录数据库实体
#[derive(Debug, Clone, FromRow)]
pub struct CollectionDb {
    pub id: i64,
    pub user_id: String,
    pub activity_id: String,
    pub created_at: DateTime<Utc>,
}

// ====================
//   内部 DTO
// ====================

/// 活动实体（用于 Service 层）
#[derive(Debug, Clone, Serialize)]
pub struct Activity {
    pub id: String,
    pub title: String,
    pub content: String,
    pub cover_url: Option<String>,
    pub activity_type: i32,
    pub location: String,
    pub organizer: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub quota: i32,
    pub current_enrollments: i32,
    pub need_sign_in: bool,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub is_enrolled: Option<bool>,
    pub is_collected: Option<bool>,
}

/// 活动列表项（精简版）
#[derive(Debug, Clone, Serialize)]
pub struct ActivitySummary {
    pub id: String,
    pub title: String,
    pub cover_url: Option<String>,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub quota: i32,
    pub current_enrollments: i32,
}

/// 报名记录
#[derive(Debug, Clone, Serialize)]
pub struct EnrollmentRecord {
    pub user_id: String,
    pub user_name: String,
    pub student_id: String,
    pub major: String,
    pub phone_number: Option<String>,
    pub activity_id: String,
    pub enroll_time: DateTime<Utc>,
    pub attendance_status: i32,
}

/// 我的报名活动项
#[derive(Debug, Clone, Serialize)]
pub struct EnrollmentSummary {
    pub activity_id: String,
    pub title: String,
    pub cover_url: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub my_status: i32,
}

/// 我的收藏活动项
#[derive(Debug, Clone, Serialize)]
pub struct CollectionSummary {
    pub activity_id: String,
    pub title: String,
    pub cover_url: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

/// 分页信息
#[derive(Debug, Clone, Serialize)]
pub struct Pagination {
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub pages: i32,
}

// ====================
//   请求 DTO
// ====================

/// 创建活动请求
#[derive(Debug, Clone)]
pub struct CreateActivityInput {
    pub title: String,
    pub content: String,
    pub location: String,
    pub organizer: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub cover_url: Option<String>,
    pub activity_type: Option<i32>,
    pub quota: Option<i32>,
    pub need_sign_in: Option<bool>,
}

/// 更新活动请求
#[derive(Debug, Clone)]
pub struct UpdateActivityInput {
    pub title: Option<String>,
    pub content: Option<String>,
    pub cover_url: Option<String>,
    pub activity_type: Option<i32>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub quota: Option<i32>,
    pub need_sign_in: Option<bool>,
    pub status: Option<i32>,
}

/// 获取活动列表请求
#[derive(Debug, Clone)]
pub struct GetActivitiesParams {
    pub keyword: Option<String>,
    pub activity_type: Option<i32>,
    pub page: i32,
    pub page_size: i32,
    pub user_id: Option<String>,
}

/// 学生报名请求
#[derive(Debug, Clone)]
pub struct EnrollActivityInput {
    pub user_name: String,
    pub student_id: String,
    pub major: String,
    pub phone_number: Option<String>,
}

/// 我的活动请求
#[derive(Debug, Clone)]
pub struct GetMyActivitiesParams {
    pub include_enrollments: bool,
    pub include_collections: bool,
    pub page: i32,
    pub page_size: i32,
}

// ====================
//   响应 DTO
// ====================

/// 活动列表响应
#[derive(Debug, Clone, Serialize)]
pub struct ActivitiesListResponse {
    pub list: Vec<ActivitySummary>,
    pub pagination: Pagination,
}

/// 报名列表响应
#[derive(Debug, Clone, Serialize)]
pub struct EnrollmentsResponse {
    pub total_enrolled: i32,
    pub enrollment_list: Vec<EnrollmentRecord>,
}

/// 我的活动响应
#[derive(Debug, Clone, Serialize)]
pub struct MyActivitiesResponse {
    pub enrolled_data: EnrolledData,
    pub collected_data: CollectedData,
}

#[derive(Debug, Clone, Serialize)]
pub struct EnrolledData {
    pub pagination: Pagination,
    pub list: Vec<EnrollmentSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CollectedData {
    pub pagination: Pagination,
    pub list: Vec<CollectionSummary>,
}

// ====================
//   转换函数
// ====================

impl From<ActivityDb> for Activity {
    fn from(db: ActivityDb) -> Self {
        Self {
            id: db.id,
            title: db.title,
            content: db.content,
            cover_url: db.cover_url,
            activity_type: db.activity_type,
            location: db.location,
            organizer: db.organizer,
            start_time: db.start_time,
            end_time: db.end_time,
            quota: db.quota,
            current_enrollments: db.current_enrollments,
            need_sign_in: db.need_sign_in != 0,
            status: db.status,
            created_at: db.created_at,
            is_enrolled: None,
            is_collected: None,
        }
    }
}

impl From<EnrollmentDb> for EnrollmentRecord {
    fn from(db: EnrollmentDb) -> Self {
        Self {
            user_id: db.user_id,
            user_name: db.user_name,
            student_id: db.student_id,
            major: db.major,
            phone_number: db.phone_number,
            activity_id: db.activity_id,
            enroll_time: db.enroll_time,
            attendance_status: db.attendance_status,
        }
    }
}
