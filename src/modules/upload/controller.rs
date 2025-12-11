use crate::common::AppError;
use crate::modules::upload::{UploadRequest, UploadResult, UploadService};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{error, info, warn};

/// 上传控制器
pub struct UploadController;

impl UploadController {
    /// POST /api/v1/storage/upload - 上传文件
    pub async fn upload_file(
        State(upload_service): State<Arc<UploadService>>,
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
            "Received upload request: {} ({} bytes)",
            original_filename,
            file_data.len()
        );

        // 创建上传请求
        let upload_request = UploadRequest {
            file_data,
            original_filename,
            file_type,
        };

        // 执行上传
        let result = upload_service.upload_file(upload_request).await?;

        // 返回成功响应
        Ok(Json(json!({
            "code": 200,
            "message": "上传成功",
            "data": {
                "url": result.url,
                "thumbnail_url": result.thumbnail_url,
                "filename": result.filename,
                "size": result.size
            }
        })))
    }
    /// GET /api/v1/storage/info/{filename} - 获取文件信息
    pub async fn get_file_info(
        State(upload_service): State<Arc<UploadService>>,
        axum::extract::Path(filename): axum::extract::Path<String>,
    ) -> Result<Json<Value>, AppError> {
        info!("Getting file info for: {}", filename);

        // 这里简化处理，实际应该从数据库查询完整路径
        // 假设文件名格式为: timestamp_uuid.extension
        let object_key = format!("images/2024/12/11/{}", filename); // 简化示例

        match upload_service.get_file_info(&object_key).await? {
            Some(size) => Ok(Json(json!({
                "code": 200,
                "message": "文件存在",
                "data": {
                    "filename": filename,
                    "size": size,
                    "exists": true
                }
            }))),
            None => Ok(Json(json!({
                "code": 404,
                "message": "文件不存在",
                "data": {
                    "filename": filename,
                    "exists": false
                }
            }))),
        }
    }

    /// DELETE /api/v1/storage/{filename} - 删除文件 (管理员功能)
    pub async fn delete_file(
        State(upload_service): State<Arc<UploadService>>,
        axum::extract::Path(filename): axum::extract::Path<String>,
    ) -> Result<Json<Value>, AppError> {
        info!("Deleting file: {}", filename);

        // 这里简化处理，实际应该从数据库查询完整路径
        let object_key = format!("images/2024/12/11/{}", filename); // 简化示例

        upload_service.delete_file(&object_key).await?;

        Ok(Json(json!({
            "code": 200,
            "message": "文件删除成功",
            "data": null
        })))
    }
}