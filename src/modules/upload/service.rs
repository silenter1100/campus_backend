use crate::common::{AppError, FileDetector, FileInfo, FileType};
use crate::modules::upload::entity::{OssConfig, UploadRequest, UploadResult};
use aliyun_oss_client::{Client, ClientBuilder};
use std::sync::Arc;
use tracing::{error, info};

/// 上传服务
#[derive(Clone)]
pub struct UploadService {
    oss_client: Arc<Client>,
    config: OssConfig,
}

impl UploadService {
    /// 创建新的上传服务实例
    pub fn new(config: OssConfig) -> Result<Self, AppError> {
        let client = ClientBuilder::new()
            .access_key_id(&config.access_key_id)
            .access_key_secret(&config.access_key_secret)
            .endpoint(&config.endpoint)
            .bucket(&config.bucket)
            .build()
            .map_err(|e| {
                error!("Failed to create OSS client: {}", e);
                AppError::InternalServerError("OSS客户端初始化失败".to_string())
            })?;

        Ok(Self {
            oss_client: Arc::new(client),
            config,
        })
    }

    /// 上传文件到OSS
    pub async fn upload_file(&self, request: UploadRequest) -> Result<UploadResult, AppError> {
        // 1. 检测文件类型
        let file_info = FileDetector::detect_from_bytes(&request.file_data);
        
        if !file_info.is_supported {
            return Err(AppError::BadRequest(format!(
                "不支持的文件类型: {}",
                file_info.mime_type
            )));
        }

        // 2. 生成安全的文件名
        let safe_filename = FileDetector::generate_safe_filename(
            &request.original_filename,
            &file_info,
        );

        // 3. 根据文件类型确定存储路径
        let object_key = self.generate_object_key(&file_info, &safe_filename);

        info!(
            "Uploading file: {} -> {} (size: {} bytes)",
            request.original_filename,
            object_key,
            request.file_data.len()
        );

        // 4. 上传到OSS
        self.oss_client
            .put_object()
            .key(&object_key)
            .body(request.file_data.clone())
            .content_type(&file_info.mime_type)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to upload to OSS: {}", e);
                AppError::InternalServerError("文件上传失败".to_string())
            })?;

        // 5. 生成访问URL
        let url = format!("{}/{}", self.config.url_prefix.trim_end_matches('/'), object_key);

        // 6. 如果是图片，生成缩略图 (简化实现，实际可以用OSS图片处理)
        let thumbnail_url = if matches!(file_info.file_type, FileType::Image(_)) {
            Some(format!("{}?x-oss-process=image/resize,w_200,h_200", url))
        } else {
            None
        };

        Ok(UploadResult {
            url,
            thumbnail_url,
            filename: safe_filename,
            size: request.file_data.len() as i64,
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

    /// 获取文件信息 (用于下载前验证)
    pub async fn get_file_info(&self, object_key: &str) -> Result<Option<i64>, AppError> {
        match self.oss_client
            .head_object()
            .key(object_key)
            .send()
            .await
        {
            Ok(response) => {
                let content_length = response.content_length().unwrap_or(0);
                Ok(Some(content_length))
            }
            Err(e) => {
                if e.to_string().contains("404") {
                    Ok(None) // 文件不存在
                } else {
                    error!("Failed to get file info: {}", e);
                    Err(AppError::InternalServerError("获取文件信息失败".to_string()))
                }
            }
        }
    }

    /// 删除文件 (可选功能)
    pub async fn delete_file(&self, object_key: &str) -> Result<(), AppError> {
        self.oss_client
            .delete_object()
            .key(object_key)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to delete file: {}", e);
                AppError::InternalServerError("删除文件失败".to_string())
            })?;

        info!("File deleted: {}", object_key);
        Ok(())
    }
}

/// 从环境变量创建OSS配置
impl OssConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            access_key_id: std::env::var("OSS_ACCESS_KEY_ID")
                .map_err(|_| AppError::InternalServerError("OSS_ACCESS_KEY_ID not set".to_string()))?,
            access_key_secret: std::env::var("OSS_ACCESS_KEY_SECRET")
                .map_err(|_| AppError::InternalServerError("OSS_ACCESS_KEY_SECRET not set".to_string()))?,
            endpoint: std::env::var("OSS_ENDPOINT")
                .map_err(|_| AppError::InternalServerError("OSS_ENDPOINT not set".to_string()))?,
            bucket: std::env::var("OSS_BUCKET")
                .map_err(|_| AppError::InternalServerError("OSS_BUCKET not set".to_string()))?,
            url_prefix: std::env::var("OSS_URL_PREFIX")
                .map_err(|_| AppError::InternalServerError("OSS_URL_PREFIX not set".to_string()))?,
        })
    }
}