// src/modules/activity/controller.rs

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
};
use prost::Message;
use serde::Deserialize;

use crate::common::auth::AuthUser;
use crate::common::error::AppError;
use crate::common::state::AppState;
use crate::modules::activity::entity::*;
use crate::modules::activity::service;

// 引入生成的 Protobuf 代码
mod proto {
    include!(concat!(env!("OUT_DIR"), "/campus.activity.rs"));
}

// ====================
//   查询参数结构体 (用于URL参数)
// ====================

#[derive(Debug, Deserialize)]
pub struct ListActivitiesQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub activity_type: Option<i32>,
    pub keyword: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListMyActivitiesQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub include_enrollments: Option<bool>,
    pub include_collections: Option<bool>,
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

/// 将活动实体转换为Proto格式
fn activity_to_proto(activity: Activity) -> proto::Activity {
    proto::Activity {
        id: activity.id,
        title: activity.title,
        content: activity.content,
        cover_url: activity.cover_url.unwrap_or_default(),
        activity_type: activity.activity_type,
        location: activity.location,
        organizer: activity.organizer,
        start_time: activity.start_time.to_rfc3339(),
        end_time: activity.end_time.to_rfc3339(),
        quota: activity.quota,
        current_enrollments: activity.current_enrollments,
        need_sign_in: activity.need_sign_in,
        status: activity.status,
        created_at: activity.created_at.to_rfc3339(),
        is_enrolled: activity.is_enrolled,
        is_collected: activity.is_collected,
    }
}

