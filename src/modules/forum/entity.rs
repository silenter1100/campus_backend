use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Local};

// =========================================================================
//  Enums
// =========================================================================

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "post_status", rename_all = "lowercase")]
pub enum PostStatus {
    Approved,
    Pending,
    Rejected,
    Hidden,
}

impl Default for PostStatus {
    fn default() -> Self {
        Self::Pending
    }
}

// =========================================================================
//  Request DTOs (Data Transfer Objects)
// =========================================================================

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub board_id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub media: Vec<MediaItem>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub media: Option<Vec<MediaItem>>,
}

#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub board_id: Option<String>,
    pub filter: Option<String>, // "all", "my_college"
    pub sort: Option<String>,   // "latest", "new", "hot"
    pub keyword: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LikeActionRequest {
    pub actions: String, // "like", "unlike"
}

#[derive(Debug, Deserialize)]
pub struct CollectActionRequest {
    pub action: String, // "collect", "uncollect"
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    pub reply_to_comment_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CommentQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReportRequest {
    pub target_type: String, // "post", "comment"
    pub target_id: String,
    pub reason: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminReportQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub status: Option<String>,
    pub target_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminPostStatusRequest {
    pub status: String, // APPROVED, REJECTED
    pub notes: Option<String>,
}

// =========================================================================
//  Inner Structures
// =========================================================================
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct MediaMeta {
    pub size: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub filename: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct MediaItem {
    #[serde(rename = "type")]
    pub media_type: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub meta: MediaMeta, 
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserLite {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub avatar_url: String,
    pub college: String,
}

#[derive(Debug, Serialize)]
pub struct PostStats {
    pub view_count: i32,
    pub like_count: i32,
    pub comment_count: i32,
}

#[derive(Debug, Serialize)]
pub struct UserInteraction {
    pub is_liked: bool,
    pub is_collected: bool,
}

// =========================================================================
//  Response VO (View Objects) - 对应 JSON 的 Schema
// =========================================================================

#[derive(Debug, Serialize)]
pub struct PostDetailVO {
    pub id: String,
    pub title: String,
    pub content: String,
    pub board_id: String,
    pub board_name: String,
    pub author: UserLite,
    pub tags: Vec<String>,
    pub media: Vec<MediaItem>,
    pub stats: PostStats,
    pub user_interaction: UserInteraction,
    pub status: String,
    pub report_count: i32,
    pub created_at: DateTime<Local>,
    pub last_replied_at: DateTime<Local>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PostLiteVO {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub cover_image_url: Option<String>,
    pub board_id: String,
    pub board_name: String,
    pub author: UserLite, 
    pub created_at: DateTime<Local>,
    pub stats: PostStats,
    pub user_interaction: UserInteraction,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BoardVO {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    #[serde(rename = "type")]
    pub board_type: String,
}

#[derive(Debug, Serialize)]
pub struct CommentVO {
    pub id: String,
    pub post_id: String,
    pub author: UserLite,
    pub content: String,
    pub parent_id: Option<String>,
    pub reply_to: Option<UserLite>, // The user being replied to
    pub stats: CommentStats,
    pub user_interaction: CommentInteraction,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Serialize)]
pub struct CommentStats {
    pub like_count: i32,
}
#[derive(Debug, Serialize)]
pub struct CommentInteraction {
    pub is_liked: bool,
}

// =========================================================================
//  Pagination Helper
// =========================================================================

#[derive(Debug, Serialize)]
pub struct Pagination<T> {
    pub list: Vec<T>,
    pub pagination: PageInfo,
}

#[derive(Debug, Serialize)]
pub struct PageInfo {
    pub total: i64,
    pub page: i64,
    pub page_size: i64, 
    pub pages: i64,
}