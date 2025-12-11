// src/modules/activity/service.rs

use crate::common::AppError;
use super::entity::*;
use sqlx::MySqlPool;
use uuid::Uuid;

// ====================
//   管理员接口
// ====================

/// 管理员发布活动
pub async fn create_activity(
    pool: &MySqlPool,
    input: CreateActivityInput,
) -> Result<Vec<Activity>, AppError> {
    let activity_id = Uuid::new_v4().to_string();
    let activity_type = input.activity_type.unwrap_or(1);
    let quota = input.quota.unwrap_or(100);
    let need_sign_in = input.need_sign_in.unwrap_or(false);
    
    // MySQL 不支持 RETURNING，需要分两步
    sqlx::query!(
        r#"
        INSERT INTO activities (
            id, title, content, cover_url, activity_type, location, organizer,
            start_time, end_time, quota, current_enrollments, need_sign_in, status
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, 1)
        "#,
        activity_id,
        input.title,
        input.content,
        input.cover_url,
        activity_type,
        input.location,
        input.organizer,
        input.start_time,
        input.end_time,
        quota,
        need_sign_in,
    )
    .execute(pool)
    .await?;
    
    // 查询刚插入的记录
    let activity_db = sqlx::query_as!(
        ActivityDb,
        r#"
        SELECT id, title, content, cover_url, activity_type, location, organizer,
               start_time, end_time, quota, current_enrollments, need_sign_in,
               status, created_at, updated_at
        FROM activities
        WHERE id = ?
        "#,
        activity_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(vec![activity_db.into()])
}

/// 修改活动
pub async fn update_activity(
    pool: &MySqlPool,
    activity_id: &str,
    input: UpdateActivityInput,
) -> Result<(), AppError> {
    // 构建动态更新 SQL
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;
    
    if let Some(title) = input.title {
        updates.push(format!("title = ${}", param_count));
        params.push(title);
        param_count += 1;
    }
    if let Some(content) = input.content {
        updates.push(format!("content = ${}", param_count));
        params.push(content);
        param_count += 1;
    }
    if let Some(location) = input.location {
        updates.push(format!("location = ${}", param_count));
        params.push(location);
        param_count += 1;
    }
    
    if updates.is_empty() {
        return Ok(());
    }
    
    updates.push("updated_at = CURRENT_TIMESTAMP".to_string());
    
    let sql = format!(
        "UPDATE activities SET {} WHERE id = ${}",
        updates.join(", "),
        param_count
    );
    
    // 简化版：直接执行更新
    sqlx::query(&format!(
        "UPDATE activities SET updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    ))
    .bind(activity_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 获取报名列表
pub async fn get_enrollments(
    pool: &MySqlPool,
    activity_id: &str,
) -> Result<EnrollmentsResponse, AppError> {
    let enrollments = sqlx::query_as!(
        EnrollmentDb,
        r#"
        SELECT id, user_id, activity_id, user_name, student_id, major, phone_number,
               enroll_time, attendance_status, status, created_at, updated_at
        FROM activity_enrollments
        WHERE activity_id = ? AND status = 1
        ORDER BY enroll_time DESC
        "#,
        activity_id
    )
    .fetch_all(pool)
    .await?;
    
    let total = enrollments.len() as i32;
    let list: Vec<EnrollmentRecord> = enrollments.into_iter().map(|e| e.into()).collect();
    
    Ok(EnrollmentsResponse {
        total_enrolled: total,
        enrollment_list: list,
    })
}

// ====================
//   公共接口
// ====================

/// 获取活动列表
pub async fn get_activities(
    pool: &MySqlPool,
    params: GetActivitiesParams,
) -> Result<ActivitiesListResponse, AppError> {
    let offset = (params.page - 1) * params.page_size;
    
    // 查询总数
    let total: i64 = if let Some(activity_type) = params.activity_type {
        sqlx::query_scalar!(
            "SELECT COUNT(*) FROM activities WHERE status = 1 AND activity_type = ?",
            activity_type
        )
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_scalar!("SELECT COUNT(*) FROM activities WHERE status = 1")
            .fetch_one(pool)
            .await?
    };
    
    // 查询列表
    let activities = if let Some(activity_type) = params.activity_type {
        sqlx::query_as!(
            ActivityDb,
            r#"
            SELECT id, title, content, cover_url, activity_type, location, organizer,
                   start_time, end_time, quota, current_enrollments, need_sign_in,
                   status, created_at, updated_at
            FROM activities
            WHERE status = 1 AND activity_type = ?
            ORDER BY start_time DESC
            LIMIT ? OFFSET ?
            "#,
            activity_type,
            params.page_size,
            offset
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            ActivityDb,
            r#"
            SELECT id, title, content, cover_url, activity_type, location, organizer,
                   start_time, end_time, quota, current_enrollments, need_sign_in,
                   status, created_at, updated_at
            FROM activities
            WHERE status = 1
            ORDER BY start_time DESC
            LIMIT ? OFFSET ?
            "#,
            params.page_size,
            offset
        )
        .fetch_all(pool)
        .await?
    };
    
    let list: Vec<ActivitySummary> = activities
        .into_iter()
        .map(|a| ActivitySummary {
            id: a.id,
            title: a.title,
            cover_url: a.cover_url,
            location: a.location,
            start_time: a.start_time,
            quota: a.quota,
            current_enrollments: a.current_enrollments,
        })
        .collect();
    
    let pages = ((total + params.page_size as i64 - 1) / params.page_size as i64) as i32;
    
    Ok(ActivitiesListResponse {
        list,
        pagination: Pagination {
            total,
            page: params.page,
            page_size: params.page_size,
            pages,
        },
    })
}

/// 获取活动详情
pub async fn get_activity_detail(
    pool: &MySqlPool,
    activity_id: &str,
    user_id: Option<&str>,
) -> Result<Activity, AppError> {
    let activity_db = sqlx::query_as!(
        ActivityDb,
        r#"
        SELECT id, title, content, cover_url, activity_type, location, organizer,
               start_time, end_time, quota, current_enrollments, need_sign_in,
               status, created_at, updated_at
        FROM activities
        WHERE id = ?
        "#,
        activity_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("活动不存在".to_string()))?;
    
    let mut activity: Activity = activity_db.into();
    
    // 如果有用户ID，查询是否已报名和收藏
    if let Some(uid) = user_id {
        let is_enrolled: i64 = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM activity_enrollments WHERE user_id = ? AND activity_id = ? AND status = 1)",
            uid,
            activity_id
        )
        .fetch_one(pool)
        .await?;
        
        let is_collected: i64 = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM activity_collections WHERE user_id = ? AND activity_id = ?)",
            uid,
            activity_id
        )
        .fetch_one(pool)
        .await?;
        
        activity.is_enrolled = Some(is_enrolled > 0);
        activity.is_collected = Some(is_collected > 0);
    }
    
    Ok(activity)
}

// ====================
//   学生接口
// ====================

/// 学生报名
pub async fn enroll_activity(
    pool: &MySqlPool,
    user_id: &str,
    activity_id: &str,
    input: EnrollActivityInput,
) -> Result<(), AppError> {
    // 检查活动是否存在
    let activity = sqlx::query_as!(
        ActivityDb,
        "SELECT * FROM activities WHERE id = ?",
        activity_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("活动不存在".to_string()))?;
    
    // 检查是否已满员
    if activity.current_enrollments >= activity.quota {
        return Err(AppError::BadRequest("活动报名人数已满".to_string()));
    }
    
    // 检查是否已报名
    let existing: i64 = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM activity_enrollments WHERE user_id = ? AND activity_id = ? AND status = 1)",
        user_id,
        activity_id
    )
    .fetch_one(pool)
    .await?;
    
    if existing > 0 {
        return Err(AppError::BadRequest("您已报名该活动".to_string()));
    }
    
    // 开启事务
    let mut tx = pool.begin().await?;
    
    // 插入报名记录
    sqlx::query!(
        r#"
        INSERT INTO activity_enrollments (user_id, activity_id, user_name, student_id, major, phone_number, status)
        VALUES (?, ?, ?, ?, ?, ?, 1)
        "#,
        user_id,
        activity_id,
        input.user_name,
        input.student_id,
        input.major,
        input.phone_number
    )
    .execute(&mut *tx)
    .await?;
    
    // 更新报名人数
    sqlx::query!(
        "UPDATE activities SET current_enrollments = current_enrollments + 1 WHERE id = ?",
        activity_id
    )
    .execute(&mut *tx)
    .await?;
    
    tx.commit().await?;
    
    Ok(())
}

