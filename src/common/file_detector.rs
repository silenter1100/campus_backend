use std::path::Path;

/// 支持的文件类型
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Image(ImageFormat),
    Document(DocumentFormat),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Bmp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentFormat {
    Pdf,
    Doc,
    Docx,
    Txt,
}

/// 文件检测结果
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub file_type: FileType,
    pub mime_type: String,
    pub extension: String,
    pub is_supported: bool,
}

/// 文件类型检测器
pub struct FileDetector;

impl FileDetector {
    /// 通过文件内容检测文件类型（推荐方法）
    pub fn detect_from_bytes(data: &[u8]) -> FileInfo {
        // 使用 infer crate 检测文件类型
        if let Some(kind) = infer::get(data) {
            let mime_type = kind.mime_type().to_string();
            let extension = kind.extension().to_string();
            
            let file_type = match kind.matcher_type() {
                infer::MatcherType::Image => {
                    match extension.as_str() {
                        "png" => FileType::Image(ImageFormat::Png),
                        "jpg" | "jpeg" => FileType::Image(ImageFormat::Jpeg),
                        "gif" => FileType::Image(ImageFormat::Gif),
                        "webp" => FileType::Image(ImageFormat::WebP),
                        "bmp" => FileType::Image(ImageFormat::Bmp),
                        _ => FileType::Unknown,
                    }
                }
                infer::MatcherType::Doc => {
                    match extension.as_str() {
                        "pdf" => FileType::Document(DocumentFormat::Pdf),
                        "doc" => FileType::Document(DocumentFormat::Doc),
                        "docx" => FileType::Document(DocumentFormat::Docx),
                        _ => FileType::Unknown,
                    }
                }
                _ => FileType::Unknown,
            };
            
            let is_supported = Self::is_supported_type(&file_type);
            
            FileInfo {
                file_type,
                mime_type,
                extension,
                is_supported,
            }
        } else {
            // 无法检测，返回未知类型
            FileInfo {
                file_type: FileType::Unknown,
                mime_type: "application/octet-stream".to_string(),
                extension: "bin".to_string(),
                is_supported: false,
            }
        }
    }
    
    /// 通过文件名检测（备用方法，不够准确）
    pub fn detect_from_filename(filename: &str) -> FileInfo {
        let path = Path::new(filename);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
            
        let (file_type, mime_type) = match extension.as_str() {
            "png" => (FileType::Image(ImageFormat::Png), "image/png"),
            "jpg" | "jpeg" => (FileType::Image(ImageFormat::Jpeg), "image/jpeg"),
            "gif" => (FileType::Image(ImageFormat::Gif), "image/gif"),
            "webp" => (FileType::Image(ImageFormat::WebP), "image/webp"),
            "bmp" => (FileType::Image(ImageFormat::Bmp), "image/bmp"),
            "pdf" => (FileType::Document(DocumentFormat::Pdf), "application/pdf"),
            "doc" => (FileType::Document(DocumentFormat::Doc), "application/msword"),
            "docx" => (FileType::Document(DocumentFormat::Docx), "application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
            "txt" => (FileType::Document(DocumentFormat::Txt), "text/plain"),
            _ => (FileType::Unknown, "application/octet-stream"),
        };
        
        let is_supported = Self::is_supported_type(&file_type);
        
        FileInfo {
            file_type,
            mime_type: mime_type.to_string(),
            extension,
            is_supported,
        }
    }
    
    /// 检查是否为支持的文件类型
    pub fn is_supported_type(file_type: &FileType) -> bool {
        match file_type {
            FileType::Image(ImageFormat::Png) |
            FileType::Image(ImageFormat::Jpeg) |
            FileType::Document(DocumentFormat::Pdf) => true,
            _ => false,
        }
    }
    
    /// 生成安全的文件名
    pub fn generate_safe_filename(_original_name: &str, file_info: &FileInfo) -> String {
        use uuid::Uuid;
        
        let uuid = Uuid::new_v4();
        let timestamp = chrono::Utc::now().timestamp();
        
        // 格式: timestamp_uuid.extension
        format!("{}_{}.{}", timestamp, uuid, file_info.extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_png_from_bytes() {
        // PNG 文件头: 89 50 4E 47 0D 0A 1A 0A
        let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let info = FileDetector::detect_from_bytes(&png_header);
        
        assert_eq!(info.file_type, FileType::Image(ImageFormat::Png));
        assert_eq!(info.mime_type, "image/png");
        assert_eq!(info.extension, "png");
        assert!(info.is_supported);
    }
    
    #[test]
    fn test_detect_jpeg_from_bytes() {
        // JPEG 文件头: FF D8 FF
        let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let info = FileDetector::detect_from_bytes(&jpeg_header);
        
        assert_eq!(info.file_type, FileType::Image(ImageFormat::Jpeg));
        assert_eq!(info.mime_type, "image/jpeg");
        assert_eq!(info.extension, "jpg");
        assert!(info.is_supported);
    }
    
    #[test]
    fn test_detect_pdf_from_bytes() {
        // PDF 文件头: %PDF
        let pdf_header = b"%PDF-1.4";
        let info = FileDetector::detect_from_bytes(pdf_header);
        
        assert_eq!(info.file_type, FileType::Document(DocumentFormat::Pdf));
        assert_eq!(info.mime_type, "application/pdf");
        assert_eq!(info.extension, "pdf");
        assert!(info.is_supported);
    }
    
    #[test]
    fn test_detect_from_filename() {
        let info = FileDetector::detect_from_filename("test.png");
        assert_eq!(info.file_type, FileType::Image(ImageFormat::Png));
        assert!(info.is_supported);
        
        let info = FileDetector::detect_from_filename("document.pdf");
        assert_eq!(info.file_type, FileType::Document(DocumentFormat::Pdf));
        assert!(info.is_supported);
    }
    
    #[test]
    fn test_generate_safe_filename() {
        let info = FileInfo {
            file_type: FileType::Image(ImageFormat::Png),
            mime_type: "image/png".to_string(),
            extension: "png".to_string(),
            is_supported: true,
        };
        
        let filename = FileDetector::generate_safe_filename("test.png", &info);
        assert!(filename.ends_with(".png"));
        assert!(filename.contains("_"));
    }
}