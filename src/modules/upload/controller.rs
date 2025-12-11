use crate::common::{AppError, auth::AuthUser};
use crate::modules::upload::{UploadRequest, UploadService};
use axum::{
    extract::{Multipart, Path, State},
    response::Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{error, info, warn};

/// 上传控制器
pub struct UploadController;

impl UploadController {
    /// POST /api/v1/storage/upload - 上传文件（需要认证）
    pub async fn upload_file(
        State(upload_service): State<Arc<UploadService>>,
        auth_user: AuthUser,
        mut multipart: Multipart,
    ) -> Result<Json<Value>, AppError> {
        let mut file_data: Option<Vec<u8>> = None;
        let mut original_filename: Option<String> = None;
        let mut file_type: Option<String> = None;

        // 解析multipart表单数据
        while let Some(field) = multipart.next_field().await.map_err(|e| {
            error!("Failed to read multipart field: {}", e);
            AppError::BadRequest("无效的表单数据".to_string())
        })? {
            let field_name = field.name().unwrap_or("").to_string();

            match field_name.as_str() {
                "file" => {
                    // 获取文件名
                    if let Some(filename) = field.file_name() {
                        original_filename = Some(filename.to_string());
                    }

                    // 读取文件数据
                    let data = field.bytes().await.map_err(|e| {
                        error!("Failed to read file data: {}", e);
                        AppError::BadRequest("文件读取失败".to_string())
                    })?;

                    file_data = Some(data.to_vec());
                }
                "file_type" => {
                    let data = field.bytes().await.map_err(|e| {
                        error!("Failed to read file_type: {}", e);
                        AppError::BadRequest("文件类型读取失败".to_string())
                    })?;
                    
                    if let Ok(type_str) = String::from_utf8(data.to_vec()) {
                        file_type = Some(type_str);
                    }
                }
                _ => {
                    warn!("Unknown field: {}", field_name);
                }
            }
        }

        // 验证必需字段
        let file_data = file_data.ok_or_else(|| {
            AppError::BadRequest("缺少文件数据".to_string())
        })?;

        let original_filename = original_filename.unwrap_or_else(|| {
            "unknown_file".to_string()
        });

        // 检查文件大小 (限制为10MB)
        const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if file_data.len() > MAX_FILE_SIZE {
            return Err(AppError::BadRequest(
                "文件大小超过限制 (最大10MB)".to_string(),
            ));
        }

        info!(
            "Received upload request: {} ({} bytes) from user: {}",
            original_filename,
            file_data.len(),
            auth_user.user_id
        );

        // 创建上传请求
        let upload_request = UploadRequest {
            file_data,
            original_filename,
            file_type,
        };

        // 执行上传
        let result = upload_service.upload_file(upload_request, &auth_user.user_id).await?;

        // 返回成功响应
        Ok(Json(json!({
            "code": 200,
            "message": "上传成功",
            "data": {
                "id": result.id,
                "url": result.url,
                "thumbnail_url": result.thumbnail_url,
                "size": result.size
            }
        })))
    }

    /// GET /api/v1/storage/files/{file_id} - 获取文件信息
    pub async fn get_file_info(
        State(upload_service): State<Arc<UploadService>>,
        Path(file_id): Path<String>,
    ) -> Result<Json<Value>, AppError> {
        info!("Getting file info for: {}", file_id);

        match upload_service.get_file_by_id(&file_id).await? {
            Some(file) => Ok(Json(json!({
                "code": 200,
                "message": "文件存在",
                "data": {
                    "id": file.id,
                    "url": file.file_url,
                    "size": file.file_size,
                    "created_at": file.created_at,
                    "is_used": file.is_used
                }
            }))),
            None => Err(AppError::NotFound("文件不存在".to_string())),
        }
    }

    /// DELETE /api/v1/storage/files/{file_id} - 删除文件（需要认证，只能删除自己的文件）
    pub async fn delete_file(
        State(upload_service): State<Arc<UploadService>>,
        auth_user: AuthUser,
        Path(file_id): Path<String>,
    ) -> Result<Json<Value>, AppError> {
        info!("Deleting file: {} by user: {}", file_id, auth_user.user_id);

        upload_service.delete_file(&file_id, &auth_user.user_id).await?;

        Ok(Json(json!({
            "code": 200,
            "message": "文件删除成功",
            "data": null
        })))
    }

    /// GET /api/v1/storage/files - 获取当前用户的文件列表
    pub async fn list_my_files(
        State(upload_service): State<Arc<UploadService>>,
        auth_user: AuthUser,
    ) -> Result<Json<Value>, AppError> {
        info!("Listing files for user: {}", auth_user.user_id);

        let files = upload_service.list_user_files(&auth_user.user_id).await?;

        Ok(Json(json!({
            "code": 200,
            "message": "查询成功",
            "data": files
        })))
    }
}