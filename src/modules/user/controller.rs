use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::common::db::DbPool;
use crate::common::error::AppError;
use crate::common::jwt;
use super::entity::{UpdateUserProfile};
use super::service::UserService;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub student_id: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub student_id: String,
    pub password: String,
    pub name: String,
    pub college: String,
    pub major: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// 辅助函数：构建成功响应
fn success_response<T: Serialize>(data: T) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "code": 200, "message": "success", "data": data })),
    )
}

// 辅助函数：构建空数据成功响应
fn empty_success_response() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "code": 200, "message": "success", "data": null })),
    )
}

// 获取当前用户ID（从JWT token，这里简化处理）
// 实际项目中应该从Authorization头中解析JWT token
fn get_current_user_id(headers: &HeaderMap) -> Result<String, AppError> {
    // 从Authorization头获取token
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::AuthError("缺少Authorization头".to_string()))?;

    // 检查Bearer前缀
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError("Authorization头格式错误, 应为 'Bearer <token>'".to_string()));
    }

    let token = &auth_header[7..]; // 跳过"Bearer "前缀

    // 先检查token是否过期
    if jwt::is_token_expired(token) {
        return Err(AppError::AuthError("Token已过期, 请重新登录".to_string()));
    }

    // 验证token并提取claims
    let claims = jwt::validate_token(token)
        .map_err(|e| AppError::AuthError(format!("Token验证失败: {}", e)))?;

    Ok(claims.sub)
}

// POST /users/login
pub async fn login_handler(
    State(pool): State<Arc<DbPool>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::login(&pool, &req.student_id, &req.password).await?;
    
    let token = jwt::generate_token(&user.id.to_string())
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    
    let login_data = json!({ // 对应user.proto中的LoginData
        "token": token,
        "user": {
            "id": user.id,
            "student_id": user.student_id,
            "name": user.username,
            "avatar_url": user.avatar_url,
            "role": user.role,
            "college": user.college,
            "major": user.major,
            "grade": user.grade,
            "class_name": user.class_name,
            "bio": user.bio,
            "phone": user.phone,
            "email": user.email,
            "wechat_id": user.wechat_id,
            "weekly_course_count": user.weekly_course_count,
            "forum_activity_score": user.forum_activity_score,
            "collection_count": user.collection_count,
            "setting_privacy_course": user.setting_privacy_course,
            "setting_notification_switch": user.setting_notification_switch,
        }
    });
    
    Ok(success_response(login_data))
}

// POST /users/register
pub async fn register_handler(
    State(pool): State<Arc<DbPool>>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::register(
        &pool,
        req.student_id,
        req.password,
        req.name,
        req.college,
        req.major,
        req.phone,
    ).await?;
    
    // 根据OpenAPI规范，只返回user_id
    let register_data = json!({ "user_id": user.id });
    
    Ok(success_response(register_data))
}

// GET /users/me
// 注意：需要认证，获取当前登录用户的个人信息
pub async fn get_user_info_handler(
    State(pool): State<Arc<DbPool>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // 从JWT token获取当前用户ID（已包含token过期验证）
    let current_user_id = get_current_user_id(&headers)?;

    let user = UserService::get_user_info(&pool, &current_user_id).await?;
    
    // 转换为proto格式的响应
    let user_info = json!({ // 对应user.proto中的UserInfo
        "id": user.id,
        "student_id": user.student_id,
        "name": user.username,
        "avatar_url": user.avatar_url,
        "role": user.role,
        "college": user.college,
        "major": user.major,
        "grade": user.grade,
        "class_name": user.class_name,
        "bio": user.bio,
        "phone": user.phone,
        "email": user.email,
        "wechat_id": user.wechat_id,
        "weekly_course_count": user.weekly_course_count,
        "forum_activity_score": user.forum_activity_score,
        "collection_count": user.collection_count,
        "setting_privacy_course": user.setting_privacy_course,
        "setting_notification_switch": user.setting_notification_switch,
    });
    
    Ok(success_response(user_info))
}

// PUT /users/me
// 注意：需要认证，只能更新自己的资料
pub async fn update_profile_handler(
    State(pool): State<Arc<DbPool>>,
    headers: HeaderMap,
    Json(update_data): Json<UpdateUserProfile>,
) -> Result<impl IntoResponse, AppError> {
    // 从JWT token获取当前用户ID
    let current_user_id = get_current_user_id(&headers)?;
    
    // 更新用户资料
    UserService::update_profile(&pool, &current_user_id, update_data).await?;
    
    // 根据OpenAPI规范，返回data为null的成功响应
    Ok(empty_success_response())
}

// POST /auth/logout
// 注意：需要认证
pub async fn logout_handler(
    State(pool): State<Arc<DbPool>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // 验证当前用户
    let current_user_id = get_current_user_id(&headers)?;
    
    UserService::logout(&pool, &current_user_id).await?;
    
    Ok(empty_success_response())
}

// PUT /auth/password
// 注意：需要认证，且只能修改自己的密码
pub async fn change_password_handler(
    State(pool): State<Arc<DbPool>>,
    headers: HeaderMap,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 验证当前用户是否有权限修改密码
    let current_user_id = get_current_user_id(&headers)?;
    
    UserService::change_password(&pool, &current_user_id, &req.old_password, &req.new_password).await?;
    
    Ok(empty_success_response())
}

// 导出路由
pub fn router(pool: Arc<DbPool>) -> axum::Router {
    axum::Router::new()
        .route("/auth/login", axum::routing::post(login_handler))
        .route("/auth/register", axum::routing::post(register_handler))
        .route("/users/me", axum::routing::get(get_user_info_handler))
        .route("/users/me", axum::routing::put(update_profile_handler))
        .route("/auth/logout", axum::routing::post(logout_handler))
        .route("/auth/password", axum::routing::put(change_password_handler))
        .with_state(pool)
}