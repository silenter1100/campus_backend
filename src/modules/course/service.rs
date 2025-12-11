use crate::common::AppError;
use super::entity::*;
use sqlx::{MySqlPool, Row, types::Json};

// ==================== 学期相关 ====================

/// 获取学期列表
pub async fn get_semesters(pool: &MySqlPool) -> Result<Vec<Semester>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT id, name, start_date, end_date, is_current
        FROM semesters
        ORDER BY is_current DESC, start_date DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let semesters = rows.into_iter().map(|row| {
        Semester {
            id: row.get("id"),
            name: row.get("name"),
            start_date: row.get("start_date"),
            end_date: row.get("end_date"),
            is_current: row.get("is_current"),
        }
    }).collect();

    Ok(semesters)
}

// ==================== 全校课程相关 ====================

/// 获取全校课程列表（支持分页和筛选）
pub async fn get_public_courses(
    pool: &MySqlPool,
    params: GetCoursesParams,
) -> Result<(Vec<PublicCourse>, Pagination), AppError> {
    // 构建查询条件
    let mut conditions = Vec::new();
    
    if params.semester_id.is_some() {
        conditions.push("semester_id = ?");
    }
    if params.name.is_some() {
        conditions.push("course_name LIKE CONCAT('%', ?, '%')");
    }
    if params.teacher.is_some() {
        conditions.push("teacher_name LIKE CONCAT('%', ?, '%')");
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // 计算总数
    let count_query = format!(
        "SELECT COUNT(*) as count FROM public_courses {}",
        where_clause
    );
    
    let mut count_query_builder = sqlx::query(&count_query);
    if let Some(semester_id) = params.semester_id {
        count_query_builder = count_query_builder.bind(semester_id);
    }
    if let Some(ref name) = params.name {
        count_query_builder = count_query_builder.bind(name);
    }
    if let Some(ref teacher) = params.teacher {
        count_query_builder = count_query_builder.bind(teacher);
    }
    
    let count_row = count_query_builder.fetch_one(pool).await?;
    let total: i64 = count_row.get("count");

    // 计算分页
    let offset = (params.page - 1) * params.page_size;
    let pages = ((total + params.page_size as i64 - 1) / params.page_size as i64) as i32;

    // 查询数据
    let data_query = format!(
        r#"
        SELECT id, semester_id, course_name, teacher_name, teacher_id, location,
               day_of_week, start_section, end_section, weeks_range, type, credits, description
        FROM public_courses
        {}
        ORDER BY id DESC
        LIMIT ? OFFSET ?
        "#,
        where_clause
    );

    let mut data_query_builder = sqlx::query(&data_query);
    if let Some(semester_id) = params.semester_id {
        data_query_builder = data_query_builder.bind(semester_id);
    }
    if let Some(ref name) = params.name {
        data_query_builder = data_query_builder.bind(name);
    }
    if let Some(ref teacher) = params.teacher {
        data_query_builder = data_query_builder.bind(teacher);
    }
    data_query_builder = data_query_builder.bind(params.page_size).bind(offset);

    let rows = data_query_builder.fetch_all(pool).await?;

    let courses = rows.into_iter().map(|row| {
        let weeks_json: Json<Vec<i32>> = row.get("weeks_range");
        let weeks_range = weeks_json.0;
        
        PublicCourse {
            id: row.get("id"),
            course_name: row.get("course_name"),
            teacher_name: row.get("teacher_name"),
            teacher_id: row.get("teacher_id"),
            location: row.get("location"),
            day_of_week: row.get("day_of_week"),
            start_section: row.get("start_section"),
            end_section: row.get("end_section"),
            weeks_range,
            r#type: row.get("type"),
            credits: row.get("credits"),
            description: row.get("description"),
        }
    }).collect();

    let pagination = Pagination {
        total,
        page: params.page,
        page_size: params.page_size,
        pages,
    };

    Ok((courses, pagination))
}

// ==================== 用户课表相关 ====================

/// 获取用户课表
pub async fn get_user_schedule(
    pool: &MySqlPool,
    user_id: &str,
    semester_id: i64,
    week: Option<i32>,
) -> Result<Vec<ScheduleItem>, AppError> {
    let rows = if let Some(week_num) = week {
        // 筛选指定周
        sqlx::query(
            r#"
            SELECT id, user_id, semester_id, source_id, course_name, teacher_name, location,
                   day_of_week, start_section, end_section, weeks_range, type, credits,
                   description, color_hex, is_custom
            FROM schedule_items
            WHERE user_id = ? AND semester_id = ?
              AND JSON_CONTAINS(weeks_range, ?)
            ORDER BY day_of_week, start_section
            "#
        )
        .bind(user_id)
        .bind(semester_id)
        .bind(week_num.to_string())
        .fetch_all(pool)
        .await?
    } else {
        // 返回整个学期的课表
        sqlx::query(
            r#"
            SELECT id, user_id, semester_id, source_id, course_name, teacher_name, location,
                   day_of_week, start_section, end_section, weeks_range, type, credits,
                   description, color_hex, is_custom
            FROM schedule_items
            WHERE user_id = ? AND semester_id = ?
            ORDER BY day_of_week, start_section
            "#
        )
        .bind(user_id)
        .bind(semester_id)
        .fetch_all(pool)
        .await?
    };

    let items = rows.into_iter().map(|row| {
        let weeks_json: Json<Vec<i32>> = row.get("weeks_range");
        let weeks_range = weeks_json.0;
        
        ScheduleItem {
            id: row.get("id"),
            source_id: row.get("source_id"),
            course_name: row.get("course_name"),
            teacher_name: row.get("teacher_name"),
            location: row.get("location"),
            day_of_week: row.get("day_of_week"),
            start_section: row.get("start_section"),
            end_section: row.get("end_section"),
            weeks_range,
            r#type: row.get("type"),
            credits: row.get("credits"),
            description: row.get("description"),
            color_hex: row.get("color_hex"),
            is_custom: row.get("is_custom"),
        }
    }).collect();

    Ok(items)
}

/// 检查时间冲突
async fn check_time_conflict(
    pool: &MySqlPool,
    user_id: &str,
    semester_id: i64,
    day_of_week: i32,
    start_section: i32,
    end_section: i32,
    weeks: &[i32],
    exclude_item_id: Option<i64>,
) -> Result<bool, AppError> {
    let rows = if let Some(item_id) = exclude_item_id {
        sqlx::query(
            r#"
            SELECT id, weeks_range
            FROM schedule_items
            WHERE user_id = ?
              AND semester_id = ?
              AND day_of_week = ?
              AND id != ?
              AND (
                  (start_section <= ? AND end_section >= ?)
                  OR (start_section <= ? AND end_section >= ?)
                  OR (start_section >= ? AND end_section <= ?)
              )
            "#
        )
        .bind(user_id)
        .bind(semester_id)
        .bind(day_of_week)
        .bind(item_id)
        .bind(start_section)
        .bind(start_section)
        .bind(end_section)
        .bind(end_section)
        .bind(start_section)
        .bind(end_section)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, weeks_range
            FROM schedule_items
            WHERE user_id = ?
              AND semester_id = ?
              AND day_of_week = ?
              AND (
                  (start_section <= ? AND end_section >= ?)
                  OR (start_section <= ? AND end_section >= ?)
                  OR (start_section >= ? AND end_section <= ?)
              )
            "#
        )
        .bind(user_id)
        .bind(semester_id)
        .bind(day_of_week)
        .bind(start_section)
        .bind(start_section)
        .bind(end_section)
        .bind(end_section)
        .bind(start_section)
        .bind(end_section)
        .fetch_all(pool)
        .await?
    };

    // 检查周次是否重叠
    for row in rows {
        let weeks_json: Json<Vec<i32>> = row.get("weeks_range");
        let conflict_weeks = weeks_json.0;
        if weeks.iter().any(|w| conflict_weeks.contains(w)) {
            return Ok(true); // 存在冲突
        }
    }

    Ok(false) // 无冲突
}

