// src/modules/activity/controller.rs

use actix_web::{get, post, patch, web, HttpResponse, Responder};
use std::sync::Arc;

use crate::modules::activity::entity::*;
use crate::modules::activity::service::{ActivityService, ServiceError};

/// 用于依赖注入的 state
#[derive(Clone)]
pub struct ActivityControllerState {
    pub service: Arc<dyn ActivityService>,
}

/// 在 mod.rs 里会调用这个方法挂载路由
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(list_categories)
            .service(list_tags)
            .service(list_activities)
            .service(get_activity_detail)
            .service(signup_activity)
            .service(cancel_signup)
            .service(list_my_activities)
            .service(create_activity)
            .service(update_activity)
            .service(submit_activity)
            .service(withdraw_activity)
            .service(list_signups)
            // .service(get_checkin_code)
            // .service(checkin)
            .service(admin_review_activity)
            .service(admin_block_activity),
    );
}

fn get_or_fake_user(opt: Option<web::ReqData<CurrentUser>>) -> CurrentUser {
    match opt {
        Some(u) => u.into_inner(),
        None => CurrentUser {
            id: 1,          // 测试阶段：假用户 id = 1
            is_admin: false,
        },
    }
}

fn get_or_fake_admin(opt: Option<web::ReqData<CurrentUser>>) -> CurrentUser {
    match opt {
        Some(u) => u.into_inner(),
        None => CurrentUser {
            id: 1,
            is_admin: true,   // 👈 测试阶段：假管理员
        },
    }
}


/// 一个帮助函数，把 ServiceResult -> HttpResponse<ApiResponse<_>>
fn to_http<T: serde::Serialize>(res: Result<T, ServiceError>) -> HttpResponse {
    match res {
        Ok(data) => HttpResponse::Ok().json(ApiResponse::ok(data)),
        Err(e) => HttpResponse::Ok().json(e.to_api_response::<()>()),
    }
}

//测试通过
/// ===== A. 辅助 API =====

#[get("/activity/categories")]
async fn list_categories(
    data: web::Data<ActivityControllerState>,
) -> impl Responder {
    let res = data.service.list_categories().await.map(|items| {
        items
            .into_iter()
            .map(|(key, name)| serde_json::json!({ "key": key, "name": name }))
            .collect::<Vec<_>>()
    });
    to_http(res)
}


#[get("/activity/tags")]
async fn list_tags(
    data: web::Data<ActivityControllerState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let keyword = query.get("keyword").cloned();
    let res = data.service.list_tags(keyword).await.map(|items| {
        items
            .into_iter()
            .map(|(key, name)| serde_json::json!({ "key": key, "name": name }))
            .collect::<Vec<_>>()
    });
    to_http(res)
}

//测试通过
/// ===== B. 公共活动 API =====

#[get("/activities")]
async fn list_activities(
    data: web::Data<ActivityControllerState>,
    query: web::Query<ListActivitiesQuery>,
    // 当前用户可选：未登录时为 None，取决于你怎么做 extractor
    current_user: Option<web::ReqData<CurrentUser>>,
) -> impl Responder {
    let user_ref = current_user.as_deref();
    let res = data
        .service
        .list_activities(user_ref, query.into_inner())
        .await;
    to_http(res)
}

#[get("/activities/{id}")]
async fn get_activity_detail(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    current_user: Option<web::ReqData<CurrentUser>>,
) -> impl Responder {
    let id = path.into_inner();
    let user_ref = current_user.as_deref();
    let res = data.service.get_activity_detail(user_ref, id).await;
    to_http(res)
}

//测试通过
/// ===== C. 用户报名相关 =====

#[post("/activities/{id}/signup")]
async fn signup_activity(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    body: web::Json<SignupActivityBody>,
    current_user: Option<web::ReqData<CurrentUser>>,
) -> impl Responder {
    let id = path.into_inner();
    let user = get_or_fake_user(current_user);         // 👈 使用 helper
    let res = data
        .service
        .signup_activity(&user, id, body.into_inner())
        .await;
    to_http(res)
}

#[post("/activities/{id}/cancel")]
async fn cancel_signup(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    current_user: Option<web::ReqData<CurrentUser>>,
) -> impl Responder {
    let id = path.into_inner();
    let user = get_or_fake_user(current_user);
    let res = data.service.cancel_signup(&user, id).await;
    to_http(res)
}

