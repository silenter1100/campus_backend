pub mod auth;
pub mod db;
pub mod dev_tools;
pub mod error;
pub mod file_detector;

pub use error::AppError;
pub use file_detector::{FileDetector, FileInfo, FileType};
