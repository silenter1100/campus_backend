use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use crate::common::db::DBPool;
use crate::common::auth::{auth_middleware, admin_auth_middleware};

pub mod controller;
pub mod entity;
pub mod service;

pub fn router() -> Router<DBPool> {
    // 1. 公开接口 (不需要 Token)
    // 包括：查看板块、查看帖子列表、查看详情、查看评论、临时登录
    let public_routes = Router::new()
        .route("/boards", get(controller::list_boards))
        .route("/posts", get(controller::list_posts))
        .route("/posts/:id", get(controller::get_post))
        .route("/posts/:id/comments", get(controller::list_comments));

    // 2. 受保护接口 (必须带 Token，否则 401)
    // 包括：发帖、修改、删除、点赞、收藏、发评、举报
    let protected_routes = Router::new()
        .route("/posts", post(controller::create_post))
        .route("/posts/:id", delete(controller::delete_post).patch(controller::update_post))
        .route("/posts/:id/like", post(controller::like_post))
        .route("/posts/:id/collect", post(controller::collect_post))
        .route("/posts/:id/comments", post(controller::create_comment))
        .route("/comments/:id", delete(controller::delete_comment))
        .route("/comments/:id/like", post(controller::like_comment))
        .route("/reports", post(controller::create_report))
        .route_layer(axum::middleware::from_fn(auth_middleware));

    // 3. 管理员接口 
    let admin_routes = Router::new()
        .route("/reports", get(controller::admin_list_reports))
        .route("/posts/:id/status", patch(controller::admin_audit_post))
        .route_layer(axum::middleware::from_fn(admin_auth_middleware));

    // 4. 组合所有路由
    Router::new()
        .nest("/api/v1/forum", public_routes.merge(protected_routes))
        .nest("/api/v1/admin/forum", admin_routes)
}