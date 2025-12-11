use sqlx::{MySql, Pool};
use std::sync::Once;

static INIT: Once = Once::new();

/// 初始化测试环境（只执行一次）
pub fn init_test_env() {
    INIT.call_once(|| {
        // 加载测试环境变量
        dotenv::from_filename(".env.test").ok();
        
        // 初始化日志
        tracing_subscriber::fmt()
            .with_test_writer()
            .with_env_filter("debug")
            .try_init()
            .ok();
    });
}

/// 创建测试数据库连接池
pub async fn create_test_pool() -> Pool<MySql> {
    init_test_env();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");
    
    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create test database pool")
}

/// 清理测试数据
pub async fn cleanup_test_data(pool: &Pool<MySql>) {
    sqlx::query("DELETE FROM schedule_items")
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM public_courses")
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM semesters")
        .execute(pool)
        .await
        .ok();
}

/// 创建测试学期
pub async fn create_test_semester(pool: &Pool<MySql>) -> i64 {
    let result = sqlx::query(
        "INSERT INTO semesters (name, start_date, end_date, is_current) 
         VALUES (?, ?, ?, ?)"
    )
    .bind("2024-2025学年第一学期")
    .bind("2024-09-01")
    .bind("2025-01-15")
    .bind(true)
    .execute(pool)
    .await
    .expect("Failed to create test semester");
    
    result.last_insert_id() as i64
}

/// 创建测试课程
pub async fn create_test_course(pool: &Pool<MySql>, semester_id: i64) -> i64 {
    let result = sqlx::query(
        "INSERT INTO public_courses 
         (semester_id, course_name, teacher_name, teacher_id, location, 
          day_of_week, start_section, end_section, weeks_range, type, credits) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(semester_id)
    .bind("测试课程")
    .bind("测试老师")
    .bind(10001)  // 改为数字类型
    .bind("A101")
    .bind(1)
    .bind(1)
    .bind(2)
    .bind("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]")  // JSON 数组格式
    .bind("必修")
    .bind(3)
    .execute(pool)
    .await
    .expect("Failed to create test course");
    
    result.last_insert_id() as i64
}

/// 创建测试课表项
pub async fn create_test_schedule_item(
    pool: &Pool<MySql>,
    user_id: &str,
    semester_id: i64,
) -> i64 {
    let result = sqlx::query(
        "INSERT INTO schedule_items 
         (user_id, semester_id, course_name, teacher_name, location, 
          day_of_week, start_section, end_section, weeks_range, type, credits, is_custom) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(user_id)
    .bind(semester_id)
    .bind("测试课表项")
    .bind("测试老师")
    .bind("B202")
    .bind(2)
    .bind(3)
    .bind(4)
    .bind("[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]")  // JSON 数组格式
    .bind("选修")
    .bind(2)
    .bind(false)
    .execute(pool)
    .await
    .expect("Failed to create test schedule item");
    
    result.last_insert_id() as i64
}
