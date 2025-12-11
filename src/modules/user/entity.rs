use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User 实体（对应数据库 users 表）
/// 适配：主分支的 JSON 风格 camelCase、SQLX FromRow、Clone
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// 用户ID（UUID）
    pub id: String,
    /// 学号
    pub student_id: String,
    /// 用户名
    pub username: String,
    /// 密码哈希
    pub password: String,
    /// 性别
    pub gender: Option<String>,
    /// 学院
    pub college: String,
    /// 专业
    pub major: String,
    /// 班级
    pub class_name: Option<String>,
    /// 电话号
    pub phone: String,
    /// 邮箱
    pub email: Option<String>,
    /// 头像链接
    pub avatar_url: Option<String>,
    /// 角色
    pub role: Option<String>,
    /// 微信号
    pub wechat_id: Option<String>,
    /// 收藏数
    pub collection_count: Option<i64>,
    /// 论坛活跃度
    pub forum_activity_score: Option<i64>,
    /// 本周课时数
    pub weekly_course_count: Option<i64>,
    /// 年级
    pub grade: Option<String>,
    /// 个人简介
    pub bio: Option<String>,
    /// 通知开关
    pub setting_notification_switch: Option<bool>,
    /// 课表隐私
    pub setting_privacy_course: Option<String>,
    /// 主题设置
    pub setting_theme: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 最后登录时间
    pub last_login_at: Option<DateTime<Utc>>,
}

impl User {
    /// 创建一个新用户（用于注册）
    pub fn new(
        student_id: String,
        username: String,
        password_hash: String,
        gender: String,
        college: String,
        major: String,
        class_name: String,
        phone: String,
        email: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            student_id,
            username,
            password: password_hash,
            gender: Some(gender),
            college,
            major,
            class_name: Some(class_name),
            phone,
            email: Some(email),
            avatar_url: Some(String::new()),
            role: Some("student".to_string()),
            wechat_id: Some(String::new()),
            collection_count: Some(0),
            forum_activity_score: Some(0),
            weekly_course_count: Some(0),
            grade: Some(String::new()),
            bio: Some(String::new()),
            setting_notification_switch: Some(true),
            setting_privacy_course: Some("public".to_string()),
            setting_theme: Some("light".to_string()),
            created_at: now,
            updated_at: now,
            last_login_at: None,
        }
    }

    /// 更新最后登录时间
    pub fn update_last_login(&mut self) {
        self.last_login_at = Some(Utc::now());
    }

    /// （弱）密码比较（注：实际项目中不可用）
    pub fn check_password(&self, password: &str) -> bool {
        self.password == password
    }
}

/// 用户资料更新结构体（部分更新）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserProfile {
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub wechat_id: Option<String>,
    pub setting_theme: Option<String>,
    pub setting_privacy_course: Option<String>,
    pub setting_notification_switch: Option<bool>,
}
