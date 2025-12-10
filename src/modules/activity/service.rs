// src/modules/activity/service.rs

use crate::modules::activity::entity::*;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use chrono::Utc; 
use sqlx::Row;
use crate::modules::activity::entity::{ActivitySignup, ActivitySignupStatus};


pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("not found")]
    NotFound,
    #[error("permission denied")]
    PermissionDenied,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("internal error")]
    Internal,
}

impl ServiceError {
    pub fn to_api_response<T>(&self) -> ApiResponse<T> {
        match self {
            ServiceError::NotFound => ApiResponse::error(40001, "活动不存在或无权访问"),
            ServiceError::PermissionDenied => ApiResponse::error(40008, "权限不足"),
            ServiceError::BadRequest(msg) => ApiResponse::error(40002, msg.clone()),
            ServiceError::Db(_) | ServiceError::Internal => {
                ApiResponse::error(50000, "服务器内部错误")
            }
        }
    }
}

#[async_trait]
pub trait ActivityService: Send + Sync + 'static {
    // 分类 / 标签
    async fn list_categories(&self) -> ServiceResult<Vec<(String, String)>>;
    async fn list_tags(&self, keyword: Option<String>) -> ServiceResult<Vec<(String, String)>>;

    // 公共活动
    async fn list_activities(
        &self,
        current_user: Option<&CurrentUser>,
        query: ListActivitiesQuery,
    ) -> ServiceResult<Paged<ActivityListItem>>;

    async fn get_activity_detail(
        &self,
        current_user: Option<&CurrentUser>,
        activity_id: i64,
    ) -> ServiceResult<Activity>;

    // 用户报名相关
    async fn signup_activity(
        &self,
        user: &CurrentUser,
        activity_id: i64,
        body: SignupActivityBody,
    ) -> ServiceResult<ActivitySignup>;

    async fn cancel_signup(
        &self,
        user: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<ActivitySignup>;

    async fn list_my_activities(
        &self,
        user: &CurrentUser,
        query: ListMyActivitiesQuery,
    ) -> ServiceResult<Paged<ActivityListItem>>;

    // 举办方 CRUD
    async fn create_activity(
        &self,
        user: &CurrentUser,
        body: CreateActivityBody,
    ) -> ServiceResult<Activity>;

    async fn update_activity(
        &self,
        user: &CurrentUser,
        activity_id: i64,
        body: UpdateActivityBody,
    ) -> ServiceResult<Activity>;

    async fn submit_activity(
        &self,
        user: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<Activity>;

    async fn withdraw_activity(
        &self,
        user: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<Activity>;

    async fn list_signups(
        &self,
        user: &CurrentUser,
        activity_id: i64,
        page: i64,
        page_size: i64,
        status: Option<String>,
    ) -> ServiceResult<Paged<ActivitySignup>>;

    // 签到
    async fn get_checkin_code(
        &self,
        user: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<(String, chrono::DateTime<chrono::Utc>)>;

    async fn checkin(
        &self,
        user: &CurrentUser,
        activity_id: i64,
        body: CheckinBody,
    ) -> ServiceResult<ActivitySignup>;

    // 管理员
    async fn admin_review_activity(
        &self,
        admin: &CurrentUser,
        activity_id: i64,
        body: ReviewActivityBody,
    ) -> ServiceResult<Activity>;

    async fn admin_block_activity(
        &self,
        admin: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<Activity>;
}

/// 具体实现
pub struct ActivityServiceImpl {
    pub db: PgPool,
}

impl ActivityServiceImpl {
    pub fn new(db: PgPool) -> Arc<Self> {
        Arc::new(Self { db })
    }
}

#[async_trait]
impl ActivityService for ActivityServiceImpl {
    async fn list_categories(&self) -> ServiceResult<Vec<(String, String)>> {
        // TODO: 从配置表/枚举表里查
        Ok(vec![
            ("lecture".into(), "讲座/分享".into()),
            ("club".into(), "社团活动".into()),
            ("volunteer".into(), "志愿服务".into()),
        ])
    }

    async fn list_tags(&self, keyword: Option<String>) -> ServiceResult<Vec<(String, String)>> {
    // 默认匹配全部
    let pattern = match keyword {
        Some(kw) if !kw.trim().is_empty() => format!("%{}%", kw.trim()),
        _ => "%".to_string(),
    };

    // 查询标签
    let rows = sqlx::query!(
        r#"
        SELECT key, name
        FROM activity_tags
        WHERE key ILIKE $1 OR name ILIKE $1
        ORDER BY name ASC
        "#,
        pattern
    )
    .fetch_all(&self.db)
    .await?;   // 出错会自动转成 ServiceError::Db → 50000

    // 返回 (key, name)
    let result = rows
        .into_iter()
        .map(|row| (row.key, row.name))
        .collect();

    Ok(result)
}



        async fn list_activities(
        &self,
        _current_user: Option<&CurrentUser>,
        query: ListActivitiesQuery,
    ) -> ServiceResult<Paged<ActivityListItem>> {
        // 1. 处理分页参数
        let page = query.page.unwrap_or(1).max(1);
        let mut page_size = query.page_size.unwrap_or(10);
        if page_size > 50 {
            page_size = 50;
        } else if page_size <= 0 {
            page_size = 10;
        }
        let offset = (page - 1) * page_size;

        // 2. 先查总数（去掉 deleted 的活动）
        let total: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM activities
            WHERE deleted = FALSE
            "#,
        )
        .fetch_one(&self.db)
        .await?;

        // 3. 查列表（精简版字段映射到 ActivityListItem）
        let list: Vec<ActivityListItem> = sqlx::query_as::<_, ActivityListItem>(
            r#"
            SELECT
                id,
                title,
                cover_url,
                summary,
                category,
                tags,
                location,
                start_time,
                end_time,
                signup_end_time,
                capacity,
                signup_count,
                status,
                visibility,
                organizer_name,
                is_official
            FROM activities
            WHERE deleted = FALSE
            ORDER BY start_time DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(Paged { total, list })
    }


        async fn get_activity_detail(
        &self,
        _current_user: Option<&CurrentUser>,
        activity_id: i64,
    ) -> ServiceResult<Activity> {
        // 直接从 activities 表查一行，映射成 Activity
        let activity = sqlx::query_as::<_, Activity>(
            r#"
            SELECT
                id,
                title,
                cover_url,
                summary,
                description,
                category,
                tags,
                location,
                longitude,
                latitude,
                start_time,
                end_time,
                signup_start_time,
                signup_end_time,
                capacity,
                signup_count,
                organizer_id,
                organizer_name,
                organizer_type,
                status,
                visibility,
                can_comment,
                is_official,
                signup_required,
                checkin_required,
                created_at,
                updated_at,
                deleted
            FROM activities
            WHERE id = $1
            "#
        )
        .bind(activity_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(ServiceError::NotFound)?; // 查不到就返回 NotFound

        Ok(activity)
    }


    async fn signup_activity(
    &self,
    user: &CurrentUser,
    activity_id: i64,
    _body: SignupActivityBody,
) -> ServiceResult<ActivitySignup> {
    let mut tx = self.db.begin().await?;

    // 1. 查询活动
    let activity = sqlx::query!(
        r#"
        SELECT id, signup_start_time, signup_end_time, capacity, signup_count,
               signup_required, deleted
        FROM activities
        WHERE id = $1
        "#,
        activity_id
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ServiceError::NotFound)?;

    if activity.deleted {
        return Err(ServiceError::NotFound);
    }

    // 2. 活动是否允许报名
    if !activity.signup_required {
        return Err(ServiceError::BadRequest("该活动不需要报名".into()));
    }

    // 3. 校验报名时间
    let now = chrono::Utc::now();
    if let Some(start) = activity.signup_start_time {
        if now < start {
            return Err(ServiceError::BadRequest("报名尚未开始".into()));
        }
    }
    if let Some(end) = activity.signup_end_time {
        if now > end {
            return Err(ServiceError::BadRequest("报名已经结束".into()));
        }
    }

    // 4. 名额检查
    if let Some(cap) = activity.capacity {
        if activity.signup_count >= cap {
            return Err(ServiceError::BadRequest("名额已满".into()));
        }
    }

    // 5. 查询用户是否已报名
    let existing = sqlx::query!(
        r#"
        SELECT id FROM activity_signups
        WHERE user_id = $1 AND activity_id = $2
        "#,
        user.id,
        activity_id
    )
    .fetch_optional(&mut *tx)
    .await?;

    if existing.is_some() {
        return Err(ServiceError::BadRequest("你已经报名过该活动".into()));
    }

    // 6. 插入报名记录
    let signup = sqlx::query_as::<_, ActivitySignup>(
        r#"
        INSERT INTO activity_signups (user_id, activity_id, status)
        VALUES ($1, $2, 'APPLIED')
        RETURNING id, user_id, activity_id, status, checkin_time, created_at, updated_at
        "#
    )
    .bind(user.id)
    .bind(activity_id)
    .fetch_one(&mut *tx)
    .await?;

    // 7. 更新活动报名人数
    sqlx::query!(
        r#"
        UPDATE activities
        SET signup_count = signup_count + 1,
            updated_at = NOW()
        WHERE id = $1
        "#,
        activity_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(signup)
}


    async fn cancel_signup(
    &self,
    user: &CurrentUser,
    activity_id: i64,
) -> ServiceResult<ActivitySignup> {
    let mut tx = self.db.begin().await?;

    // 1. 查询报名记录
    let signup = sqlx::query_as::<_, ActivitySignup>(
        r#"
        SELECT * FROM activity_signups
        WHERE user_id = $1 AND activity_id = $2
        "#,
    )
    .bind(user.id)
    .bind(activity_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(ServiceError::BadRequest("尚未报名该活动".into()))?;

    // 2. 已取消则不允许再次取消
    if signup.status == ActivitySignupStatus::Cancelled {
        return Err(ServiceError::BadRequest("你已经取消过报名".into()));
    }

    // 3. 更新报名状态
    let updated = sqlx::query_as::<_, ActivitySignup>(
        r#"
        UPDATE activity_signups
        SET status = 'CANCELLED',
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, user_id, activity_id, status, checkin_time, created_at, updated_at
        "#
    )
    .bind(signup.id)
    .fetch_one(&mut *tx)
    .await?;

    // 4. 活动报名人数 -1
    sqlx::query!(
        r#"
        UPDATE activities
        SET signup_count = signup_count - 1,
            updated_at = NOW()
        WHERE id = $1
        "#,
        activity_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(updated)
}


    async fn list_my_activities(
    &self,
    user: &CurrentUser,
    query: ListMyActivitiesQuery,
) -> ServiceResult<Paged<ActivityListItem>> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).clamp(1, 50);
    let offset = (page - 1) * page_size;

    // 1. 查总数
    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM activity_signups s
        JOIN activities a ON s.activity_id = a.id
        WHERE s.user_id = $1 AND s.status = 'APPLIED'
        "#
    )
    .bind(user.id)
    .fetch_one(&self.db)
    .await?;

    // 2. 查列表
    let list = sqlx::query_as::<_, ActivityListItem>(
        r#"
        SELECT
            a.id, a.title, a.cover_url, a.summary,
            a.category, a.tags, a.location,
            a.start_time, a.end_time, a.signup_end_time,
            a.capacity, a.signup_count, a.status,
            a.visibility, a.organizer_name, a.is_official
        FROM activity_signups s
        JOIN activities a ON s.activity_id = a.id
        WHERE s.user_id = $1 AND s.status = 'APPLIED'
        ORDER BY a.start_time DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(user.id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(&self.db)
    .await?;

    Ok(Paged { total, list })
}


    async fn create_activity(
        &self,
        user: &CurrentUser,
        body: CreateActivityBody,
    ) -> ServiceResult<Activity> {
        // 1. 处理默认值
        let now = chrono::Utc::now();

        let can_comment = body.can_comment.unwrap_or(true);
        let signup_required = body.signup_required.unwrap_or(true);
        let checkin_required = body.checkin_required.unwrap_or(false);

        let status = ActivityStatus::Draft;
        let visibility = body.visibility;

        // 先简单写死举办方名称/类型，后续你可以从用户表查
        let organizer_name = "TODO organizer name".to_string();
        let organizer_type = "club".to_string();

        // 2. 执行 INSERT ... RETURNING *，直接映射成 Activity
        let activity = sqlx::query_as::<_, Activity>(
            r#"
            INSERT INTO activities (
                title,
                cover_url,
                summary,
                description,
                category,
                tags,
                location,
                longitude,
                latitude,
                start_time,
                end_time,
                signup_start_time,
                signup_end_time,
                capacity,
                signup_count,
                organizer_id,
                organizer_name,
                organizer_type,
                status,
                visibility,
                can_comment,
                is_official,
                signup_required,
                checkin_required,
                created_at,
                updated_at,
                deleted
            )
            VALUES (
                $1,  $2,  $3,  $4,  $5,
                $6,  $7,  $8,  $9,  $10,
                $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20,
                $21, $22, $23, $24, $25,
                $26, $27
            )
            RETURNING
                id,
                title,
                cover_url,
                summary,
                description,
                category,
                tags,
                location,
                longitude,
                latitude,
                start_time,
                end_time,
                signup_start_time,
                signup_end_time,
                capacity,
                signup_count,
                organizer_id,
                organizer_name,
                organizer_type,
                status,
                visibility,
                can_comment,
                is_official,
                signup_required,
                checkin_required,
                created_at,
                updated_at,
                deleted
            "#
        )
        .bind(&body.title)
        .bind(&body.cover_url)
        .bind(&body.summary)
        .bind(&body.description)
        .bind(&body.category)
        .bind(&body.tags)                 // Vec<String> -> TEXT[]
        .bind(&body.location)
        .bind(None::<f64>)                // longitude 先空
        .bind(None::<f64>)                // latitude 先空
        .bind(body.start_time)
        .bind(body.end_time)
        .bind(body.signup_start_time)
        .bind(body.signup_end_time)
        .bind(body.capacity)
        .bind(0_i32)                      // signup_count = 0
        .bind(user.id)
        .bind(&organizer_name)
        .bind(&organizer_type)
        .bind(status)                     // Postgres enum activity_status
        .bind(visibility)                 // Postgres enum activity_visibility
        .bind(can_comment)
        .bind(false)                      // is_official 默认 false
        .bind(signup_required)
        .bind(checkin_required)
        .bind(now)
        .bind(now)
        .bind(false)                      // deleted = false
        .fetch_one(&self.db)
        .await?;                          // 出错会变成 ServiceError::Db

        Ok(activity)
    }




        async fn update_activity(
        &self,
        user: &CurrentUser,
        activity_id: i64,
        _body: UpdateActivityBody,
    ) -> ServiceResult<Activity> {
        // 1. 查活动
        let activity = sqlx::query_as::<_, Activity>(
            r#"
            SELECT *
            FROM activities
            WHERE id = $1
            "#,
        )
        .bind(activity_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(ServiceError::NotFound)?;

        // 2. 权限检查：只能编辑自己创建的活动
        if activity.organizer_id != user.id {
            return Err(ServiceError::PermissionDenied);
        }

        // 3. 状态检查：只允许草稿编辑（简单规则，后续可以放宽）
        if activity.status != ActivityStatus::Draft {
            return Err(ServiceError::BadRequest(
                "只有草稿状态的活动可以编辑".into(),
            ));
        }

        // 4. 目前为了让接口可用，先只更新时间。真正字段 PATCH 之后再补。
        let updated = sqlx::query_as::<_, Activity>(
            r#"
            UPDATE activities
            SET updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(activity_id)
        .fetch_one(&self.db)
        .await?;

        Ok(updated)
    }


        async fn submit_activity(
        &self,
        user: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<Activity> {
        // 1. 查活动
        let activity = sqlx::query_as::<_, Activity>(
            r#"
            SELECT *
            FROM activities
            WHERE id = $1
            "#,
        )
        .bind(activity_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(ServiceError::NotFound)?;

        if activity.organizer_id != user.id {
            return Err(ServiceError::PermissionDenied);
        }

        if activity.status != ActivityStatus::Draft {
            return Err(ServiceError::BadRequest(
                "只有草稿状态的活动可以提交审核".into(),
            ));
        }

        // 2. 更新状态为 PENDING_REVIEW
        let updated = sqlx::query_as::<_, Activity>(
            r#"
            UPDATE activities
            SET status = 'PENDING_REVIEW',
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(activity_id)
        .fetch_one(&self.db)
        .await?;

        Ok(updated)
    }


        async fn withdraw_activity(
        &self,
        user: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<Activity> {
        let activity = sqlx::query_as::<_, Activity>(
            r#"
            SELECT *
            FROM activities
            WHERE id = $1
            "#,
        )
        .bind(activity_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(ServiceError::NotFound)?;

        if activity.organizer_id != user.id {
            return Err(ServiceError::PermissionDenied);
        }

        if activity.status != ActivityStatus::PendingReview {
            return Err(ServiceError::BadRequest(
                "只有审核中的活动可以撤回为草稿".into(),
            ));
        }

        let updated = sqlx::query_as::<_, Activity>(
            r#"
            UPDATE activities
            SET status = 'DRAFT',
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(activity_id)
        .fetch_one(&self.db)
        .await?;

        Ok(updated)
    }


        async fn list_signups(
        &self,
        user: &CurrentUser,
        activity_id: i64,
        page: i64,
        page_size: i64,
        status: Option<String>,
    ) -> ServiceResult<Paged<ActivitySignup>> {
        // 1. 权限：必须是该活动的举办方，或者管理员
        let activity = sqlx::query!(
            r#"
            SELECT organizer_id
            FROM activities
            WHERE id = $1 AND deleted = FALSE
            "#,
            activity_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or(ServiceError::NotFound)?;

        if activity.organizer_id != user.id && !user.is_admin {
            return Err(ServiceError::PermissionDenied);
        }

        // 2. 分页参数
        let page = page.max(1);
        let mut size = page_size;
        if size <= 0 {
            size = 20;
        } else if size > 100 {
            size = 100;
        }
        let offset = (page - 1) * size;

        // 3. 处理 status 过滤
        let status_enum: Option<ActivitySignupStatus> = match status {
            Some(s) => match s.as_str() {
                "APPLIED" => Some(ActivitySignupStatus::Applied),
                "CANCELLED" => Some(ActivitySignupStatus::Cancelled),
                "CHECKED_IN" => Some(ActivitySignupStatus::CheckedIn),
                _ => {
                    return Err(ServiceError::BadRequest("无效的报名状态筛选".into()));
                }
            },
            None => None,
        };

        // 4. 查询总数 & 列表
        let (total, list) = if let Some(st) = status_enum {
            let total: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM activity_signups
                WHERE activity_id = $1 AND status = $2
                "#,
            )
            .bind(activity_id)
            .bind(st)
            .fetch_one(&self.db)
            .await?;

            let list: Vec<ActivitySignup> = sqlx::query_as::<_, ActivitySignup>(
                r#"
                SELECT *
                FROM activity_signups
                WHERE activity_id = $1 AND status = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#,
            )
            .bind(activity_id)
            .bind(st)
            .bind(size)
            .bind(offset)
            .fetch_all(&self.db)
            .await?;

            (total, list)
        } else {
            let total: i64 = sqlx::query_scalar(
                r#"
                SELECT COUNT(*)
                FROM activity_signups
                WHERE activity_id = $1
                "#,
            )
            .bind(activity_id)
            .fetch_one(&self.db)
            .await?;

            let list: Vec<ActivitySignup> = sqlx::query_as::<_, ActivitySignup>(
                r#"
                SELECT *
                FROM activity_signups
                WHERE activity_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(activity_id)
            .bind(size)
            .bind(offset)
            .fetch_all(&self.db)
            .await?;

            (total, list)
        };

        Ok(Paged { total, list })
    }


    async fn get_checkin_code(
        &self,
        _user: &CurrentUser,
        _activity_id: i64,
    ) -> ServiceResult<(String, chrono::DateTime<chrono::Utc>)> {
        // TODO: 生成/查询签到 token
        Ok(("dummy_token".into(), chrono::Utc::now()))
    }

    async fn checkin(
        &self,
        _user: &CurrentUser,
        _activity_id: i64,
        _body: CheckinBody,
    ) -> ServiceResult<ActivitySignup> {
        // TODO: 校验 token + 更新签到状态
        Err(ServiceError::Internal)
    }

        async fn admin_review_activity(
        &self,
        admin: &CurrentUser,
        activity_id: i64,
        body: ReviewActivityBody,
    ) -> ServiceResult<Activity> {
        // 1. 必须是管理员
        if !admin.is_admin {
            return Err(ServiceError::PermissionDenied);
        }

        // 2. 查活动
        let activity = sqlx::query_as::<_, Activity>(
            r#"
            SELECT *
            FROM activities
            WHERE id = $1
            "#,
        )
        .bind(activity_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(ServiceError::NotFound)?;

        // 3. 只能审核 PendingReview 状态
        match activity.status {
            ActivityStatus::PendingReview => {}
            _ => {
                return Err(ServiceError::BadRequest(
                    "只有待审核状态的活动可以进行审核".into(),
                ));
            }
        }

        // 4. 根据 action 决定新状态
        let action = body.action.to_uppercase();
        let new_status = if action == "APPROVE" {
            ActivityStatus::Published
        } else if action == "REJECT" {
            ActivityStatus::Draft
        } else {
            return Err(ServiceError::BadRequest("无效的审核动作".into()));
        };

        // 5. 更新状态
        let updated = sqlx::query_as::<_, Activity>(
            r#"
            UPDATE activities
            SET status = $2,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(activity_id)
        .bind(new_status)
        .fetch_one(&self.db)
        .await?;

        Ok(updated)
    }


        async fn admin_block_activity(
        &self,
        admin: &CurrentUser,
        activity_id: i64,
    ) -> ServiceResult<Activity> {
        // 1. 只有管理员可以强制下线
        if !admin.is_admin {
            return Err(ServiceError::PermissionDenied);
        }

        // 2. 更新状态为 CANCELLED，并标记 deleted = TRUE
        let updated = sqlx::query_as::<_, Activity>(
            r#"
            UPDATE activities
            SET status = 'CANCELLED',
                deleted = TRUE,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(activity_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(ServiceError::NotFound)?;

        Ok(updated)
    }

}
