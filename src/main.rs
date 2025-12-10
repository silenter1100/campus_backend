use axum::{middleware, Router};
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

mod common;
mod modules;
use common::{db, auth};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // 确保 tracing-subscriber 在所有操作前初始化
    tracing_subscriber::fmt::init(); 

    let pool = db::init_db_pool().await;

    // 正确的顺序：
    let app = Router::new()
        .merge(modules::forum::router())
        .with_state(pool) 
        .layer(middleware::from_fn(auth::auth_middleware)) 
        .layer(TraceLayer::new_for_http()); 

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// =========================================================================
// 👇 把这段代码贴在 src/main.rs 的最底部
// =========================================================================

// =========================================================================
// 👇 把这段代码覆盖 src/main.rs 最底部的 #[cfg(test)] 模块
// =========================================================================

// =========================================================================
// 👇 把这段代码覆盖 src/main.rs 最底部的 #[cfg(test)] 模块
// =========================================================================

#[cfg(test)]
mod integration_tests {
    use crate::modules::forum;
    use axum::Router;
    use sqlx::mysql::MySqlPoolOptions;
    use tokio::net::TcpListener;
    use serde_json::json;

    // 🛠️ 启动测试环境
    async fn spawn_app() -> String {
        dotenv::dotenv().ok(); 
        
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to DB");

        // 模拟中间件：所有请求都视为 'test-user-001' 发起的
        let app = Router::new()
            .merge(forum::router())
            .layer(axum::middleware::from_fn(|mut req: axum::extract::Request, next: axum::middleware::Next| async move {
                req.extensions_mut().insert("test-user-001".to_string());
                next.run(req).await
            }))
            .with_state(pool);

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        format!("http://127.0.0.1:{}", port)
    }

