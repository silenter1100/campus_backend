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
    pub gender: String,
    /// 学院
    pub college: String,
    /// 专业
    pub major: String,
    /// 班级
    pub class_name: String,
    /// 电话号
    pub phone: String,
    /// 邮箱
    pub email: String,
    /// 头像链接
    pub avatar_url: String,
    /// 角色
    pub role: String,
    /// 微信号
    pub wechat_id: String,
    /// 收藏数
    pub collection_count: i64,
    /// 论坛活跃度
    pub forum_activity_score: i64,
    /// 本周课时数
    pub weekly_course_count: i64,
    /// 年级
    pub grade: String,
    /// 个人简介
    pub bio: String,
    /// 通知开关
    pub setting_notification_switch: bool,
    /// 课表隐私
    pub setting_privacy_course: String,
    /// 主题设置
    pub setting_theme: String,
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
            gender,
            college,
            major,
            class_name,
            phone,
            email,
            avatar_url: String::new(),
            role: String::new(),
            wechat_id: String::new(),
            collection_count: 0,
            forum_activity_score: 0,
            weekly_course_count: 0,
            grade: String::new(),
            bio: String::new(),
            setting_notification_switch: true,
            setting_privacy_course: String::new(),
            setting_theme: String::new(),
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
