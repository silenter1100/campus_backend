use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::IntoResponse,
    Json, Extension,
};
use serde_json::json;
use crate::common::{db::DBPool, error::AppError};
use super::{entity::*, service::ForumService};

// -------------------------------------------------------------------------
// Helper: Standard Response
// -------------------------------------------------------------------------
fn success_response<T: serde::Serialize>(data: T) -> impl IntoResponse {
    Json(json!({
        "code": 200,
        "message": "success",
        "data": data
    }))
}

// =========================================================================
//  Boards
// =========================================================================
pub async fn list_boards(State(pool): State<DBPool>) -> Result<impl IntoResponse, AppError> {
    let list = ForumService::get_board_list(&pool).await?;
    Ok(success_response(json!({ "list": list })))
}

// =========================================================================
//  Posts
// =========================================================================
pub async fn create_post(
    State(pool): State<DBPool>,
    headers: HeaderMap,
    Extension(user_id): Extension<String>, 
    Json(req): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !headers.contains_key("Idempotency-Key") {
        return Err(AppError::BadRequest("Missing Idempotency-Key header".into()));
    }

    let post_id = ForumService::create_post(&pool, &user_id, req).await?;
    let post = ForumService::get_post_detail(&pool, &post_id, Some(&user_id)).await?;

    Ok(success_response(post))
}

pub async fn list_posts(
    State(pool): State<DBPool>,
    // 列表页可能未登录，使用 Option<Extension> 来处理可选的 Token
    user_ext: Option<Extension<String>>, 
    Query(query): Query<PostQuery>,
) -> Result<impl IntoResponse, AppError> {
    // 提取 user_id (如果有)
    let user_id = user_ext.map(|x| x.0);
    
    let result = ForumService::get_post_list(&pool, user_id.as_deref(), query).await?;
    Ok(success_response(result))
}

pub async fn get_post(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    user_ext: Option<Extension<String>>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_ext.map(|x| x.0);
    
    let result = ForumService::get_post_detail(&pool, &id, user_id.as_deref()).await?;
    Ok(success_response(result))
}

pub async fn delete_post(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::delete_post(&pool, &id, &user_id).await?;
    Ok(success_response(serde_json::Value::Null))
}

pub async fn update_post(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::update_post(&pool, &id, req).await?;
    let post = ForumService::get_post_detail(&pool, &id, Some(&user_id)).await?;
    Ok(success_response(post))
}

// =========================================================================
//  Interactions (Post)
// =========================================================================
pub async fn like_post(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<LikeActionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (count, is_liked) = ForumService::toggle_like_post(&pool, &id, &user_id, &req.actions).await?;
    Ok(success_response(json!({
        "current_like_count": count,
        "is_liked": is_liked
    })))
}

pub async fn collect_post(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<CollectActionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (is_collected, _total_count) = ForumService::toggle_collect_post(&pool, &id, &user_id, &req.action).await?;
    
    Ok(success_response(json!({
        "is_collected": is_collected
    })))
}

// =========================================================================
//  Comments
// =========================================================================
pub async fn create_comment(
    State(pool): State<DBPool>,
    Path(post_id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, AppError> {
    let comment = ForumService::create_comment(&pool, &post_id, &user_id, req).await?;
    Ok(success_response(json!({
        "comment_id": comment.id,
        "comment": comment
    })))
}

pub async fn list_comments(
    State(pool): State<DBPool>,
    Path(post_id): Path<String>,
    user_ext: Option<Extension<String>>,
    Query(query): Query<CommentQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_ext.map(|x| x.0);
    
    let result = ForumService::get_comments(&pool, &post_id, user_id.as_deref(), query).await?;
    Ok(success_response(result))
}

pub async fn delete_comment(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::delete_comment(&pool, &id, &user_id).await?;
    Ok(success_response(serde_json::Value::Null))
}

pub async fn like_comment(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<LikeActionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (count, is_liked) = ForumService::toggle_like_comment(&pool, &id, &user_id, &req.actions).await?;
    Ok(success_response(json!({
        "current_like_count": count,
        "is_liked": is_liked
    })))
}

// =========================================================================
//  Reports & Admin
// =========================================================================
pub async fn create_report(
    State(pool): State<DBPool>,
    Extension(user_id): Extension<String>,
    Json(req): Json<CreateReportRequest>,
) -> Result<impl IntoResponse, AppError> {
    let report_id = ForumService::create_report(&pool, &user_id, req).await?;
    Ok(success_response(json!({ "report_id": report_id })))
}

pub async fn admin_list_reports(
    State(pool): State<DBPool>,
    // 假设管理员也需要登录，这里强制提取 UserID
    // 实际项目中可能还需要检查 Role
    Extension(_user_id): Extension<String>,
    Query(query): Query<AdminReportQuery>,
) -> Result<impl IntoResponse, AppError> {
    let result = ForumService::admin_list_reports(&pool, query).await?;
    Ok(success_response(result))
}

pub async fn admin_audit_post(
    State(pool): State<DBPool>,
    Path(id): Path<String>,
    Extension(_user_id): Extension<String>,
    Json(req): Json<AdminPostStatusRequest>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::admin_audit_post(&pool, &id, req).await?;
    Ok(success_response(serde_json::Value::Null))
}