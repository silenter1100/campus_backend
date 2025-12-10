mod common;

use campus_backend::modules::course::{entity, service};
use serial_test::serial;

/// 测试获取学期列表
#[tokio::test]
#[serial]
async fn test_get_semesters() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    
    // 执行测试
    let result = service::get_semesters(&pool).await;
    
    assert!(result.is_ok());
    let semesters = result.unwrap();
    assert!(!semesters.is_empty());
    assert_eq!(semesters[0].id, semester_id);
    assert_eq!(semesters[0].name, "2024-2025学年第一学期");
    assert!(semesters[0].is_current);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试获取全校课程（无过滤）
#[tokio::test]
#[serial]
async fn test_get_public_courses_no_filter() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let course_id = common::create_test_course(&pool, semester_id).await;
    
    // 执行测试
    let params = entity::GetCoursesParams {
        semester_id: None,
        name: None,
        teacher: None,
        page: 1,
        page_size: 20,
    };
    
    let result = service::get_public_courses(&pool, params).await;
    
    assert!(result.is_ok());
    let (courses, pagination) = result.unwrap();
    assert!(!courses.is_empty());
    assert_eq!(courses[0].id, course_id);
    assert_eq!(courses[0].course_name, "测试课程");
    assert_eq!(pagination.total, 1);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试获取全校课程（按学期过滤）
#[tokio::test]
#[serial]
async fn test_get_public_courses_filter_by_semester() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    common::create_test_course(&pool, semester_id).await;
    
    // 执行测试
    let params = entity::GetCoursesParams {
        semester_id: Some(semester_id),
        name: None,
        teacher: None,
        page: 1,
        page_size: 20,
    };
    
    let result = service::get_public_courses(&pool, params).await;
    
    assert!(result.is_ok());
    let (courses, _) = result.unwrap();
    assert_eq!(courses.len(), 1);
    
    // 测试不存在的学期
    let params = entity::GetCoursesParams {
        semester_id: Some(99999),
        name: None,
        teacher: None,
        page: 1,
        page_size: 20,
    };
    
    let result = service::get_public_courses(&pool, params).await;
    assert!(result.is_ok());
    let (courses, _) = result.unwrap();
    assert_eq!(courses.len(), 0);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试获取全校课程（按课程名过滤）
#[tokio::test]
#[serial]
async fn test_get_public_courses_filter_by_name() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    common::create_test_course(&pool, semester_id).await;
    
    // 执行测试 - 匹配
    let params = entity::GetCoursesParams {
        semester_id: None,
        name: Some("测试".to_string()),
        teacher: None,
        page: 1,
        page_size: 20,
    };
    
    let result = service::get_public_courses(&pool, params).await;
    assert!(result.is_ok());
    let (courses, _) = result.unwrap();
    assert_eq!(courses.len(), 1);
    
    // 执行测试 - 不匹配
    let params = entity::GetCoursesParams {
        semester_id: None,
        name: Some("不存在的课程".to_string()),
        teacher: None,
        page: 1,
        page_size: 20,
    };
    
    let result = service::get_public_courses(&pool, params).await;
    assert!(result.is_ok());
    let (courses, _) = result.unwrap();
    assert_eq!(courses.len(), 0);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试分页功能
