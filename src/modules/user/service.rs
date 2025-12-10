use crate::common::db::DbPool;
use crate::common::error::AppError;
use super::entity::{User, UpdateUserProfile};
use chrono::Utc;

pub struct UserService;

impl UserService {
    /// 用户登录
    pub async fn login(
        pool: &DbPool,
        student_id: &str,
        password: &str,
    ) -> Result<User, AppError> {
        // 查找用户
        let mut user = sqlx::query_as::<_, User>(
            r#"SELECT id, student_id, username, password, gender, college, 
                major, class_name, phone, email, avatar_url, 
                created_at, updated_at, last_login_at, setting_privacy_course, 
                setting_notification_switch, setting_theme, role, wechat_id, bio, 
                collection_count, forum_activity_score, grade, weekly_course_count 
                FROM users WHERE student_id = ?"#
        )
        .bind(student_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::AuthError("用户不存在".to_string()))?;
        
        // 验证密码（简化，实际应该哈希比较）
        if !user.check_password(password) {
            return Err(AppError::AuthError("密码错误".to_string()));
        }
        
        // 更新最后登录时间
        user.update_last_login();
        
        // 更新数据库中的最后登录时间
        sqlx::query(
            r#"UPDATE users SET last_login_at = ? WHERE id = ?"#
        )
        .bind(user.last_login_at.unwrap_or(Utc::now()))
        .bind(&user.id)
        .execute(pool)
        .await?;
        
        Ok(user)
    }
    
    /// 用户注册
    pub async fn register(
        pool: &DbPool,
        student_id: String,
        password: String,
        name: String,
        college: String,
        major: String,
        phone: String,
    ) -> Result<User, AppError> {
        // 检查学号是否已存在
        let existing = sqlx::query_as::<_, User>(
            r#"SELECT * FROM users WHERE student_id = ?"#
        )
        .bind(&student_id)
        .fetch_optional(pool)
        .await?;
        
        if existing.is_some() {
            return Err(AppError::BadRequest("学号已存在".to_string()));
        }
        
        // 创建新用户（密码应该哈希，这里简单处理）
        let password_hash = password; // 实际应该哈希密码
        let new_user = User::new(
            student_id,
            name, // 对应username
            password_hash,
            "".to_string(), // gender 默认为空
            college,
            major,
            "".to_string(), // class_name 默认为空
            phone,
            "".to_string(), // email 默认为空
        );
        
        // 插入数据库
        sqlx::query(
            r#"INSERT INTO users (
                id, student_id, username, password, gender, college, 
                major, class_name, phone, email, avatar_url, 
                created_at, updated_at, last_login_at, setting_privacy_course, 
                setting_notification_switch, setting_theme, role, wechat_id, bio, 
                collection_count, forum_activity_score, grade, weekly_course_count
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&new_user.id)
        .bind(&new_user.student_id)
        .bind(&new_user.username)
        .bind(&new_user.password)
        .bind(&new_user.gender)
        .bind(&new_user.college)
        .bind(&new_user.major)
        .bind(&new_user.class_name)
        .bind(&new_user.phone)
        .bind(&new_user.email)
        .bind(&new_user.avatar_url)
        .bind(new_user.created_at)
        .bind(new_user.updated_at)
        .bind(new_user.last_login_at)
        .bind(&new_user.setting_privacy_course)
        .bind(&new_user.setting_notification_switch)
        .bind(&new_user.setting_theme)  // 添加setting_theme
        .bind(&new_user.role)
        .bind(&new_user.wechat_id)
        .bind(&new_user.bio)
        .bind(&new_user.collection_count)
        .bind(&new_user.forum_activity_score)
        .bind(&new_user.grade)
        .bind(&new_user.weekly_course_count)
        .execute(pool)
        .await?;
        
        Ok(new_user)
    }
    
    /// 获取用户信息
    pub async fn get_user_info(
        pool: &DbPool,
        user_id: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT 
                id, student_id, username, password, gender, college, major, class_name, 
                phone, email, avatar_url,  role, wechat_id, collection_count, 
                forum_activity_score, weekly_course_count, grade, bio, setting_notification_switch, 
                setting_privacy_course, setting_theme, created_at, updated_at, last_login_at 
                FROM users WHERE id = ?"#
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;
        
        Ok(user)
    }
    
    /// 更新用户资料
    pub async fn update_profile(
        pool: &DbPool,
        user_id: &str,
        update_data: UpdateUserProfile,
    ) -> Result<User, AppError> {
        // 首先获取当前用户
        let mut user = Self::get_user_info(pool, user_id).await?;
        
        // 更新字段（如果提供）
        if let Some(name) = update_data.name {
            user.username = name;
        }
        if let Some(avatar_url) = update_data.avatar_url {
            user.avatar_url = avatar_url;
        }
        if let Some(bio) = update_data.bio {
            user.bio = bio;
        }
        if let Some(phone) = update_data.phone {
            user.phone = phone;
        }
        if let Some(email) = update_data.email {
            user.email = email;
        }
        if let Some(wechat_id) = update_data.wechat_id {
            user.wechat_id = wechat_id;
        }
        if let Some(setting_theme) = update_data.setting_theme {
            user.setting_theme = setting_theme;
        }
        if let Some(setting_privacy_course) = update_data.setting_privacy_course {
            user.setting_privacy_course = setting_privacy_course;
        }
        if let Some(setting_notification_switch) = update_data.setting_notification_switch {
            user.setting_notification_switch = setting_notification_switch;
        }
        
        // 更新更新时间
        user.updated_at = Utc::now();
        
        // 保存到数据库
        sqlx::query(
            r#"UPDATE users SET 
                username = ?, avatar_url = ?, bio = ?, 
                phone = ?, email = ?, wechat_id = ?, setting_theme = ?, 
                setting_privacy_course = ?, setting_notification_switch = ?, updated_at = ?
                WHERE id = ?"#
        )
        .bind(&user.username)
        .bind(&user.avatar_url)
        .bind(&user.bio)
        .bind(&user.phone)
        .bind(&user.email)
        .bind(&user.wechat_id)
        .bind(&user.setting_theme)
        .bind(&user.setting_privacy_course)
        .bind(&user.setting_notification_switch)
        .bind(user.updated_at)
        .bind(&user.id)
        .execute(pool)
        .await?;
        
        Ok(user)
    }
    
    /// 修改密码
    pub async fn change_password(
        pool: &DbPool,
        user_id: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        // 获取用户
        let user = Self::get_user_info(pool, user_id).await?;
        
        // 验证旧密码
        if !user.check_password(old_password) {
            return Err(AppError::AuthError("旧密码错误".to_string()));
        }
        
        // 更新密码（实际应该哈希新密码）
        let new_password_hash = new_password.to_string(); // 实际应该哈希
        
        sqlx::query(
            r#"UPDATE users SET password = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(new_password_hash)
        .bind(Utc::now())
        .bind(user_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
    
    /// 退出登录（这里主要是客户端处理，服务器端可以记录日志或清除token）
    /// 由于是JWT-based，服务器端通常不需要操作，但可以记录日志
    pub async fn logout(_pool: &DbPool, _user_id: &str) -> Result<(), AppError> {
        // 在实际应用中，可以记录日志或使token失效（如果使用黑名单）
        // 这里简单返回成功
        Ok(())
    }
}









