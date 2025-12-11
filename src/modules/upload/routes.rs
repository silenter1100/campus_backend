use crate::common::state::AppState;
use crate::modules::upload::UploadController;
use axum::{
    routing::{delete, get, post},
    Router,
};

/// 创建上传模块路由
/// 注意：认证通过 AuthUser 提取器在 controller 中处理
/// UploadService 从 AppState 中提取
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // 上传文件（需要认证）
        .route("/api/v1/storage/upload", post(UploadController::upload_file))
        // 获取我的文件列表（需要认证）
        .route("/api/v1/storage/files", get(UploadController::list_my_files))
        // 获取文件信息（公开）
        .route("/api/v1/storage/files/:file_id", get(UploadController::get_file_info))
        // 删除文件（需要认证）
        .route("/api/v1/storage/files/:file_id", delete(UploadController::delete_file))
}
