use axum::{
    body::Bytes,
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use prost::Message;
use crate::common::{auth::AuthUser, AppError, state::AppState};
use super::service::UserService;

// 引入生成的 Protobuf 代码
mod proto {
    include!(concat!(env!("OUT_DIR"), "/campus.user.rs"));
}

// ==================== 路由注册 ====================

pub fn routes() -> Router<AppState> {
    Router::new()
        // 公开路由（不需要认证）
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/register", post(register_handler))
        // 需要认证的路由
        .route("/api/v1/users/me", get(get_user_info_handler))
        .route("/api/v1/users/me", put(update_profile_handler))
        .route("/api/v1/auth/logout", post(logout_handler))
        .route("/api/v1/auth/password", put(change_password_handler))
}

// ==================== 处理函数 ====================

/// 用户登录
async fn login_handler(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::LoginRequest::decode(body)?;

    let user = UserService::login(&state.pool, &proto_req.student_id, &proto_req.password).await?;

    let token = crate::common::auth::generate_token_for_user(&user.id)?;

    let proto_user = proto::User {
        id: user.id.clone(),
        student_id: user.student_id.clone(),
        name: user.username.clone(),
        avatar_url: user.avatar_url.unwrap_or_default(),
        role: user.role.unwrap_or_default(),
        college: user.college.clone(),
        major: user.major.clone(),
        grade: user.grade.unwrap_or_default(),
        class_name: user.class_name.unwrap_or_default(),
        bio: user.bio.unwrap_or_default(),
        phone: user.phone.clone(),
        email: user.email.unwrap_or_default(),
        wechat_id: user.wechat_id.unwrap_or_default(),
        weekly_course_count: user.weekly_course_count.unwrap_or_default(),
        forum_activity_score: user.forum_activity_score.unwrap_or_default(),
        collection_count: user.collection_count.unwrap_or_default(),
        setting_privacy_course: user.setting_privacy_course.unwrap_or_default(),
        setting_notification_switch: user.setting_notification_switch.unwrap_or_default(),
    };

    let response = proto::LoginResponse {
        code: 200,
        message: "登录成功".to_string(),
        data: Some(proto::LoginData {
            token,
            user: Some(proto_user),
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 用户注册
async fn register_handler(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::RegisterRequest::decode(body)?;

    let user = UserService::register(
        &state.pool,
        proto_req.student_id,
        proto_req.password,
        proto_req.name,
        proto_req.college,
        proto_req.major,
        proto_req.phone,
    ).await?;

    let response = proto::RegisterResponse {
        code: 200,
        message: "注册成功".to_string(),
        data: Some(proto::RegisterData {
            user_id: user.id,
        }),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 获取用户信息
async fn get_user_info_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::get_user_info(&state.pool, &auth_user.user_id).await?;

    let proto_user = proto::User {
        id: user.id,
        student_id: user.student_id,
        name: user.username,
        avatar_url: user.avatar_url.unwrap_or_default(),
        role: user.role.unwrap_or_default(),
        college: user.college,
        major: user.major,
        grade: user.grade.unwrap_or_default(),
        class_name: user.class_name.unwrap_or_default(),
        bio: user.bio.unwrap_or_default(),
        phone: user.phone,
        email: user.email.unwrap_or_default(),
        wechat_id: user.wechat_id.unwrap_or_default(),
        weekly_course_count: user.weekly_course_count.unwrap_or_default(),
        forum_activity_score: user.forum_activity_score.unwrap_or_default(),
        collection_count: user.collection_count.unwrap_or_default(),
        setting_privacy_course: user.setting_privacy_course.unwrap_or_default(),
        setting_notification_switch: user.setting_notification_switch.unwrap_or_default(),
    };

    let response = proto::GetUserInfoResponse {
        code: 200,
        message: "获取成功".to_string(),
        data: Some(proto_user),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 更新用户资料
async fn update_profile_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::UpdateProfileRequest::decode(body)?;

    // 转换为服务层的更新结构
    let update_data = super::entity::UpdateUserProfile {
        name: if proto_req.name.is_empty() { None } else { Some(proto_req.name) },
        avatar_url: if proto_req.avatar_url.is_empty() { None } else { Some(proto_req.avatar_url) },
        bio: if proto_req.bio.is_empty() { None } else { Some(proto_req.bio) },
        phone: if proto_req.phone.is_empty() { None } else { Some(proto_req.phone) },
        email: if proto_req.email.is_empty() { None } else { Some(proto_req.email) },
        wechat_id: if proto_req.wechat_id.is_empty() { None } else { Some(proto_req.wechat_id) },
        setting_theme: None, // proto中没有这个字段
        setting_privacy_course: if proto_req.setting_privacy_course.is_empty() { None } else { Some(proto_req.setting_privacy_course) },
        setting_notification_switch: Some(proto_req.setting_notification_switch),
    };

    UserService::update_profile(&state.pool, &auth_user.user_id, update_data).await?;

    let response = proto::UpdateProfileResponse {
        code: 200,
        message: "更新成功".to_string(),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 用户退出
async fn logout_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    UserService::logout(&state.pool, &auth_user.user_id).await?;

    let response = proto::LogoutResponse {
        code: 200,
        message: "退出成功".to_string(),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}

/// 修改密码
async fn change_password_handler(
    State(state): State<AppState>,
    auth_user: AuthUser,
    body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let proto_req = proto::ChangePasswordRequest::decode(body)?;

    UserService::change_password(&state.pool, &auth_user.user_id, &proto_req.old_password, &proto_req.new_password).await?;

    let response = proto::ChangePasswordResponse {
        code: 200,
        message: "密码修改成功".to_string(),
    };

    let bytes = response.encode_to_vec();
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        bytes,
    ))
}