/// 批量添加课表项
pub async fn add_schedule_items(
    pool: &MySqlPool,
    user_id: &str,
    semester_id: i64,
    items: Vec<ScheduleItemInput>,
) -> Result<BatchAddResult, AppError> {
    let mut successful_items = Vec::new();
    let mut failed_items = Vec::new();

    for item in items {
        // 验证业务规则
        if item.is_custom && item.source_id.is_some() {
            failed_items.push(FailedItem {
                course_name: item.course_name.clone(),
                error_message: "自定义课程不能有 source_id".to_string(),
            });
            continue;
        }

        if !item.is_custom && item.source_id.is_none() {
            failed_items.push(FailedItem {
                course_name: item.course_name.clone(),
                error_message: "非自定义课程必须有 source_id".to_string(),
            });
            continue;
        }

        // 验证时间范围
        if item.start_section > item.end_section {
            failed_items.push(FailedItem {
                course_name: item.course_name.clone(),
                error_message: "开始节次不能大于结束节次".to_string(),
            });
            continue;
        }

        if item.day_of_week < 1 || item.day_of_week > 7 {
            failed_items.push(FailedItem {
                course_name: item.course_name.clone(),
                error_message: "星期几必须在 1-7 之间".to_string(),
            });
            continue;
        }

        // 检查时间冲突
        match check_time_conflict(
            pool,
            user_id,
            semester_id,
            item.day_of_week,
            item.start_section,
            item.end_section,
            &item.weeks,
            None,
        )
        .await
        {
            Ok(true) => {
                failed_items.push(FailedItem {
                    course_name: item.course_name.clone(),
                    error_message: "课程时间冲突".to_string(),
                });
                continue;
            }
            Err(e) => {
                failed_items.push(FailedItem {
                    course_name: item.course_name.clone(),
                    error_message: format!("检查冲突失败: {}", e),
                });
                continue;
            }
            Ok(false) => {}
        }

        // 插入数据库
        let weeks_json = serde_json::to_string(&item.weeks).unwrap();
        match sqlx::query(
            r#"
            INSERT INTO schedule_items
            (user_id, semester_id, source_id, course_name, teacher_name, location,
             day_of_week, start_section, end_section, weeks_range, type, credits,
             description, color_hex, is_custom)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(user_id)
        .bind(semester_id)
        .bind(item.source_id)
        .bind(&item.course_name)
        .bind(&item.teacher_name)
        .bind(&item.location)
        .bind(item.day_of_week)
        .bind(item.start_section)
        .bind(item.end_section)
        .bind(&weeks_json)
        .bind(&item.r#type)
        .bind(item.credits)
        .bind(&item.description)
        .bind(&item.color_hex)
        .bind(item.is_custom)
        .execute(pool)
        .await
        {
            Ok(result) => {
                successful_items.push(ScheduleItem {
                    id: result.last_insert_id() as i64,
                    source_id: item.source_id,
                    course_name: item.course_name,
                    teacher_name: item.teacher_name,
                    location: item.location,
                    day_of_week: item.day_of_week,
                    start_section: item.start_section,
                    end_section: item.end_section,
                    weeks_range: item.weeks,
                    r#type: item.r#type,
                    credits: item.credits,
                    description: item.description,
                    color_hex: item.color_hex,
                    is_custom: item.is_custom,
                });
            }
            Err(e) => {
                failed_items.push(FailedItem {
                    course_name: item.course_name,
                    error_message: format!("数据库插入失败: {}", e),
                });
            }
        }
    }

    Ok(BatchAddResult {
        successful_items,
        failed_items,
    })
}

/// 更新课表项
pub async fn update_schedule_item(
    pool: &MySqlPool,
    user_id: &str,
    item_id: i64,
    input: UpdateScheduleItemInput,
) -> Result<ScheduleItem, AppError> {
    // 先查询原有数据
    let row = sqlx::query(
        r#"
        SELECT id, user_id, semester_id, source_id, course_name, teacher_name, location,
               day_of_week, start_section, end_section, weeks_range, type, credits,
               description, color_hex, is_custom
        FROM schedule_items
        WHERE id = ? AND user_id = ?
        "#
    )
    .bind(item_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let existing = row.ok_or_else(|| AppError::NotFound("课表项不存在".to_string()))?;

    // 解析现有数据
    let existing_weeks_json: Json<Vec<i32>> = existing.get("weeks_range");
    let existing_weeks = existing_weeks_json.0;

    // 合并更新字段
    let course_name = input.course_name.unwrap_or_else(|| existing.get("course_name"));
    let teacher_name: Option<String> = input.teacher_name.or_else(|| existing.get("teacher_name"));
    let location: Option<String> = input.location.or_else(|| existing.get("location"));
    let day_of_week = input.day_of_week.unwrap_or_else(|| existing.get("day_of_week"));
    let start_section = input.start_section.unwrap_or_else(|| existing.get("start_section"));
    let end_section = input.end_section.unwrap_or_else(|| existing.get("end_section"));
    let weeks = input.weeks.unwrap_or(existing_weeks);
    let r#type: Option<String> = input.r#type.or_else(|| existing.get("type"));
    let credits: Option<i32> = input.credits.or_else(|| existing.get("credits"));
    let description: Option<String> = input.description.or_else(|| existing.get("description"));
    let color_hex = input.color_hex.unwrap_or_else(|| existing.get("color_hex"));

    let semester_id: i64 = existing.get("semester_id");
    let source_id: Option<i64> = existing.get("source_id");
    let is_custom: bool = existing.get("is_custom");

    // 验证时间范围
    if start_section > end_section {
        return Err(AppError::BadRequest("开始节次不能大于结束节次".to_string()));
    }

    if day_of_week < 1 || day_of_week > 7 {
        return Err(AppError::BadRequest("星期几必须在 1-7 之间".to_string()));
    }

    // 检查时间冲突（排除当前项）
    let has_conflict = check_time_conflict(
        pool,
        user_id,
        semester_id,
        day_of_week,
        start_section,
        end_section,
        &weeks,
        Some(item_id),
    )
    .await?;

    if has_conflict {
        return Err(AppError::BadRequest("课程时间冲突".to_string()));
    }

    // 更新数据库
    let weeks_json = serde_json::to_string(&weeks).unwrap();
    sqlx::query(
        r#"
        UPDATE schedule_items
        SET course_name = ?, teacher_name = ?, location = ?,
            day_of_week = ?, start_section = ?, end_section = ?,
            weeks_range = ?, type = ?, credits = ?, description = ?,
            color_hex = ?
        WHERE id = ? AND user_id = ?
        "#
    )
    .bind(&course_name)
    .bind(&teacher_name)
    .bind(&location)
    .bind(day_of_week)
    .bind(start_section)
    .bind(end_section)
    .bind(&weeks_json)
    .bind(&r#type)
    .bind(credits)
    .bind(&description)
    .bind(&color_hex)
    .bind(item_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(ScheduleItem {
        id: item_id,
        source_id,
        course_name,
        teacher_name,
        location,
        day_of_week,
        start_section,
        end_section,
        weeks_range: weeks,
        r#type,
        credits,
        description,
        color_hex,
        is_custom,
    })
}

/// 删除课表项
pub async fn delete_schedule_item(
    pool: &MySqlPool,
    user_id: &str,
    item_id: i64,
) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM schedule_items
        WHERE id = ? AND user_id = ?
        "#
    )
    .bind(item_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("课表项不存在".to_string()));
    }

    Ok(())
}