#[get("/me/activities")]
async fn list_my_activities(
    data: web::Data<ActivityControllerState>,
    query: web::Query<ListMyActivitiesQuery>,
    current_user: Option<web::ReqData<CurrentUser>>,
) -> impl Responder {
    let user = get_or_fake_user(current_user);
    let res = data
        .service
        .list_my_activities(&user, query.into_inner())
        .await;
    to_http(res)
}

//测试通过
/// ===== D. 举办方 API =====

#[post("/organizer/activities")]
async fn create_activity(
    data: web::Data<ActivityControllerState>,
    body: web::Json<CreateActivityBody>,
    // current_user: web::ReqData<CurrentUser>,
) -> impl Responder {
    // let user = current_user.into_inner();
    // let res = data
    //     .service
    //     .create_activity(&user, body.into_inner())
    //     .await;
    // to_http(res)
    let user = CurrentUser {
        id: 1,
        is_admin: true,
    };

    let res = data
        .service
        .create_activity(&user, body.into_inner())
        .await;

    to_http(res)
}


#[patch("/organizer/activities/{id}")]
async fn update_activity(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    body: web::Json<UpdateActivityBody>,
    current_user: web::ReqData<CurrentUser>,
) -> impl Responder {
    let id = path.into_inner();
    let user = current_user.into_inner();
    let res = data
        .service
        .update_activity(&user, id, body.into_inner())
        .await;
    to_http(res)
}

#[post("/organizer/activities/{id}/submit")]
async fn submit_activity(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    current_user: web::ReqData<CurrentUser>,
) -> impl Responder {
    let id = path.into_inner();
    let user = current_user.into_inner();
    let res = data.service.submit_activity(&user, id).await;
    to_http(res)
}

#[post("/organizer/activities/{id}/withdraw")]
async fn withdraw_activity(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    current_user: web::ReqData<CurrentUser>,
) -> impl Responder {
    let id = path.into_inner();
    let user = current_user.into_inner();
    let res = data.service.withdraw_activity(&user, id).await;
    to_http(res)
}

#[get("/organizer/activities/{id}/signups")]
async fn list_signups(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    query: web::Query<std::collections::HashMap<String, String>>,
    current_user: web::ReqData<CurrentUser>,
) -> impl Responder {
    let id = path.into_inner();
    let user = current_user.into_inner();

    let page = query
        .get("page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let page_size = query
        .get("page_size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);
    let status = query.get("status").cloned();

    let res = data
        .service
        .list_signups(&user, id, page, page_size, status)
        .await;
    to_http(res)
}


//弃用
/// ===== E. 签到 API =====

// #[get("/organizer/activities/{id}/checkin-code")]
// async fn get_checkin_code(
//     data: web::Data<ActivityControllerState>,
//     path: web::Path<i64>,
//     current_user: web::ReqData<CurrentUser>,
// ) -> impl Responder {
//     let id = path.into_inner();
//     let user = current_user.into_inner();
//     let res = data.service.get_checkin_code(&user, id).await.map(|(token, expire_at)| {
//         serde_json::json!({
//             "checkin_token": token,
//             "expire_at": expire_at,
//         })
//     });
//     to_http(res)
// }

// #[post("/activities/{id}/checkin")]
// async fn checkin(
//     data: web::Data<ActivityControllerState>,
//     path: web::Path<i64>,
//     body: web::Json<CheckinBody>,
//     current_user: web::ReqData<CurrentUser>,
// ) -> impl Responder {
//     let id = path.into_inner();
//     let user = current_user.into_inner();
//     let res = data
//         .service
//         .checkin(&user, id, body.into_inner())
//         .await;
//     to_http(res)
// }

//测试通过
/// ===== F. 管理员审核 API =====

#[post("/admin/activities/{id}/review")]
async fn admin_review_activity(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
    body: web::Json<ReviewActivityBody>,
    current_user: Option<web::ReqData<CurrentUser>>,   // 👈 变成 Option
) -> impl Responder {
    let id = path.into_inner();
    let user = get_or_fake_admin(current_user);        // 👈 用假管理员
    let res = data
        .service
        .admin_review_activity(&user, id, body.into_inner())
        .await;
    to_http(res)
}

#[post("/admin/activities/{id}/block")]
async fn admin_block_activity(
    data: web::Data<ActivityControllerState>,
    path: web::Path<i64>,
   current_user: Option<web::ReqData<CurrentUser>>,
) -> impl Responder {
    let id = path.into_inner();
    let user = get_or_fake_admin(current_user);
    let res = data.service.admin_block_activity(&user, id).await;
    to_http(res)
}
