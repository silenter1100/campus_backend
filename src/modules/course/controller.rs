use axum::{
    body::Bytes,
    extract::{Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use prost::Message;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::common::AppError;
use super::{entity, service};

// 引入生成的 Protobuf 代码
mod proto {
    include!(concat!(env!("OUT_DIR"), "/campus.course.rs"));
}

// ==================== 路由注册 ====================

pub fn routes() -> Router<MySqlPool> {
    Router::new()
        .route("/api/v1/semesters", get(get_semesters_handler))
        .route("/api/v1/courses", get(get_public_courses_handler))
        .route("/api/v1/schedule", get(get_schedule_handler))
        .route("/api/v1/schedule", post(add_schedule_items_handler))
        .route("/api/v1/schedule", patch(update_schedule_item_handler))
        .route("/api/v1/schedule", delete(delete_schedule_item_handler))
}

// ==================== 查询参数 ====================

#[derive(Debug, Deserialize)]
struct GetCoursesQuery {
    semester_id: Option<i64>,
    name: Option<String>,
    teacher: Option<String>,
    page: Option<i32>,
    #[serde(rename = "pageSize")]
    page_size: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct GetScheduleQuery {
    semester_id: i64,
    week: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct ItemIdQuery {
    item_id: i64,
}

// ==================== 处理函数 ====================

/// 获取学期列表
async fn get_semesters_handler(
    State(pool): State<MySqlPool>,
) -> Result<impl IntoResponse, AppError> {
    let semesters = service::get_semesters(&pool).await?;

    let proto_semesters: Vec<proto::Semester> = semesters
        .into_iter()
        .map(|s| proto::Semester {
            id: s.id,
            name: s.name,
            start_date: s.start_date,
            end_date: s.end_date,
            is_current: s.is_current,
        })
        .collect();

    let response = proto::GetSemestersResponse {
        code: 200,
        message: "成功".to_string(),
        data: Some(proto::GetSemestersData {
            semesters: proto_semesters,
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 获取全校课程列表
async fn get_public_courses_handler(
    State(pool): State<MySqlPool>,
    Query(query): Query<GetCoursesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let params = entity::GetCoursesParams {
        semester_id: query.semester_id,
        name: query.name,
        teacher: query.teacher,
        page: query.page.unwrap_or(1),
        page_size: query.page_size.unwrap_or(20),
    };

    let (courses, pagination) = service::get_public_courses(&pool, params).await?;

    let proto_courses: Vec<proto::PublicCourse> = courses
        .into_iter()
        .map(|c| proto::PublicCourse {
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

    let response = proto::GetPublicCoursesResponse {
        code: 200,
        message: "成功".to_string(),
        data: Some(proto::GetPublicCoursesData {
            list: proto_courses,
            pagination: Some(proto::Pagination {
                total: pagination.total,
                page: pagination.page,
                page_size: pagination.page_size,
                pages: pagination.pages,
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

/// 获取用户课表
async fn get_schedule_handler(
    State(pool): State<MySqlPool>,
    Query(query): Query<GetScheduleQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = 1; // TODO: 从 JWT 获取

    let items = service::get_user_schedule(&pool, user_id, query.semester_id, query.week).await?;

    let proto_items: Vec<proto::ScheduleItem> = items
        .into_iter()
        .map(|item| proto::ScheduleItem {
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

    let response = proto::GetScheduleResponse {
        code: 200,
        message: "成功".to_string(),
        data: Some(proto::GetScheduleData { items: proto_items }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 批量添加课表项
async fn add_schedule_items_handler(
    State(pool): State<MySqlPool>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::AddScheduleItemsRequest::decode(body)?;

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

    let user_id = 1; // TODO: 从 JWT 获取
    let result = service::add_schedule_items(&pool, user_id, proto_req.semester_id, items).await?;

    let successful_items: Vec<proto::ScheduleItem> = result
        .successful_items
        .into_iter()
        .map(|item| proto::ScheduleItem {
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

    let failed_items: Vec<proto::FailedItem> = result
        .failed_items
        .into_iter()
        .map(|item| proto::FailedItem {
            course_name: item.course_name,
            error_message: item.error_message,
        })
        .collect();

    let message = if failed_items.is_empty() {
        format!("成功添加 {} 项课程", successful_items.len())
    } else {
        format!(
            "处理完成：成功 {} 项，失败 {} 项",
            successful_items.len(),
            failed_items.len()
        )
    };

    let response = proto::AddScheduleItemsResponse {
        code: 200,
        message,
        data: Some(proto::AddScheduleItemsData {
            successful_items,
            failed_items,
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 更新课表项
async fn update_schedule_item_handler(
    State(pool): State<MySqlPool>,
    Query(query): Query<ItemIdQuery>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::UpdateScheduleItemRequest::decode(body)?;

    let input = entity::UpdateScheduleItemInput {
        course_name: proto_req.course_name,
        teacher_name: proto_req.teacher_name,
        location: proto_req.location,
        day_of_week: proto_req.day_of_week,
        start_section: proto_req.start_section,
        end_section: proto_req.end_section,
        weeks: if proto_req.weeks.is_empty() {
            None
        } else {
            Some(proto_req.weeks)
        },
        r#type: proto_req.r#type,
        credits: proto_req.credits,
        description: proto_req.description,
        color_hex: proto_req.color_hex,
    };

    let user_id = 1; // TODO: 从 JWT 获取
    let item = service::update_schedule_item(&pool, user_id, query.item_id, input).await?;

    let proto_item = proto::ScheduleItem {
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

    let response = proto::UpdateScheduleItemResponse {
        code: 200,
        message: "更新成功".to_string(),
        data: Some(proto::UpdateScheduleItemData {
            item: Some(proto_item),
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 删除课表项
async fn delete_schedule_item_handler(
    State(pool): State<MySqlPool>,
    Query(query): Query<ItemIdQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = 1; // TODO: 从 JWT 获取

    service::delete_schedule_item(&pool, user_id, query.item_id).await?;

    let response = proto::DeleteScheduleItemResponse {
        code: 200,
        message: "删除成功".to_string(),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}
