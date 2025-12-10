use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod modules;

use crate::modules::activity::service::ActivityServiceImpl;
use crate::modules::activity::controller::ActivityControllerState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 建立数据库连接池
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:123456@localhost/campus_backend")
        .await
        .expect("failed to connect db");

    let activity_service = ActivityServiceImpl::new(db);
    let activity_state = ActivityControllerState {
        service: activity_service,
    };

    println!("🔥 Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(activity_state.clone()))
            .configure(modules::activity::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
