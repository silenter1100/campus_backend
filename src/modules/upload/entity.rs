use serde::{Deserialize, Serialize};

/// 文件上传结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResult {
    /// 文件在OSS中的永久访问链接
    pub url: String,
    /// 如果是图片，返回缩略图URL (可选)
    pub thumbnail_url: Option<String>,
    /// 服务器保存的文件名
    pub filename: String,
    /// 文件大小，单位：字节
    pub size: i64,
}

/// 文件上传请求
#[derive(Debug)]
pub struct UploadRequest {
    /// 文件二进制数据
    pub file_data: Vec<u8>,
    /// 原始文件名
    pub original_filename: String,
    /// 文件类型提示 (可选)
    pub file_type: Option<String>,
}

/// OSS配置
#[derive(Debug, Clone)]
pub struct OssConfig {
    /// OSS访问密钥ID
    pub access_key_id: String,
    /// OSS访问密钥Secret
    pub access_key_secret: String,
    /// OSS Endpoint
    pub endpoint: String,
    /// OSS Bucket名称
    pub bucket: String,
    /// 文件URL前缀 (CDN域名或OSS域名)
    pub url_prefix: String,
}