    // 🚀 15步全接口显式测试 (Verbose Mode)
    #[tokio::test]
    async fn test_all_15_interfaces() {
        let base_url = spawn_app().await;
        let client = reqwest::Client::new();
        
        println!("🚀 开始 15 步全接口显式测试 (详细打印版)...");

        // =====================================================================
        // 1. [GET] 获取板块列表 (/boards)
        // =====================================================================
        println!("\n📍 [Step 1] 获取板块列表...");
        let resp = client.get(format!("{}/api/v1/forum/boards", base_url)).send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 1 返回]: {:#?}", body); // 打印内容

        // =====================================================================
        // 2. [POST] 发布帖子 (/posts)
        // =====================================================================
        println!("\n📍 [Step 2] 发布帖子...");
        let post_data = json!({
            "board_id": "board-life",
            "title": "Rust显式测试贴",
            "content": "这是一条严格测试每一步的帖子，并带有详细打印。",
            "tags": ["test", "verbose"],
            "media": [
                {
                    "type": "image",
                    "url": "https://example.com/test.jpg",
                    "thumbnail_url": null,
                    "meta": { "width": "100", "height": "100" } 
                }
            ]
        });
        
        let resp = client.post(format!("{}/api/v1/forum/posts", base_url))
            .header("Idempotency-Key", "test-key-step-2-verbose")
            .json(&post_data)
            .send().await.unwrap();
        
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 2 返回]: {:#?}", body); // 打印内容

        let post_id = body["data"]["id"].as_str().expect("缺少 post id").to_string();
        println!("✅ 拿到 Post ID: {}", post_id);

        // =====================================================================
        // 3. [GET] 获取帖子列表 (/posts)
        // =====================================================================
        println!("\n📍 [Step 3] 获取帖子列表...");
        let resp = client.get(format!("{}/api/v1/forum/posts", base_url))
            .query(&[("board_id", "board-life")])
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        // 这里列表可能很长，我们只打印前 2 个元素或者 summary，防止刷屏，也可以全打
        println!("🔎 [Step 3 返回]: {:#?}", body); 

        // =====================================================================
        // 4. [GET] 获取帖子详情 (/posts/:id)
        // =====================================================================
        println!("\n📍 [Step 4] 获取帖子详情...");
        let resp = client.get(format!("{}/api/v1/forum/posts/{}", base_url, post_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 4 返回]: {:#?}", body);

        // =====================================================================
        // 5. [PATCH] 修改帖子 (/posts/:id)
        // =====================================================================
        println!("\n📍 [Step 5] 修改帖子...");
        let update_data = json!({ "title": "Rust显式测试贴-已修改" });
        let resp = client.patch(format!("{}/api/v1/forum/posts/{}", base_url, post_id))
            .json(&update_data)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 5 返回]: {:#?}", body); // 你的 update 接口现在返回的是详情，所以能看到

        // =====================================================================
        // 6. [POST] 点赞帖子 (/posts/:id/like)
        // =====================================================================
        println!("\n📍 [Step 6] 点赞帖子...");
        let resp = client.post(format!("{}/api/v1/forum/posts/{}/like", base_url, post_id))
            .json(&json!({"actions": "like"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 6 返回]: {:#?}", body);

        // =====================================================================
        // 7. [POST] 收藏帖子 (/posts/:id/collect)
        // =====================================================================
        println!("\n📍 [Step 7] 收藏帖子...");
        let resp = client.post(format!("{}/api/v1/forum/posts/{}/collect", base_url, post_id))
            .json(&json!({"action": "collect"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 7 返回]: {:#?}", body);

        // =====================================================================
        // 8. [POST] 发表评论 (/posts/:id/comments)
        // =====================================================================
        println!("\n📍 [Step 8] 发表评论...");
        let comment_data = json!({
            "content": "这是一条显式测试的评论",
            "reply_to_comment_id": null
        });
        let resp = client.post(format!("{}/api/v1/forum/posts/{}/comments", base_url, post_id))
            .json(&comment_data)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 8 返回]: {:#?}", body);
        let comment_id = body["data"]["comment_id"].as_str().unwrap().to_string();

        // =====================================================================
        // 9. [GET] 获取评论列表 (/posts/:id/comments)
        // =====================================================================
        println!("\n📍 [Step 9] 获取评论列表...");
        let resp = client.get(format!("{}/api/v1/forum/posts/{}/comments", base_url, post_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 9 返回]: {:#?}", body);

        // =====================================================================
        // 10. [POST] 点赞评论 (/comments/:id/like)
        // =====================================================================
        println!("\n📍 [Step 10] 点赞评论...");
        let resp = client.post(format!("{}/api/v1/forum/comments/{}/like", base_url, comment_id))
            .json(&json!({"actions": "like"}))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 10 返回]: {:#?}", body);

        // =====================================================================
        // 11. [POST] 举报内容 (/reports)
        // =====================================================================
        println!("\n📍 [Step 11] 举报内容...");
        let report_data = json!({
            "target_type": "post",
            "target_id": post_id,
            "reason": "spam",
            "description": "显式测试举报"
        });
        let resp = client.post(format!("{}/api/v1/forum/reports", base_url))
            .json(&report_data)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 11 返回]: {:#?}", body);

        // =====================================================================
        // 12. [GET] 管理员获取举报列表 (/admin/forum/reports)
        // =====================================================================
        println!("\n📍 [Step 12] 管理员获取举报列表...");
        let resp = client.get(format!("{}/api/v1/admin/forum/reports", base_url))
            .query(&[("status", "new")])
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 12 返回]: {:#?}", body);

        // =====================================================================
        // 13. [PATCH] 管理员审核帖子 (/admin/forum/posts/:id/status)
        // =====================================================================
        println!("\n📍 [Step 13] 管理员审核帖子...");
        let audit_data = json!({ "status": "rejected", "notes": "显式测试拒绝" });
        let resp = client.patch(format!("{}/api/v1/admin/forum/posts/{}/status", base_url, post_id))
            .json(&audit_data)
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 13 返回]: {:#?}", body);

        // =====================================================================
        // 14. [DELETE] 删除评论 (/comments/:id)
        // =====================================================================
        println!("\n📍 [Step 14] 删除评论...");
        let resp = client.delete(format!("{}/api/v1/forum/comments/{}", base_url, comment_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);
        
        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 14 返回]: {:#?}", body);

        // =====================================================================
        // 15. [DELETE] 删除帖子 (/posts/:id)
        // =====================================================================
        println!("\n📍 [Step 15] 删除帖子...");
        let resp = client.delete(format!("{}/api/v1/forum/posts/{}", base_url, post_id))
            .send().await.unwrap();
        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = resp.json().await.unwrap();
        println!("🔎 [Step 15 返回]: {:#?}", body);

        println!("\n🎉🎉🎉 15 个接口全部测试通过！日志如上。");
    }
}