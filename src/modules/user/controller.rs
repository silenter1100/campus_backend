use axum::{
    extract::{Json, State},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::common::error::AppError;
use crate::common::auth::get_user_id_from_token;
use crate::common::state::AppState;

use super::entity::UpdateUserProfile;
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

fn success_response<T: Serialize>(data: T) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "code": 200, "message": "success", "data": data })),
    )
}

fn empty_success_response() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "code": 200, "message": "success", "data": null })),
    )
}

/// 从 JWT 中解析 user_id
fn get_current_user_id(headers: &HeaderMap, _state: &AppState) -> Result<String, AppError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::AuthError("缺少Authorization头".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::AuthError("Authorization头格式错误".into()));
    }

    let token = &auth_header[7..];
    
    // 使用新的 auth 模块验证 token
    get_user_id_from_token(token)
}

/// 路由
pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api/v1/auth/login", axum::routing::post(login_handler))
        .route("/api/v1/auth/register", axum::routing::post(register_handler))
        .route("/api/v1/users/me", axum::routing::get(get_user_info_handler))
        .route("/api/v1/users/me", axum::routing::put(update_profile_handler))
        .route("/api/v1/auth/logout", axum::routing::post(logout_handler))
        .route("/api/v1/auth/password", axum::routing::put(change_password_handler))
}

/// 登录
pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let pool = &state.pool;

    let user = UserService::login(pool, &req.student_id, &req.password).await?;

    let token = crate::common::auth::generate_token_for_user(&user.id)?;

    let login_data = json!({
        "token": token,
        "user": {
            "id": user.id,
            "student_id": user.student_id,
            "name": user.username,
            "avatar_url": user.avatar_url.unwrap_or_default(),
            "role": user.role.unwrap_or_default(),
            "college": user.college,
            "major": user.major,
            "grade": user.grade.unwrap_or_default(),
            "class_name": user.class_name.unwrap_or_default(),
            "bio": user.bio.unwrap_or_default(),
            "phone": user.phone,
            "email": user.email.unwrap_or_default(),
            "wechat_id": user.wechat_id.unwrap_or_default(),
            "weekly_course_count": user.weekly_course_count.unwrap_or_default(),
            "forum_activity_score": user.forum_activity_score.unwrap_or_default(),
            "collection_count": user.collection_count.unwrap_or_default(),
            "setting_privacy_course": user.setting_privacy_course.unwrap_or_default(),
            "setting_notification_switch": user.setting_notification_switch.unwrap_or_default(),
        }
    });

    Ok(success_response(login_data))
}

/// 注册
pub async fn register_handler(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::register(
        &state.pool,
        req.student_id,
        req.password,
        req.name,
        req.college,
        req.major,
        req.phone,
    )
        .await?;

    Ok(success_response(json!({ "user_id": user.id })))
}

/// 获取个人信息
pub async fn get_user_info_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let uid = get_current_user_id(&headers, &state)?;

    let user = UserService::get_user_info(&state.pool, &uid).await?;

    let resp = json!({
        "id": user.id,
        "student_id": user.student_id,
        "name": user.username,
        "avatar_url": user.avatar_url.unwrap_or_default(),
        "role": user.role.unwrap_or_default(),
        "college": user.college,
        "major": user.major,
        "grade": user.grade.unwrap_or_default(),
        "class_name": user.class_name.unwrap_or_default(),
        "bio": user.bio.unwrap_or_default(),
        "phone": user.phone,
        "email": user.email.unwrap_or_default(),
        "wechat_id": user.wechat_id.unwrap_or_default(),
        "weekly_course_count": user.weekly_course_count.unwrap_or_default(),
        "forum_activity_score": user.forum_activity_score.unwrap_or_default(),
        "collection_count": user.collection_count.unwrap_or_default(),
        "setting_privacy_course": user.setting_privacy_course.unwrap_or_default(),
        "setting_notification_switch": user.setting_notification_switch.unwrap_or_default(),
    });

    Ok(success_response(resp))
}

/// 更新个人资料
pub async fn update_profile_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(update_data): Json<UpdateUserProfile>,
) -> Result<impl IntoResponse, AppError> {
    let uid = get_current_user_id(&headers, &state)?;

    UserService::update_profile(&state.pool, &uid, update_data).await?;

    Ok(empty_success_response())
}

/// 退出
pub async fn logout_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let uid = get_current_user_id(&headers, &state)?;

    UserService::logout(&state.pool, &uid).await?;

    Ok(empty_success_response())
}

/// 修改密码
pub async fn change_password_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    let uid = get_current_user_id(&headers, &state)?;

    UserService::change_password(&state.pool, &uid, &req.old_password, &req.new_password)
        .await?;

    Ok(empty_success_response())
}
