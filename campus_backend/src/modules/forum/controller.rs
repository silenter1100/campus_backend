use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::IntoResponse,
    Json, Extension, Router,
    routing::{get, post, put, delete},
};
use serde_json::json;

use crate::common::{state::AppState, error::AppError};
use super::{
    entity::*,
    service::ForumService,
};

fn success<T: serde::Serialize>(data: T) -> impl IntoResponse {
    Json(json!({
        "code": 200,
        "message": "success",
        "data": data
    }))
}

//
// =====================================================================
// Boards
// =====================================================================
//
async fn list_boards(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let list = ForumService::get_board_list(&state).await?;
    Ok(success(json!({ "list": list })))
}

//
// =====================================================================
// Posts
// =====================================================================
//
async fn create_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Extension(user_id): Extension<String>,
    Json(req): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !headers.contains_key("Idempotency-Key") {
        return Err(AppError::BadRequest("Missing Idempotency-Key header".into()));
    }

    let post_id = ForumService::create_post(&state, &user_id, req).await?;
    let post = ForumService::get_post_detail(&state, &post_id, Some(&user_id)).await?;

    Ok(success(post))
}

async fn list_posts(
    State(state): State<AppState>,
    user_ext: Option<Extension<String>>,
    Query(query): Query<PostQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_ext.map(|x| x.0);

    let result = ForumService::get_post_list(&state, user_id.as_deref(), query).await?;
    Ok(success(result))
}

async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    user_ext: Option<Extension<String>>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_ext.map(|x| x.0);

    let result = ForumService::get_post_detail(&state, &id, user_id.as_deref()).await?;
    Ok(success(result))
}

async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::delete_post(&state, &id, &user_id).await?;
    Ok(success(serde_json::Value::Null))
}

async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::update_post(&state, &id, req).await?;
    let post = ForumService::get_post_detail(&state, &id, Some(&user_id)).await?;
    Ok(success(post))
}

//
// =====================================================================
// Interactions - Post
// =====================================================================
//
async fn like_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<LikeActionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (count, is_liked) =
        ForumService::toggle_like_post(&state, &id, &user_id, &req.actions).await?;

    Ok(success(json!({
        "current_like_count": count,
        "is_liked": is_liked
    })))
}

async fn collect_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<CollectActionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (is_collected, _total) =
        ForumService::toggle_collect_post(&state, &id, &user_id, &req.action).await?;

    Ok(success(json!({ "is_collected": is_collected })))
}

//
// =====================================================================
// Comments
// =====================================================================
//
async fn create_comment(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, AppError> {
    let comment = ForumService::create_comment(&state, &post_id, &user_id, req).await?;
    Ok(success(json!({
        "comment_id": comment.id,
        "comment": comment
    })))
}

async fn list_comments(
    State(state): State<AppState>,
    Path(post_id): Path<String>,
    user_ext: Option<Extension<String>>,
    Query(query): Query<CommentQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = user_ext.map(|x| x.0);

    let list = ForumService::get_comments(&state, &post_id, user_id.as_deref(), query).await?;
    Ok(success(list))
}

async fn delete_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::delete_comment(&state, &id, &user_id).await?;
    Ok(success(serde_json::Value::Null))
}

async fn like_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user_id): Extension<String>,
    Json(req): Json<LikeActionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let (count, is_liked) =
        ForumService::toggle_like_comment(&state, &id, &user_id, &req.actions).await?;

    Ok(success(json!({
        "current_like_count": count,
        "is_liked": is_liked
    })))
}

//
// =====================================================================
// Reports
// =====================================================================
//
async fn create_report(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Json(req): Json<CreateReportRequest>,
) -> Result<impl IntoResponse, AppError> {
    let id = ForumService::create_report(&state, &user_id, req).await?;
    Ok(success(json!({ "report_id": id })))
}

async fn admin_list_reports(
    State(state): State<AppState>,
    Extension(_user_id): Extension<String>,
    Query(query): Query<AdminReportQuery>,
) -> Result<impl IntoResponse, AppError> {
    let list = ForumService::admin_list_reports(&state, query).await?;
    Ok(success(list))
}

async fn admin_audit_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(_user_id): Extension<String>,
    Json(req): Json<AdminPostStatusRequest>,
) -> Result<impl IntoResponse, AppError> {
    ForumService::admin_audit_post(&state, &id, req).await?;
    Ok(success(serde_json::Value::Null))
}

//
// =====================================================================
// Router Export
// =====================================================================
//
pub fn router() -> Router<AppState> {
    Router::new()
        // boards
        .route("/boards", get(list_boards))

        // posts
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/:id", get(get_post).put(update_post).delete(delete_post))

        // post interactions
        .route("/posts/:id/like", post(like_post))
        .route("/posts/:id/collect", post(collect_post))

        // comments
        .route("/posts/:post_id/comments", get(list_comments).post(create_comment))
        .route("/comments/:id", delete(delete_comment))
        .route("/comments/:id/like", post(like_comment))

        // reports & admin
        .route("/reports", post(create_report))
        .route("/admin/reports", get(admin_list_reports))
        .route("/admin/posts/:id/audit", post(admin_audit_post))
}
