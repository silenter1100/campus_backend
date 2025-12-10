use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use crate::common::db::DBPool;

pub mod controller;
pub mod entity;
pub mod service;

pub fn router() -> Router<DBPool> {
    // Public/User Routes
    let user_routes = Router::new()
        .route("/boards", get(controller::list_boards))
        .route("/posts", post(controller::create_post).get(controller::list_posts))
        .route("/posts/:id", get(controller::get_post).delete(controller::delete_post).patch(controller::update_post))
        .route("/posts/:id/like", post(controller::like_post))
        // NOTICE: JSON spec says GET /collect with body, using POST for robustness, 
        // or ensure your client sends body with GET.
        .route("/posts/:id/collect", post(controller::collect_post)) 
        .route("/posts/:id/comments", post(controller::create_comment).get(controller::list_comments))
        .route("/comments/:id", delete(controller::delete_comment))
        .route("/comments/:id/like", post(controller::like_comment))
        .route("/reports", post(controller::create_report));

    // Admin Routes
    let admin_routes = Router::new()
        .route("/reports", get(controller::admin_list_reports))
        .route("/posts/:id/status", patch(controller::admin_audit_post));

    Router::new()
        .nest("/api/v1/forum", user_routes)
        .nest("/api/v1/admin/forum", admin_routes)
}