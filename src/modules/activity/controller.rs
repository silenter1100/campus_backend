// src/modules/activity/controller.rs

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::common::auth::AuthUser;
use crate::common::error::AppError;
use crate::common::state::AppState;
use crate::modules::activity::entity::*;
use crate::modules::activity::service;

// ====================
//   请求参数结构体
// ====================

#[derive(Debug, Deserialize)]
pub struct ListActivitiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub activity_type: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListMyActivitiesQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub include_enrollments: Option<bool>,
    pub include_collections: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateActivityBody {
    pub title: String,
    pub content: String,
    pub cover_url: Option<String>,
    pub activity_type: Option<i32>,
    pub location: String,
    pub organizer: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub quota: Option<i32>,
    pub need_sign_in: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateActivityBody {
    pub title: Option<String>,
    pub content: Option<String>,
    pub cover_url: Option<String>,
    pub activity_type: Option<i32>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub quota: Option<i32>,
    pub need_sign_in: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SignupActivityBody {
    pub user_name: String,
    pub student_id: String,
    pub major: String,
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewActivityBody {
    pub status: i32, // 1=通过, 2=拒绝
}

// ====================
//   通用响应结构体
// ====================

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

// ====================
//   辅助函数
// ====================

/// 获取用户信息或返回假用户（用于测试）
fn get_or_fake_user(auth_user: Option<AuthUser>) -> AuthUser {
    match auth_user {
        Some(user) => user,
        None => AuthUser {
            user_id: "fake_user_123".to_string(),
            role: "student".to_string(),
        },
    }
}

/// 获取管理员用户或返回假管理员（用于测试）
fn get_or_fake_admin(auth_user: Option<AuthUser>) -> AuthUser {
    match auth_user {
        Some(user) => user,
        None => AuthUser {
            user_id: "fake_admin_123".to_string(),
            role: "admin".to_string(),
        },
    }
}

// ====================
//   控制器函数
// ====================

/// 获取活动列表
pub async fn list_activities(
    State(state): State<AppState>,
    Query(query): Query<ListActivitiesQuery>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    let params = GetActivitiesParams {
        page: query.page.unwrap_or(1) as i32,
        page_size: query.page_size.unwrap_or(10) as i32,
        activity_type: query.activity_type,
        keyword: None,
        user_id: Some(user.user_id),
    };

    let result = service::get_activities(&state.pool, params).await?;
    Ok(Json(ApiResponse::ok(result)))
}

/// 获取活动详情
pub async fn get_activity_detail(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    let result = service::get_activity_detail(&state.pool, &activity_id, Some(&user.user_id)).await?;
    Ok(Json(ApiResponse::ok(result)))
}

/// 报名活动
pub async fn signup_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
    Json(body): Json<SignupActivityBody>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    let input = EnrollActivityInput {
        user_name: body.user_name,
        student_id: body.student_id,
        major: body.major,
        phone_number: body.phone_number,
    };

    service::enroll_activity(&state.pool, &user.user_id, &activity_id, input).await?;
    Ok(Json(ApiResponse::ok("报名成功")))
}

/// 取消报名
pub async fn cancel_signup(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    service::cancel_enrollment(&state.pool, &user.user_id, &activity_id).await?;
    Ok(Json(ApiResponse::ok("取消报名成功")))
}

/// 收藏活动
pub async fn collect_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    service::collect_activity(&state.pool, &user.user_id, &activity_id).await?;
    Ok(Json(ApiResponse::ok("收藏成功")))
}

/// 取消收藏
pub async fn uncollect_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    service::uncollect_activity(&state.pool, &user.user_id, &activity_id).await?;
    Ok(Json(ApiResponse::ok("取消收藏成功")))
}

/// 我的活动
pub async fn list_my_activities(
    State(state): State<AppState>,
    Query(query): Query<ListMyActivitiesQuery>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    let params = GetMyActivitiesParams {
        page: query.page.unwrap_or(1) as i32,
        page_size: query.page_size.unwrap_or(10) as i32,
        include_enrollments: query.include_enrollments.unwrap_or(true),
        include_collections: query.include_collections.unwrap_or(true),
    };

    let result = service::get_my_activities(&state.pool, &user.user_id, params).await?;
    Ok(Json(ApiResponse::ok(result)))
}

// ====================
//   管理员接口
// ====================

/// 创建活动（管理员）
pub async fn create_activity(
    State(state): State<AppState>,
    Json(body): Json<CreateActivityBody>,
) -> Result<impl IntoResponse, AppError> {
    // 在实际应用中，这里应该验证管理员权限
    let user = AuthUser {
        user_id: "admin_123".to_string(),
        role: "admin".to_string(),
    };
    
    let input = CreateActivityInput {
        title: body.title,
        content: body.content,
        cover_url: body.cover_url,
        activity_type: body.activity_type,
        location: body.location,
        organizer: body.organizer,
        start_time: body.start_time,
        end_time: body.end_time,
        quota: body.quota,
        need_sign_in: body.need_sign_in,
    };

    let result = service::create_activity(&state.pool, input).await?;
    Ok(Json(ApiResponse::ok(result)))
}

/// 更新活动（管理员）
pub async fn update_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: AuthUser,
    Json(body): Json<UpdateActivityBody>,
) -> Result<impl IntoResponse, AppError> {
    let input = UpdateActivityInput {
        title: body.title,
        content: body.content,
        cover_url: body.cover_url,
        activity_type: body.activity_type,
        location: body.location,
        organizer: body.organizer,
        start_time: body.start_time,
        end_time: body.end_time,
        quota: body.quota,
        need_sign_in: body.need_sign_in,
        status: None, // 不允许通过更新接口修改状态
    };

    let result = service::update_activity(&state.pool, &activity_id, input).await?;
    Ok(Json(ApiResponse::ok(result)))
}

/// 获取报名列表（管理员）
pub async fn list_signups(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let result = service::get_enrollments(&state.pool, &activity_id).await?;
    Ok(Json(ApiResponse::ok(result)))
}

/// 审核活动（管理员）
pub async fn admin_review_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
    Json(body): Json<ReviewActivityBody>,
) -> Result<impl IntoResponse, AppError> {
    let _user = get_or_fake_admin(auth_user);
    
    // 这里应该调用相应的 service 函数来更新活动状态
    // 暂时返回成功响应
    Ok(Json(ApiResponse::ok("审核成功")))
}

/// 封禁活动（管理员）
pub async fn admin_block_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let _user = get_or_fake_admin(auth_user);
    
    // 这里应该调用相应的 service 函数来封禁活动
    // 暂时返回成功响应
    Ok(Json(ApiResponse::ok("封禁成功")))
}