use crate::common::{AppError, FileDetector, FileInfo, FileType};
use crate::modules::upload::entity::{OssConfig, UploadRequest, UploadResult, UploadFile};
use sqlx::MySqlPool;
use tracing::{error, info, warn};
use uuid::Uuid;

/// 上传服务
#[derive(Clone)]
pub struct UploadService {
    config: OssConfig,
    pool: MySqlPool,
}

impl UploadService {
    /// 创建新的上传服务实例
    pub fn new(config: OssConfig, pool: MySqlPool) -> Result<Self, AppError> {
        info!("Upload service initialized (Mock mode - OSS not connected)");
        Ok(Self {
            config,
            pool,
        })
    }

    /// 上传文件到OSS并记录到数据库
    pub async fn upload_file(
        &self,
        request: UploadRequest,
        user_id: &str,
    ) -> Result<UploadResult, AppError> {
        // 1. 检测文件类型
        let file_info = FileDetector::detect_from_bytes(&request.file_data);
        
        if !file_info.is_supported {
            return Err(AppError::BadRequest(format!(
                "不支持的文件类型: {}",
                file_info.mime_type
            )));
        }

        // 2. 生成文件ID和安全的文件名
        let file_id = Uuid::new_v4().to_string();
        let safe_filename = FileDetector::generate_safe_filename(
            &request.original_filename,
            &file_info,
        );

        // 3. 生成OSS存储路径
        let object_key = self.generate_object_key(&file_info, &safe_filename);

        info!(
            "Uploading file: {} -> {} (size: {} bytes, user: {})",
            request.original_filename,
            object_key,
            request.file_data.len(),
            user_id
        );

        // 4. TODO: 上传到OSS (当前为Mock模式)
        warn!("OSS upload is mocked - file not actually uploaded to cloud storage");
        // 实际部署时，在这里调用 OSS SDK 上传文件
        // 示例代码（需要配置真实的OSS客户端）:
        // self.oss_client.put_content_base(request.file_data.clone(), &file_info.mime_type, &object_key).await?;

        // 5. 生成访问URL (Mock URL)
        let url = format!("{}/{}", self.config.url_prefix.trim_end_matches('/'), object_key);

        // 6. 如果是图片，生成缩略图URL
        let thumbnail_url = if matches!(file_info.file_type, FileType::Image(_)) {
            Some(format!("{}?x-oss-process=image/resize,w_200,h_200", url))
        } else {
            None
        };

        // 7. 保存到数据库
        let file_size = request.file_data.len() as i64;
        sqlx::query(
            r#"
            INSERT INTO upload_files (id, user_id, file_path, file_url, file_size, is_used)
            VALUES (?, ?, ?, ?, ?, FALSE)
            "#
        )
        .bind(&file_id)
        .bind(user_id)
        .bind(&object_key)
        .bind(&url)
        .bind(file_size)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to save file record to database: {}", e);
            AppError::InternalError("保存文件记录失败".to_string())
        })?;

        info!("File record saved successfully: {} (id: {})", url, file_id);

        Ok(UploadResult {
            id: file_id,
            url,
            thumbnail_url,
            size: file_size,
        })
    }

    /// 生成OSS对象键 (存储路径)
    fn generate_object_key(&self, file_info: &FileInfo, filename: &str) -> String {
        let date = chrono::Utc::now().format("%Y/%m/%d");
        
        match &file_info.file_type {
            FileType::Image(_) => format!("images/{}/{}", date, filename),
            FileType::Document(_) => format!("documents/{}/{}", date, filename),
            _ => format!("others/{}/{}", date, filename),
        }
    }

    /// 根据文件ID获取文件信息
    pub async fn get_file_by_id(&self, file_id: &str) -> Result<Option<UploadFile>, AppError> {
        let file = sqlx::query_as::<_, UploadFile>(
            "SELECT * FROM upload_files WHERE id = ?"
        )
        .bind(file_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to query file: {}", e);
            AppError::InternalError("查询文件失败".to_string())
        })?;

        Ok(file)
    }

    /// 删除文件 (验证用户权限)
    pub async fn delete_file(&self, file_id: &str, user_id: &str) -> Result<(), AppError> {
        // 1. 查询文件记录
        let file = self.get_file_by_id(file_id).await?
            .ok_or_else(|| AppError::NotFound("文件不存在".to_string()))?;

        // 2. 验证权限（只能删除自己的文件）
        if file.user_id != user_id {
            return Err(AppError::Forbidden("无权删除此文件".to_string()));
        }

        // 3. TODO: 从OSS删除 (当前为Mock模式)
        warn!("OSS delete is mocked - file not actually deleted from cloud storage");
        // 实际部署时，在这里调用 OSS SDK 删除文件
        // 示例代码:
        // self.oss_client.delete_object(&file.file_path).await?;

        // 4. 从数据库删除
        sqlx::query("DELETE FROM upload_files WHERE id = ?")
            .bind(file_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to delete file record: {}", e);
                AppError::InternalError("删除文件记录失败".to_string())
            })?;

        info!("File record deleted: {} (id: {})", file.file_path, file_id);
        Ok(())
    }

    /// 获取用户的文件列表
    pub async fn list_user_files(&self, user_id: &str) -> Result<Vec<UploadFile>, AppError> {
        let files = sqlx::query_as::<_, UploadFile>(
            "SELECT * FROM upload_files WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to list user files: {}", e);
            AppError::InternalError("查询文件列表失败".to_string())
        })?;

        Ok(files)
    }

    /// 标记文件为已使用（可选功能）
    pub async fn mark_file_as_used(&self, file_id: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE upload_files SET is_used = TRUE WHERE id = ?")
            .bind(file_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to mark file as used: {}", e);
                AppError::InternalError("更新文件状态失败".to_string())
            })?;

        Ok(())
    }
}

/// 从环境变量创建OSS配置
impl OssConfig {
    pub fn from_env() -> Result<Self, AppError> {
        // 使用默认值，如果环境变量未设置
        Ok(Self {
            access_key_id: std::env::var("OSS_ACCESS_KEY_ID")
                .unwrap_or_else(|_| "mock_key_id".to_string()),
            access_key_secret: std::env::var("OSS_ACCESS_KEY_SECRET")
                .unwrap_or_else(|_| "mock_key_secret".to_string()),
            endpoint: std::env::var("OSS_ENDPOINT")
                .unwrap_or_else(|_| "oss-cn-hangzhou.aliyuncs.com".to_string()),
            bucket: std::env::var("OSS_BUCKET")
                .unwrap_or_else(|_| "mock_bucket".to_string()),
            url_prefix: std::env::var("OSS_URL_PREFIX")
                .unwrap_or_else(|_| "https://mock-bucket.oss-cn-hangzhou.aliyuncs.com".to_string()),
        })
    }
}