/// 将活动摘要转换为Proto格式
fn activity_summary_to_proto(summary: ActivitySummary) -> proto::ActivitySummary {
    proto::ActivitySummary {
        id: summary.id,
        title: summary.title,
        cover_url: summary.cover_url.unwrap_or_default(),
        location: summary.location,
        start_time: summary.start_time.to_rfc3339(),
        quota: summary.quota,
        current_enrollments: summary.current_enrollments,
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
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(10),
        activity_type: query.activity_type,
        keyword: query.keyword,
        user_id: Some(user.user_id),
    };

    let result = service::get_activities(&state.pool, params).await?;
    
    let proto_activities: Vec<proto::ActivitySummary> = result.list
        .into_iter()
        .map(activity_summary_to_proto)
        .collect();

    let response = proto::GetActivitiesResponse {
        code: 200,
        message: "success".to_string(),
        data: Some(proto::GetActivitiesData {
            list: proto_activities,
            pagination: Some(proto::Pagination {
                total: result.pagination.total,
                page: result.pagination.page as i32,
                page_size: result.pagination.page_size as i32,
                pages: result.pagination.pages as i32,
            }),
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 获取活动详情
pub async fn get_activity_detail(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    let result = service::get_activity_detail(&state.pool, &activity_id, Some(&user.user_id)).await?;
    
    let proto_activity = activity_to_proto(result);

    let response = proto::GetActivityDetailResponse {
        code: 200,
        message: "success".to_string(),
        data: Some(proto_activity),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 报名活动
pub async fn signup_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::EnrollActivityRequest::decode(body)?;
    let user = get_or_fake_user(auth_user);
    
    let input = EnrollActivityInput {
        user_name: proto_req.user_name,
        student_id: proto_req.student_id,
        major: proto_req.major,
        phone_number: proto_req.phone_number,
    };

    service::enroll_activity(&state.pool, &user.user_id, &activity_id, input).await?;
    
    let response = proto::EnrollActivityResponse {
        code: 200,
        message: "报名成功".to_string(),
        data: None,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 取消报名
pub async fn cancel_signup(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    service::cancel_enrollment(&state.pool, &user.user_id, &activity_id).await?;
    
    let response = proto::CancelEnrollmentResponse {
        code: 200,
        message: "取消报名成功".to_string(),
        data: None,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 收藏活动
pub async fn collect_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    service::collect_activity(&state.pool, &user.user_id, &activity_id).await?;
    
    let response = proto::CollectActivityResponse {
        code: 200,
        message: "收藏成功".to_string(),
        data: None,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 取消收藏
pub async fn uncollect_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    service::uncollect_activity(&state.pool, &user.user_id, &activity_id).await?;
    
    let response = proto::UncollectActivityResponse {
        code: 200,
        message: "取消收藏成功".to_string(),
        data: None,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 我的活动
pub async fn list_my_activities(
    State(state): State<AppState>,
    Query(query): Query<ListMyActivitiesQuery>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = get_or_fake_user(auth_user);
    
    let params = GetMyActivitiesParams {
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(10),
        include_enrollments: query.include_enrollments.unwrap_or(true),
        include_collections: query.include_collections.unwrap_or(true),
    };

    let result = service::get_my_activities(&state.pool, &user.user_id, params).await?;
    
    // 转换报名活动
    let enrolled_list: Vec<proto::EnrollmentSummary> = result.enrolled_data.list
        .into_iter()
        .map(|item| proto::EnrollmentSummary {
            activity_id: item.activity_id,
            title: item.title,
            cover_url: item.cover_url.unwrap_or_default(),
            start_time: item.start_time.to_rfc3339(),
            end_time: item.end_time.to_rfc3339(),
            my_status: item.my_status,
        })
        .collect();

    // 转换收藏活动
    let collected_list: Vec<proto::CollectionSummary> = result.collected_data.list
        .into_iter()
        .map(|item| proto::CollectionSummary {
            activity_id: item.activity_id,
            title: item.title,
            cover_url: item.cover_url.unwrap_or_default(),
            start_time: item.start_time.to_rfc3339(),
            end_time: item.end_time.to_rfc3339(),
        })
        .collect();

    let response = proto::GetMyActivitiesResponse {
        code: 200,
        message: "success".to_string(),
        data: Some(proto::GetMyActivitiesData {
            enrolled_data: Some(proto::EnrolledData {
                pagination: Some(proto::Pagination {
                    total: result.enrolled_data.pagination.total,
                    page: result.enrolled_data.pagination.page as i32,
                    page_size: result.enrolled_data.pagination.page_size as i32,
                    pages: result.enrolled_data.pagination.pages as i32,
                }),
                list: enrolled_list,
            }),
            collected_data: Some(proto::CollectedData {
                pagination: Some(proto::Pagination {
                    total: result.collected_data.pagination.total,
                    page: result.collected_data.pagination.page as i32,
                    page_size: result.collected_data.pagination.page_size as i32,
                    pages: result.collected_data.pagination.pages as i32,
                }),
                list: collected_list,
            }),
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

// ====================
//   管理员接口
// ====================

/// 创建活动（管理员）
pub async fn create_activity(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::CreateActivityRequest::decode(body)?;
    
    // 在实际应用中，这里应该验证管理员权限
    let _user = AuthUser {
        user_id: "admin_123".to_string(),
        role: "admin".to_string(),
    };
    
    let input = CreateActivityInput {
        title: proto_req.title,
        content: proto_req.content,
        cover_url: proto_req.cover_url,
        activity_type: proto_req.activity_type.map(|t| t as i32),
        location: proto_req.location,
        organizer: proto_req.organizer,
        start_time: chrono::DateTime::parse_from_rfc3339(&proto_req.start_time)
            .map_err(|_| AppError::BadRequest("Invalid start_time format".to_string()))?
            .with_timezone(&chrono::Utc),
        end_time: chrono::DateTime::parse_from_rfc3339(&proto_req.end_time)
            .map_err(|_| AppError::BadRequest("Invalid end_time format".to_string()))?
            .with_timezone(&chrono::Utc),
        quota: proto_req.quota,
        need_sign_in: proto_req.need_sign_in,
    };

    let result = service::create_activity(&state.pool, input).await?;
    let proto_activities: Vec<proto::Activity> = result.into_iter().map(activity_to_proto).collect();
    
    let response = proto::CreateActivityResponse {
        code: 200,
        message: "创建成功".to_string(),
        data: proto_activities,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 更新活动（管理员）
pub async fn update_activity(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    _auth_user: AuthUser,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::UpdateActivityRequest::decode(body)?;
    
    let input = UpdateActivityInput {
        title: proto_req.title,
        content: proto_req.content,
        cover_url: proto_req.cover_url,
        activity_type: proto_req.activity_type.map(|t| t as i32),
        location: proto_req.location,
        organizer: proto_req.organizer,
        start_time: proto_req.start_time.map(|time_str| {
            chrono::DateTime::parse_from_rfc3339(&time_str)
                .map_err(|_| AppError::BadRequest("Invalid start_time format".to_string()))
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }).transpose()?,
        end_time: proto_req.end_time.map(|time_str| {
            chrono::DateTime::parse_from_rfc3339(&time_str)
                .map_err(|_| AppError::BadRequest("Invalid end_time format".to_string()))
                .map(|dt| dt.with_timezone(&chrono::Utc))
        }).transpose()?,
        quota: proto_req.quota,
        need_sign_in: proto_req.need_sign_in,
        status: proto_req.status.map(|s| s as i32),
    };

    let _result = service::update_activity(&state.pool, &activity_id, input).await?;
    
    let response = proto::UpdateActivityResponse {
        code: 200,
        message: "更新成功".to_string(),
        data: None,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 获取报名列表（管理员）
pub async fn list_signups(
    State(state): State<AppState>,
    Path(activity_id): Path<String>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let result = service::get_enrollments(&state.pool, &activity_id).await?;
    
    let proto_enrollments: Vec<proto::EnrollmentRecord> = result.enrollment_list
        .into_iter()
        .map(|record| proto::EnrollmentRecord {
            user_id: record.user_id,
            user_name: record.user_name,
            student_id: record.student_id,
            major: record.major,
            phone_number: record.phone_number,
            activity_id: record.activity_id,
            enroll_time: record.enroll_time.to_rfc3339(),
            attendance_status: record.attendance_status,
        })
        .collect();

    let response = proto::GetEnrollmentsResponse {
        code: 200,
        message: "success".to_string(),
        data: Some(proto::GetEnrollmentsData {
            total_enrolled: result.total_enrolled,
            enrollment_list: proto_enrollments,
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 审核活动（管理员）
pub async fn admin_review_activity(
    State(_state): State<AppState>,
    Path(_activity_id): Path<String>,
    auth_user: Option<AuthUser>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let _proto_req = proto::UpdateActivityRequest::decode(body)?;
    let _user = get_or_fake_admin(auth_user);
    
    // 这里应该调用相应的 service 函数来更新活动状态
    // 暂时返回成功响应
    let response = proto::UpdateActivityResponse {
        code: 200,
        message: "审核成功".to_string(),
        data: None,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 封禁活动（管理员）
pub async fn admin_block_activity(
    State(_state): State<AppState>,
    Path(_activity_id): Path<String>,
    auth_user: Option<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let _user = get_or_fake_admin(auth_user);
    
    // 这里应该调用相应的 service 函数来封禁活动
    // 暂时返回成功响应
    let response = proto::UpdateActivityResponse {
        code: 200,
        message: "封禁成功".to_string(),
        data: None,
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}