#[tokio::test]
#[serial]
async fn test_pagination() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据（创建多个课程）
    let semester_id = common::create_test_semester(&pool).await;
    for i in 1..=15 {
        sqlx::query(
            "INSERT INTO public_courses 
             (semester_id, course_name, teacher_name, teacher_id, location, 
              day_of_week, start_section, end_section, weeks_range, type, credits) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(semester_id)
        .bind(format!("课程{}", i))
        .bind("老师")
        .bind(10001i64)
        .bind("A101")
        .bind(1)
        .bind(1)
        .bind(2)
        .bind("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]")
        .bind("必修")
        .bind(3)
        .execute(&pool)
        .await
        .unwrap();
    }
    
    // 测试第一页
    let params = entity::GetCoursesParams {
        semester_id: None,
        name: None,
        teacher: None,
        page: 1,
        page_size: 10,
    };
    
    let result = service::get_public_courses(&pool, params).await;
    assert!(result.is_ok());
    let (courses, pagination) = result.unwrap();
    assert_eq!(courses.len(), 10);
    assert_eq!(pagination.total, 15);
    assert_eq!(pagination.page, 1);
    assert_eq!(pagination.pages, 2);
    
    // 测试第二页
    let params = entity::GetCoursesParams {
        semester_id: None,
        name: None,
        teacher: None,
        page: 2,
        page_size: 10,
    };
    
    let result = service::get_public_courses(&pool, params).await;
    assert!(result.is_ok());
    let (courses, pagination) = result.unwrap();
    assert_eq!(courses.len(), 5);
    assert_eq!(pagination.page, 2);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试获取用户课表
#[tokio::test]
#[serial]
async fn test_get_user_schedule() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let user_id = 1;
    let item_id = common::create_test_schedule_item(&pool, user_id, semester_id).await;
    
    // 执行测试
    let result = service::get_user_schedule(&pool, user_id, semester_id, None).await;
    
    assert!(result.is_ok());
    let items = result.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, item_id);
    assert_eq!(items[0].course_name, "测试课表项");
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试添加课表项
#[tokio::test]
#[serial]
async fn test_add_schedule_items() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let user_id = 1;
    // 创建一门全校课程用于非自定义课表项的 source_id
    let course_id = common::create_test_course(&pool, semester_id).await;
    
    // 准备输入
    let items = vec![
        entity::ScheduleItemInput {
            source_id: Some(course_id),
            course_name: "新课程A".to_string(),
            teacher_name: Some("张老师".to_string()),
            location: Some("A101".to_string()),
            day_of_week: 1,
            start_section: 1,
            end_section: 2,
            weeks: vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],
            r#type: Some("必修".to_string()),
            credits: Some(3),
            description: None,
            color_hex: "#FF5733".to_string(),
            is_custom: false,
        },
        entity::ScheduleItemInput {
            source_id: None,
            course_name: "新课程B".to_string(),
            teacher_name: Some("李老师".to_string()),
            location: Some("B202".to_string()),
            day_of_week: 2,
            start_section: 3,
            end_section: 4,
            weeks: vec![1,2,3,4,5,6,7,8],
            r#type: Some("选修".to_string()),
            credits: Some(2),
            description: None,
            color_hex: "".to_string(),
            is_custom: true,
        },
    ];
    
    // 执行测试
    let result = service::add_schedule_items(&pool, user_id, semester_id, items).await;
    
    assert!(result.is_ok());
    let batch_result = result.unwrap();
    assert_eq!(batch_result.successful_items.len(), 2);
    assert_eq!(batch_result.failed_items.len(), 0);
    assert_eq!(batch_result.successful_items[0].course_name, "新课程A");
    assert_eq!(batch_result.successful_items[1].course_name, "新课程B");
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试更新课表项
#[tokio::test]
#[serial]
async fn test_update_schedule_item() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let user_id = 1;
    let item_id = common::create_test_schedule_item(&pool, user_id, semester_id).await;
    
    // 准备更新数据
    let input = entity::UpdateScheduleItemInput {
        course_name: Some("更新后的课程".to_string()),
        teacher_name: Some("新老师".to_string()),
        location: Some("C303".to_string()),
        day_of_week: Some(3),
        start_section: Some(5),
        end_section: Some(6),
        weeks: Some(vec![1,2,3,4,5,6,7,8,9,10,11,12]),
        r#type: Some("选修".to_string()),
        credits: Some(4),
        description: Some("已更新".to_string()),
        color_hex: Some("#00FF00".to_string()),
    };
    
    // 执行测试
    let result = service::update_schedule_item(&pool, user_id, item_id, input).await;
    
    assert!(result.is_ok());
    let updated_item = result.unwrap();
    assert_eq!(updated_item.id, item_id);
    assert_eq!(updated_item.course_name, "更新后的课程");
    assert_eq!(updated_item.teacher_name.unwrap(), "新老师");
    assert_eq!(updated_item.location.unwrap(), "C303");
    assert_eq!(updated_item.day_of_week, 3);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试删除课表项
#[tokio::test]
#[serial]
async fn test_delete_schedule_item() {
    let pool = common::create_test_pool().await;
    common::cleanup_test_data(&pool).await;
    
    // 创建测试数据
    let semester_id = common::create_test_semester(&pool).await;
    let user_id = 1;
    let item_id = common::create_test_schedule_item(&pool, user_id, semester_id).await;
    
    // 执行删除
    let result = service::delete_schedule_item(&pool, user_id, item_id).await;
    assert!(result.is_ok());
    
    // 验证已删除
    let check_result = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM schedule_items WHERE id = ?"
    )
    .bind(item_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert_eq!(check_result, 0);
    
    // 清理
    common::cleanup_test_data(&pool).await;
}

/// 测试删除不存在的课表项
#[tokio::test]
#[serial]
async fn test_delete_nonexistent_schedule_item() {
    let pool = common::create_test_pool().await;
    
    let user_id = 1;
    let nonexistent_id = 99999;
    
    // 执行删除
    let result = service::delete_schedule_item(&pool, user_id, nonexistent_id).await;
    
    // 应该返回错误
    assert!(result.is_err());
}
