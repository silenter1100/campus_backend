use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use prost::Message;
use sqlx::MySqlPool;

use crate::common::{auth::AuthUser, AppError};
use crate::common::state::AppState;

use super::{entity, service};

// Protobuf
mod proto;
use proto::*;

/// 注册课程模块路由
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/semesters", get(get_semesters_handler))
        .route("/api/v1/courses", get(get_public_courses_handler))
        .route("/api/v1/schedule", get(get_schedule_handler))
        .route("/api/v1/schedule", post(add_schedule_items_handler))
        .route("/api/v1/schedule", patch(update_schedule_item_handler))
        .route("/api/v1/schedule", delete(delete_schedule_item_handler))
}

/// 获取学期列表
async fn get_semesters_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let semesters = service::get_semesters(&state.pool).await?;

    let proto_semesters: Vec<Semester> = semesters
        .into_iter()
        .map(|s| Semester {
            id: s.id,
            name: s.name,
            start_date: s.start_date,
            end_date: s.end_date,
            is_current: s.is_current,
        })
        .collect();

    let response = GetSemestersResponse {
        code: 200,
        message: "成功".to_string(),
        data: Some(GetSemestersData { semesters: proto_semesters }),
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        response.encode_to_vec(),
    ))
}

/// 获取全校课程
async fn get_public_courses_handler(
    State(state): State<AppState>,
    Query(query): Query<GetCoursesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let params = entity::GetCoursesParams {
        semester_id: query.semester_id,
        name: query.name,
        teacher: query.teacher,
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(20),
    };

    let (courses, pagination) = service::get_public_courses(&state.pool, params).await?;

    let proto_courses: Vec<PublicCourse> = courses
        .into_iter()
        .map(|c| PublicCourse {
            id: c.id,
            course_name: c.course_name,
            teacher_name: c.teacher_name,
            teacher_id: c.teacher_id,
            location: c.location,
            day_of_week: c.day_of_week,
            start_section: c.start_section,
            end_section: c.end_section,
            weeks_range: c.weeks_range,
            r#type: c.r#type,
            credits: c.credits,
            description: c.description,
        })
        .collect();

    let response = GetPublicCoursesResponse {
        code: 200,
        message: "成功".to_string(),
        data: Some(GetPublicCoursesData {
            list: proto_courses,
            pagination: Some(Pagination {
                total: pagination.total,
                page: pagination.page,
                page_size: pagination.page_size,
                pages: pagination.pages,
            }),
        }),
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        response.encode_to_vec(),
    ))
}

/// 获取用户课表
async fn get_schedule_handler(
    State(state): State<AppState>,
    Query(query): Query<GetScheduleQuery>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user_id = auth_user.user_id;

    let items = service::get_user_schedule(&state.pool, user_id, query.semester_id, query.week).await?;

    let proto_items: Vec<ScheduleItem> = items
        .into_iter()
        .map(|item| ScheduleItem {
            id: item.id,
            source_id: item.source_id,
            course_name: item.course_name,
            teacher_name: item.teacher_name,
            location: item.location,
            day_of_week: item.day_of_week,
            start_section: item.start_section,
            end_section: item.end_section,
            weeks: item.weeks_range,
            r#type: item.r#type,
            credits: item.credits,
            description: item.description,
            color_hex: item.color_hex,
            is_custom: item.is_custom,
        })
        .collect();

    let response = GetScheduleResponse {
        code: 200,
        message: "成功".to_string(),
        data: Some(GetScheduleData { items: proto_items }),
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        response.encode_to_vec(),
    ))
}

/// 批量增加课表项
async fn add_schedule_items_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    body: axum::body::Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = AddScheduleItemsRequest::decode(body)?;

    let items: Vec<entity::ScheduleItemInput> = proto_req
        .items
        .into_iter()
        .map(|item| entity::ScheduleItemInput {
            source_id: item.source_id,
            course_name: item.course_name,
            teacher_name: item.teacher_name,
            location: item.location,
            day_of_week: item.day_of_week,
            start_section: item.start_section,
            end_section: item.end_section,
            weeks: item.weeks,
            r#type: item.r#type,
            credits: item.credits,
            description: item.description,
            color_hex: item.color_hex,
            is_custom: item.is_custom,
        })
        .collect();

    let result = service::add_schedule_items(&state.pool, auth_user.user_id, proto_req.semester_id, items).await?;

    let successful_items: Vec<ScheduleItem> = result.successful_items
        .into_iter()
        .map(|item| ScheduleItem {
            id: item.id,
            source_id: item.source_id,
            course_name: item.course_name,
            teacher_name: item.teacher_name,
            location: item.location,
            day_of_week: item.day_of_week,
            start_section: item.start_section,
            end_section: item.end_section,
            weeks: item.weeks_range,
            r#type: item.r#type,
            credits: item.credits,
            description: item.description,
            color_hex: item.color_hex,
            is_custom: item.is_custom,
        })
        .collect();

    let failed_items: Vec<FailedItem> = result.failed_items
        .into_iter()
        .map(|item| FailedItem {
            course_name: item.course_name,
            error_message: item.error_message,
        })
        .collect();

    let response = AddScheduleItemsResponse {
        code: 200,
        message: "处理完成".into(),
        data: Some(AddScheduleItemsData {
            successful_items,
            failed_items,
        }),
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        response.encode_to_vec(),
    ))
}

/// 更新课表项
async fn update_schedule_item_handler(
    State(state): State<AppState>,
    Query(query): Query<ItemIdQuery>,
    auth_user: AuthUser,
    body: axum::body::Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = UpdateScheduleItemRequest::decode(body)?;

    let input = entity::UpdateScheduleItemInput {
        course_name: proto_req.course_name,
        teacher_name: proto_req.teacher_name,
        location: proto_req.location,
        day_of_week: proto_req.day_of_week,
        start_section: proto_req.start_section,
        end_section: proto_req.end_section,
        weeks: if proto_req.weeks.is_empty() { None } else { Some(proto_req.weeks) },
        r#type: proto_req.r#type,
        credits: proto_req.credits,
        description: proto_req.description,
        color_hex: proto_req.color_hex,
    };

    let item = service::update_schedule_item(
        &state.pool,
        auth_user.user_id,
        query.item_id,
        input,
    )
        .await?;

    let proto_item = ScheduleItem {
        id: item.id,
        source_id: item.source_id,
        course_name: item.course_name,
        teacher_name: item.teacher_name,
        location: item.location,
        day_of_week: item.day_of_week,
        start_section: item.start_section,
        end_section: item.end_section,
        weeks: item.weeks_range,
        r#type: item.r#type,
        credits: item.credits,
        description: item.description,
        color_hex: item.color_hex,
        is_custom: item.is_custom,
    };

    let response = UpdateScheduleItemResponse {
        code: 200,
        message: "更新成功".to_string(),
        data: Some(UpdateScheduleItemData { item: Some(proto_item) }),
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        response.encode_to_vec(),
    ))
}

/// 删除课表项
async fn delete_schedule_item_handler(
    State(state): State<AppState>,
    Query(query): Query<ItemIdQuery>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    service::delete_schedule_item(&state.pool, auth_user.user_id, query.item_id).await?;

    let response = DeleteScheduleItemResponse {
        code: 200,
        message: "删除成功".to_string(),
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        response.encode_to_vec(),
    ))
}
