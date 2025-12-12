// src/modules/activity/routes.rs

use axum::{
    routing::{get, post, patch},
    Router,
};

use crate::common::state::AppState;
use super::controller;

/// 创建活动模块的路由
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // 公共接口
        .route("/api/v1/activities", get(controller::list_activities))
        .route("/api/v1/activities/:id", get(controller::get_activity_detail))
        .route("/api/v1/activities/:id/signup", post(controller::signup_activity))
        .route("/api/v1/activities/:id/cancel", post(controller::cancel_signup))
        .route("/api/v1/activities/:id/collect", post(controller::collect_activity))
        .route("/api/v1/activities/:id/uncollect", post(controller::uncollect_activity))
        .route("/api/v1/my-activities", get(controller::list_my_activities))
        
        // 管理员接口
        .route("/api/v1/admin/activities", post(controller::create_activity))
        .route("/api/v1/admin/activities/:id", patch(controller::update_activity))
        .route("/api/v1/admin/activities/:id/signups", get(controller::list_signups))
        .route("/api/v1/admin/activities/:id/review", post(controller::admin_review_activity))
        .route("/api/v1/admin/activities/:id/block", post(controller::admin_block_activity))
}