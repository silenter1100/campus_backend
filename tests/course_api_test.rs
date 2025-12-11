mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use campus_backend::modules::course;
use campus_backend::common::state::{AppState, JwtConfig};
use prost::Message;
use serial_test::serial;
use tower::ServiceExt;
use std::sync::Arc;

// 引入生成的 Protobuf 代码
mod proto {
    include!(concat!(env!("OUT_DIR"), "/campus.course.rs"));
}

/// 创建测试应用
async fn create_test_app(pool: sqlx::MySqlPool) -> Router {
    let jwt_config = Arc::new(JwtConfig {
        secret: "test-secret-key".to_string(),
        expiration: 86400,
    });
    
    let app_state = AppState {
        jwt_config,
        pool,
        upload_service: None,
    };
    
    course::routes().with_state(app_state)
}

/// 测试获取学期列表 API
#[tokio::test]
#[serial]
async fn test_api_get_semesters() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    common::create_test_semester(&pool).await;
    
    // 创建测试应用
    let app = create_test_app(pool.clone()).await;
    
    // 发送请求
    let request = Request::builder()
        .uri("/api/v1/semesters")
        .header("Accept", "application/x-protobuf")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    // 验证响应
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/x-protobuf"
    );
    
    // 解析响应
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let proto_response = proto::GetSemestersResponse::decode(body_bytes).unwrap();
    
    assert_eq!(proto_response.code, 200);
    assert_eq!(proto_response.message, "成功");
    assert!(!proto_response.data.unwrap().semesters.is_empty());
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试获取全校课程 API
#[tokio::test]
#[serial]
async fn test_api_get_public_courses() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    common::create_test_course(&pool, semester_id).await;
    
    // 创建测试应用
    let app = create_test_app(pool.clone()).await;
    
    // 发送请求
    let request = Request::builder()
        .uri("/api/v1/courses?page=1&pageSize=10")
        .header("Accept", "application/x-protobuf")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    // 验证响应
    assert_eq!(response.status(), StatusCode::OK);
    
    // 解析响应
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let proto_response = proto::GetPublicCoursesResponse::decode(body_bytes).unwrap();
    
    assert_eq!(proto_response.code, 200);
    let data = proto_response.data.unwrap();
    assert!(!data.list.is_empty());
    assert_eq!(data.list[0].course_name, "测试课程");
    
    let pagination = data.pagination.unwrap();
    assert_eq!(pagination.total, 1);
    assert_eq!(pagination.page, 1);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试获取用户课表 API
#[tokio::test]
#[serial]
async fn test_api_get_schedule() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let user_id = "test-user-1";
    common::create_test_schedule_item(&pool, user_id, semester_id).await;
    
    // 创建测试应用
    let app = create_test_app(pool.clone()).await;
    
    // 生成测试 token
    let token = common::generate_test_token(user_id);
    
    // 发送请求
    let request = Request::builder()
        .uri(format!("/api/v1/schedule?semester_id={}", semester_id))
        .header("Accept", "application/x-protobuf")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    // 验证响应
    assert_eq!(response.status(), StatusCode::OK);
    
    // 解析响应
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let proto_response = proto::GetScheduleResponse::decode(body_bytes).unwrap();
    
    assert_eq!(proto_response.code, 200);
    let data = proto_response.data.unwrap();
    assert_eq!(data.items.len(), 1);
    assert_eq!(data.items[0].course_name, "测试课表项");
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试添加课表项 API
#[tokio::test]
#[serial]
async fn test_api_add_schedule_items() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    
    // 创建测试应用
    let app = create_test_app(pool.clone()).await;
    
    // 构造请求
    let proto_request = proto::AddScheduleItemsRequest {
        semester_id,
        items: vec![
            proto::ScheduleItemInput {
                source_id: None,
                course_name: "API测试课程".to_string(),
                teacher_name: Some("张老师".to_string()),
                location: Some("A101".to_string()),
                day_of_week: 6,
                start_section: 1,
                end_section: 2,
                weeks: vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],
                r#type: Some("必修".to_string()),
                credits: Some(3),
                description: None,
                color_hex: "#FF5733".to_string(),
                is_custom: true,
            },
        ],
    };
    
    let request_bytes = proto_request.encode_to_vec();
    
    // 生成测试 token
    let token = common::generate_test_token("test-user-1");
    
    // 发送请求
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/schedule")
        .header("Content-Type", "application/x-protobuf")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(request_bytes))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    // 验证响应
    assert_eq!(response.status(), StatusCode::OK);
    
    // 解析响应
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let proto_response = proto::AddScheduleItemsResponse::decode(body_bytes).unwrap();
    
    assert_eq!(proto_response.code, 200);
    let data = proto_response.data.unwrap();
    assert_eq!(data.successful_items.len(), 1);
    assert_eq!(data.failed_items.len(), 0);
    assert_eq!(data.successful_items[0].course_name, "API测试课程");
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试更新课表项 API
#[tokio::test]
#[serial]
async fn test_api_update_schedule_item() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let user_id = "test-user-1";
    let item_id = common::create_test_schedule_item(&pool, user_id, semester_id).await;
    
    // 创建测试应用
    let app = create_test_app(pool.clone()).await;
    
    // 构造请求
    let proto_request = proto::UpdateScheduleItemRequest {
        item_id: item_id,
        course_name: Some("API更新课程".to_string()),
        teacher_name: Some("新老师".to_string()),
        location: Some("B202".to_string()),
        day_of_week: Some(2),
        start_section: Some(3),
        end_section: Some(4),
        weeks: vec![1,2,3,4,5,6,7,8,9,10,11,12],
        r#type: Some("选修".to_string()),
        credits: Some(4),
        description: Some("已更新".to_string()),
        color_hex: Some("#00FF00".to_string()),
    };
    
    let request_bytes = proto_request.encode_to_vec();
    
    // 生成测试 token
    let token = common::generate_test_token(user_id);
    
    // 发送请求
    let request = Request::builder()
        .method("PATCH")
        .uri(format!("/api/v1/schedule?item_id={}", item_id))
        .header("Content-Type", "application/x-protobuf")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(request_bytes))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    // 验证响应
    assert_eq!(response.status(), StatusCode::OK);
    
    // 解析响应
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let proto_response = proto::UpdateScheduleItemResponse::decode(body_bytes).unwrap();
    
    assert_eq!(proto_response.code, 200);
    let item = proto_response.data.unwrap().item.unwrap();
    assert_eq!(item.course_name, "API更新课程");
    assert_eq!(item.teacher_name, Some("新老师".to_string()));
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试删除课表项 API
#[tokio::test]
#[serial]
async fn test_api_delete_schedule_item() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let user_id = "test-user-1";
    let item_id = common::create_test_schedule_item(&pool, user_id, semester_id).await;
    
    // 创建测试应用
    let app = create_test_app(pool.clone()).await;
    
    // 生成测试 token
    let token = common::generate_test_token(user_id);
    
    // 发送请求
    let request = Request::builder()
        .method("DELETE")
        .uri(format!("/api/v1/schedule?item_id={}", item_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    
    // 验证响应
    assert_eq!(response.status(), StatusCode::OK);
    
    // 解析响应
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    
    let proto_response = proto::DeleteScheduleItemResponse::decode(body_bytes).unwrap();
    
    assert_eq!(proto_response.code, 200);
    assert_eq!(proto_response.message, "删除成功");
    
    // 清理
    common::cleanup_test_data(&pool).await;
}