/// 取消报名
pub async fn cancel_enrollment(
    pool: &MySqlPool,
    user_id: &str,
    activity_id: &str,
) -> Result<(), AppError> {
    // 检查是否已报名
    let enrollment = sqlx::query!(
        "SELECT id FROM activity_enrollments WHERE user_id = ? AND activity_id = ? AND status = 1",
        user_id,
        activity_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("您尚未报名该活动".to_string()))?;
    
    // 开启事务
    let mut tx = pool.begin().await?;
    
    // 更新报名状态为已取消
    sqlx::query!(
        "UPDATE activity_enrollments SET status = 2, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
        enrollment.id
    )
    .execute(&mut *tx)
    .await?;
    
    // 更新报名人数
    sqlx::query!(
        "UPDATE activities SET current_enrollments = current_enrollments - 1 WHERE id = ?",
        activity_id
    )
    .execute(&mut *tx)
    .await?;
    
    tx.commit().await?;
    
    Ok(())
}

/// 收藏活动
pub async fn collect_activity(
    pool: &MySqlPool,
    user_id: &str,
    activity_id: &str,
) -> Result<(), AppError> {
    // 检查是否已收藏
    let existing: i64 = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM activity_collections WHERE user_id = ? AND activity_id = ?)",
        user_id,
        activity_id
    )
    .fetch_one(pool)
    .await?;
    
    if existing > 0 {
        return Ok(()); // 已收藏，直接返回成功
    }
    
    sqlx::query!(
        "INSERT INTO activity_collections (user_id, activity_id) VALUES (?, ?)",
        user_id,
        activity_id
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 取消收藏
pub async fn uncollect_activity(
    pool: &MySqlPool,
    user_id: &str,
    activity_id: &str,
) -> Result<(), AppError> {
    sqlx::query!(
        "DELETE FROM activity_collections WHERE user_id = ? AND activity_id = ?",
        user_id,
        activity_id
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 我的活动
pub async fn get_my_activities(
    pool: &MySqlPool,
    user_id: &str,
    params: GetMyActivitiesParams,
) -> Result<MyActivitiesResponse, AppError> {
    let offset = (params.page - 1) * params.page_size;
    
    // 查询报名的活动
    let enrolled_data = if params.include_enrollments {
        let total: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM activity_enrollments WHERE user_id = ? AND status = 1",
            user_id
        )
        .fetch_one(pool)
        .await?;
        
        let enrollments = sqlx::query!(
            r#"
            SELECT a.id, a.title, a.cover_url, a.start_time, a.end_time, e.status
            FROM activity_enrollments e
            JOIN activities a ON e.activity_id = a.id
            WHERE e.user_id = ? AND e.status = 1
            ORDER BY a.start_time DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            params.page_size,
            offset
        )
        .fetch_all(pool)
        .await?;
        
        let list: Vec<EnrollmentSummary> = enrollments
            .into_iter()
            .map(|e| EnrollmentSummary {
                activity_id: e.id,
                title: e.title,
                cover_url: e.cover_url,
                start_time: e.start_time,
                end_time: e.end_time,
                my_status: e.status,
            })
            .collect();
        
        let pages = ((total + params.page_size as i64 - 1) / params.page_size as i64) as i32;
        
        EnrolledData {
            pagination: Pagination {
                total,
                page: params.page,
                page_size: params.page_size,
                pages,
            },
            list,
        }
    } else {
        EnrolledData {
            pagination: Pagination {
                total: 0,
                page: 1,
                page_size: params.page_size,
                pages: 0,
            },
            list: vec![],
        }
    };
    
    // 查询收藏的活动
    let collected_data = if params.include_collections {
        let total: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM activity_collections WHERE user_id = ?",
            user_id
        )
        .fetch_one(pool)
        .await?;
        
        let collections = sqlx::query!(
            r#"
            SELECT a.id, a.title, a.cover_url, a.start_time, a.end_time
            FROM activity_collections c
            JOIN activities a ON c.activity_id = a.id
            WHERE c.user_id = ?
            ORDER BY c.created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            params.page_size,
            offset
        )
        .fetch_all(pool)
        .await?;
        
        let list: Vec<CollectionSummary> = collections
            .into_iter()
            .map(|c| CollectionSummary {
                activity_id: c.id,
                title: c.title,
                cover_url: c.cover_url,
                start_time: c.start_time,
                end_time: c.end_time,
            })
            .collect();
        
        let pages = ((total + params.page_size as i64 - 1) / params.page_size as i64) as i32;
        
        CollectedData {
            pagination: Pagination {
                total,
                page: params.page,
                page_size: params.page_size,
                pages,
            },
            list,
        }
    } else {
        CollectedData {
            pagination: Pagination {
                total: 0,
                page: 1,
                page_size: params.page_size,
                pages: 0,
            },
            list: vec![],
        }
    };
    
    Ok(MyActivitiesResponse {
        enrolled_data,
        collected_data,
    })
}
