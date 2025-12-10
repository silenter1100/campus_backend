use super::entity::*;
use crate::common::{db::DBPool, error::AppError};
use chrono::{DateTime, Local};
use sqlx::{Connection, MySql, QueryBuilder, Row};
use uuid::Uuid;

pub struct ForumService;

impl ForumService {
    // -------------------------------------------------------------------------
    // Board
    // -------------------------------------------------------------------------
    pub async fn get_board_list(pool: &DBPool) -> Result<Vec<BoardVO>, AppError> {
        let boards = sqlx::query_as!(
            BoardVO,
            r#"
            SELECT 
                id, 
                name, 
                COALESCE(icon, '') as icon, 
                COALESCE(description, '') as description, 
                COALESCE(type, 'general') as board_type 
            FROM boards 
            WHERE is_deleted = 0 
            ORDER BY sort_order ASC
            "#
        )
        .fetch_all(pool)
        .await?;
        Ok(boards)
    }

    // -------------------------------------------------------------------------
    // Post - Create
    // -------------------------------------------------------------------------
    pub async fn create_post(
        pool: &DBPool,
        user_id: &str,
        req: CreatePostRequest,
    ) -> Result<String, AppError> {
        let mut tx = pool.begin().await?;
        let post_id = Uuid::new_v4().to_string();
        let now = Local::now();

        // 1. Insert Post
        sqlx::query!(
            r#"
            INSERT INTO posts (id, board_id, author_id, title, content, status, created_at, updated_at, last_replied_at)
            VALUES (?, ?, ?, ?, ?, 'approved', ?, ?, ?)
            "#,
            post_id,
            req.board_id,
            user_id,
            req.title,
            req.content,
            now,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;

        // 2. Insert Tags
        for tag in req.tags {
             sqlx::query!(
                "INSERT INTO post_tags (post_id, tag_name) VALUES (?, ?)",
                post_id,
                tag
            ).execute(&mut *tx).await?;
        }

        // 3. Insert Media
        for media in req.media {
            let meta_json = serde_json::to_value(&media.meta).unwrap_or_default();
            sqlx::query!(
                r#"
                INSERT INTO post_medias (post_id, type, url, thumbnail_url, meta) 
                VALUES (?, ?, ?, ?, ?)
                "#,
                post_id,
                media.media_type,
                media.url,
                media.thumbnail_url,
                meta_json
            ).execute(&mut *tx).await?;
        }

        // 4. Init Stats
        sqlx::query!(
            "INSERT INTO post_stats (post_id, view_count, like_count, comment_count) VALUES (?, 0, 0, 0)",
            post_id
        ).execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(post_id)
    }

    // -------------------------------------------------------------------------
    // Post - Update (全量更新)
    // -------------------------------------------------------------------------
    pub async fn update_post(pool: &DBPool, post_id: &str, req: UpdatePostRequest) -> Result<(), AppError> {
        let mut qb: QueryBuilder<MySql> = QueryBuilder::new("UPDATE posts SET updated_at = NOW() ");

        if let Some(title) = req.title {
            qb.push(", title = ");
            qb.push_bind(title);
        }
        if let Some(content) = req.content {
            qb.push(", content = ");
            qb.push_bind(content);
        }
        
        qb.push(" WHERE id = ");
        qb.push_bind(post_id);

        let mut tx = pool.begin().await?;
        
        // 1. 更新主表
        qb.build().execute(&mut *tx).await?;

        // 2. 更新 Tags (全量替换)
        if let Some(tags) = req.tags {
            sqlx::query!("DELETE FROM post_tags WHERE post_id = ?", post_id).execute(&mut *tx).await?;
            for tag in tags {
                sqlx::query!("INSERT INTO post_tags (post_id, tag_name) VALUES (?, ?)", post_id, tag)
                    .execute(&mut *tx).await?;
            }
        }

        // 3. 更新 Media (全量替换)
        if let Some(media) = req.media {
            sqlx::query!("DELETE FROM post_medias WHERE post_id = ?", post_id).execute(&mut *tx).await?;
            for item in media {
                let meta_json = serde_json::to_value(&item.meta).unwrap_or_default(); 
                
                sqlx::query!(
                    r#"
                    INSERT INTO post_medias (post_id, type, url, thumbnail_url, meta) 
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                    post_id, 
                    item.media_type, 
                    item.url, 
                    item.thumbnail_url, 
                    meta_json
                ).execute(&mut *tx).await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }


    // -------------------------------------------------------------------------
    // Post - List (开启分页并实现 COUNT 查询)
    // -------------------------------------------------------------------------
    pub async fn get_post_list(
        pool: &DBPool,
        current_user_id: Option<&str>,
        query: PostQuery,
    ) -> Result<Pagination<PostLiteVO>, AppError> {
        
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10).max(1).min(100);
        let offset = (page - 1) * page_size;

        // =====================================================================
        // 🔥 步骤 1: 构建 COUNT 查询 (获取总记录数)
        // =====================================================================
        let mut count_qb: QueryBuilder<MySql> = QueryBuilder::new(
            r#"
            SELECT COUNT(p.id) 
            FROM posts p
            JOIN boards b ON p.board_id = b.id
            JOIN users u ON p.author_id = u.id
            JOIN post_stats s ON p.id = s.post_id
            WHERE p.status IN ('approved', 'pending') AND p.is_deleted = 0
            "#
        );

        // Filters for COUNT Query
        if let Some(bid) = &query.board_id {
            if !bid.is_empty() {
                count_qb.push(" AND p.board_id = ");
                count_qb.push_bind(bid);
            }
        }
        
        if let Some(kw) = &query.keyword {
            if !kw.is_empty() {
                count_qb.push(" AND (p.title LIKE ");
                count_qb.push_bind(format!("%{}%", kw));
                count_qb.push(" OR p.content LIKE ");
                count_qb.push_bind(format!("%{}%", kw));
                count_qb.push(")");
            }
        }

        // 执行 COUNT 查询
        let total: i64 = count_qb
            .build_query_scalar() // 使用 build_query_scalar 方便直接获取单个值
            .fetch_one(pool)
            .await?;
        
        // 如果总数为 0，直接返回空列表
        if total == 0 {
            return Ok(Pagination {
                list: vec![],
                pagination: PageInfo { total: 0, page: 1, page_size, pages: 0 },
            });
        }

        // =====================================================================
        // 步骤 2: 构建 DATA 查询 (获取当前页数据)
        // =====================================================================
        let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
            r#"
            SELECT 
                p.id, p.title, p.content, p.created_at,
                b.id as board_id, b.name as board_name,
                u.id as u_id, u.student_id as u_std_id, u.name as u_name, u.avatar_url as u_avatar, u.college as u_college,
                s.view_count, s.like_count, s.comment_count,
                (SELECT url FROM post_medias pm WHERE pm.post_id = p.id LIMIT 1) as cover_image,
                EXISTS(SELECT 1 FROM post_likes pl WHERE pl.post_id = p.id AND pl.user_id = 
            "#
        );
        
        // Dynamic binding for user interaction check (is_liked)
        match current_user_id {
            Some(uid) => { qb.push_bind(uid); },
            None => { qb.push("NULL"); }
        }

        qb.push(") as is_liked, ");
        qb.push("EXISTS(SELECT 1 FROM post_collections pc WHERE pc.post_id = p.id AND pc.user_id = ");
        // Dynamic binding for user interaction check (is_collected)
        match current_user_id {
            Some(uid) => { qb.push_bind(uid); },
            None => { qb.push("NULL"); }
        }
        qb.push(") as is_collected ");

        qb.push(r#"
            FROM posts p
            JOIN boards b ON p.board_id = b.id
            JOIN users u ON p.author_id = u.id
            JOIN post_stats s ON p.id = s.post_id
            WHERE p.status IN ('approved', 'pending') AND p.is_deleted = 0
        "#);

        // Filters for DATA Query (复用 COUNT 查询的 filters)
        if let Some(bid) = &query.board_id {
            if !bid.is_empty() {
                qb.push(" AND p.board_id = ");
                qb.push_bind(bid);
            }
        }
        
        if let Some(kw) = &query.keyword {
            if !kw.is_empty() {
                qb.push(" AND (p.title LIKE ");
                qb.push_bind(format!("%{}%", kw));
                qb.push(" OR p.content LIKE ");
                qb.push_bind(format!("%{}%", kw));
                qb.push(")");
            }
        }

        // Sorting
        match query.sort.as_deref() {
            Some("hot") => qb.push(" ORDER BY s.view_count DESC, p.created_at DESC"),
            Some("latest") => qb.push(" ORDER BY p.last_replied_at DESC"),
            _ => qb.push(" ORDER BY p.created_at DESC"),
        };

        // 🔥 步骤 3: 启用 LIMIT 和 OFFSET
        qb.push(" LIMIT ");
        qb.push_bind(page_size);
        qb.push(" OFFSET ");
        qb.push_bind(offset);

        let rows = qb.build().fetch_all(pool).await?;
        
        // Manual Mapping from Row to Struct...
        let mut list = Vec::new();
        for row in rows {
            let content: String = row.get("content");
            let summary = if content.len() > 100 { format!("{}...", &content[0..100]) } else { content };
            
            // Get Tags (N+1 query)
            let post_id: String = row.get("id");
            let tags = sqlx::query_scalar!("SELECT tag_name FROM post_tags WHERE post_id = ?", post_id)
                .fetch_all(pool).await?;

            list.push(PostLiteVO {
                id: post_id,
                title: row.get("title"),
                summary,
                cover_image_url: row.get("cover_image"),
                board_id: row.get("board_id"),
                board_name: row.get("board_name"),
                author: UserLite {
                    id: row.get("u_id"),
                    student_id: row.get("u_std_id"),
                    name: row.get("u_name"),
                    avatar_url: row.get("u_avatar"),
                    college: row.get("u_college"),
                },
                created_at: row.get("created_at"),
                stats: PostStats {
                    view_count: row.get("view_count"),
                    like_count: row.get("like_count"),
                    comment_count: row.get("comment_count"),
                },
                user_interaction: UserInteraction {
                    is_liked: row.get::<i64, _>("is_liked") == 1,
                    is_collected: row.get::<i64, _>("is_collected") == 1,
                },
                tags,
            });
        }

        // =====================================================================
        // 步骤 4: 返回真实分页信息 (基于 COUNT)
        // =====================================================================
        let pages = (total as f64 / page_size as f64).ceil() as i64;
        
        Ok(Pagination {
            list,
            pagination: PageInfo {
                total,
                page,
                page_size,
                pages,
            }
        })
    }

    // -------------------------------------------------------------------------
    // Post - Detail
    // -------------------------------------------------------------------------
    pub async fn get_post_detail(pool: &DBPool, post_id: &str, user_id: Option<&str>) -> Result<PostDetailVO, AppError> {
        let _ = sqlx::query!("UPDATE post_stats SET view_count = view_count + 1 WHERE post_id = ?", post_id)
            .execute(pool).await;

        let row = sqlx::query!(
            r#"
            SELECT 
                p.id, p.title, p.content, 
                COALESCE(p.status, 'approved') as status, -- 给 status 默认值
                p.created_at, p.last_replied_at,
                b.id as board_id, b.name as board_name,
                u.id as u_id, u.student_id, u.name as u_name, 
                COALESCE(u.avatar_url, '') as avatar_url, -- 给头像默认值
                COALESCE(u.college, '') as college,       -- 给学院默认值
                COALESCE(s.view_count, 0) as view_count,       -- 给统计数据默认值
                COALESCE(s.like_count, 0) as like_count,
                COALESCE(s.comment_count, 0) as comment_count
            FROM posts p
            JOIN boards b ON p.board_id = b.id
            JOIN users u ON p.author_id = u.id
            JOIN post_stats s ON p.id = s.post_id
            WHERE p.id = ? AND p.is_deleted = 0
            "#,
            post_id
        ).fetch_optional(pool).await?.ok_or(AppError::NotFound("Post not found".into()))?;

        // Fetch related data
        let tags = sqlx::query_scalar!("SELECT tag_name FROM post_tags WHERE post_id = ?", post_id)
            .fetch_all(pool).await?;

        // 使用 query! 获取原始数据
        let rows = sqlx::query!(
            r#"
            SELECT type as media_type, url, thumbnail_url, meta 
            FROM post_medias 
            WHERE post_id = ?
            "#, 
            post_id
        )
        .fetch_all(pool)
        .await?;

        //  手动将其转换为 MediaItem
        let media: Vec<MediaItem> = rows.into_iter().map(|row| {
            let meta_struct: MediaMeta = row.meta
                .and_then(|v| serde_json::from_value(v).ok())
                .unwrap_or(MediaMeta { 
                    size: None, 
                    width: None, 
                    height: None, 
                    filename: None 
                });

            MediaItem {
                media_type: row.media_type,
                url: row.url,
                thumbnail_url: row.thumbnail_url,
                meta: meta_struct,
            }
        }).collect();

        // Interactions
        let (is_liked, is_collected) = if let Some(uid) = user_id {
            let l = sqlx::query_scalar!("SELECT 1 FROM post_likes WHERE post_id = ? AND user_id = ?", post_id, uid)
                .fetch_optional(pool).await?.is_some();
            let c = sqlx::query_scalar!("SELECT 1 FROM post_collections WHERE post_id = ? AND user_id = ?", post_id, uid)
                .fetch_optional(pool).await?.is_some();
            (l, c)
        } else {
            (false, false)
        };

        Ok(PostDetailVO {
            id: row.id,
            title: row.title,
            content: row.content,
            board_id: row.board_id,
            board_name: row.board_name,
            author: UserLite {
                id: row.u_id,
                student_id: row.student_id,
                name: row.u_name,
                avatar_url: row.avatar_url, 
                college: row.college,
            },
            tags,
            media,
            stats: PostStats { 
                // 修复点 1: 显式转换 i64 -> i32
                view_count: row.view_count as i32, 
                like_count: row.like_count as i32, 
                comment_count: row.comment_count as i32 
            },
            user_interaction: UserInteraction { is_liked, is_collected },
            status: row.status,
            report_count: 0,
            created_at: row.created_at
                .expect("created_at should not be null")
                .and_local_timezone(Local)
                .unwrap(),
            last_replied_at: row.last_replied_at
                .expect("last_replied_at should not be null") 
                .and_local_timezone(Local)
                .unwrap(),
        })
    }

    // -------------------------------------------------------------------------
    // Post - Delete
    // -------------------------------------------------------------------------
    pub async fn delete_post(pool: &DBPool, post_id: &str, user_id: &str) -> Result<(), AppError> {
        let result = sqlx::query!("UPDATE posts SET is_deleted = 1 WHERE id = ? AND author_id = ?", post_id, user_id)
            .execute(pool)
            .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::Forbidden("Cannot delete post or post not found".into()));
        }
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Post - Interactions (Like / Collect)
    // -------------------------------------------------------------------------
    pub async fn toggle_like_post(pool: &DBPool, post_id: &str, user_id: &str, action: &str) -> Result<(i32, bool), AppError> {
        let mut tx = pool.begin().await?;
        
        let is_liked = if action == "like" {
            sqlx::query!("INSERT IGNORE INTO post_likes (post_id, user_id) VALUES (?, ?)", post_id, user_id)
                .execute(&mut *tx).await?;
            sqlx::query!("UPDATE post_stats SET like_count = like_count + 1 WHERE post_id = ?", post_id)
                .execute(&mut *tx).await?;
            true
        } else {
            sqlx::query!("DELETE FROM post_likes WHERE post_id = ? AND user_id = ?", post_id, user_id)
                .execute(&mut *tx).await?;
            sqlx::query!("UPDATE post_stats SET like_count = GREATEST(like_count - 1, 0) WHERE post_id = ?", post_id)
                .execute(&mut *tx).await?;
            false
        };

        let new_count = sqlx::query_scalar!("SELECT like_count FROM post_stats WHERE post_id = ?", post_id)
            .fetch_one(&mut *tx).await?;
        
        tx.commit().await?;
        Ok((new_count.unwrap_or(0), is_liked))
    }

    pub async fn toggle_collect_post(pool: &DBPool, post_id: &str, user_id: &str, action: &str) -> Result<(bool, i64), AppError> {
        if action == "collect" {
             sqlx::query!("INSERT IGNORE INTO post_collections (post_id, user_id) VALUES (?, ?)", post_id, user_id)
                .execute(pool).await?;
        } else {
             sqlx::query!("DELETE FROM post_collections WHERE post_id = ? AND user_id = ?", post_id, user_id)
                .execute(pool).await?;
        }

        let count_res = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM post_collections WHERE post_id = ?", 
            post_id
        )
        .fetch_one(pool)
        .await?;

        let is_collected = action == "collect";
        let total_count = count_res;

        Ok((is_collected, total_count))
    }

    // -------------------------------------------------------------------------
    // Comments - Create
    // -------------------------------------------------------------------------
    pub async fn create_comment(pool: &DBPool, post_id: &str, user_id: &str, req: CreateCommentRequest) -> Result<CommentVO, AppError> {
        let comment_id = Uuid::new_v4().to_string();
        let now = Local::now();

        let mut tx = pool.begin().await?;

        // 1. 插入评论
        sqlx::query!(
            r#"
            INSERT INTO comments (id, post_id, author_id, content, parent_id, created_at, is_deleted)
            VALUES (?, ?, ?, ?, ?, ?, 0)
            "#,
            comment_id, post_id, user_id, req.content, req.reply_to_comment_id, now
        )
        .execute(&mut *tx).await?;

        // 2. 更新帖子统计信息
        sqlx::query!(
            "UPDATE posts SET last_replied_at = ?, updated_at = ? WHERE id = ?",
            now, now, post_id
        )
        .execute(&mut *tx).await?;

        sqlx::query!(
            "UPDATE post_stats SET comment_count = comment_count + 1 WHERE post_id = ?",
            post_id
        )
        .execute(&mut *tx).await?;

        // 3. 查出完整的 CommentVO
        let author = sqlx::query_as!(
            UserLite,
            r#"
            SELECT 
                id, student_id, name, 
                COALESCE(avatar_url, '') as avatar_url, 
                COALESCE(college, '') as college 
            FROM users WHERE id = ?
            "#,
            user_id
        ).fetch_one(&mut *tx).await?;

        let mut reply_to = None;
        if let Some(pid) = &req.reply_to_comment_id {
            reply_to = sqlx::query_as!(
                UserLite,
                r#"
                SELECT 
                    u.id, u.student_id, u.name, 
                    COALESCE(u.avatar_url, '') as avatar_url, 
                    COALESCE(u.college, '') as college 
                FROM comments c 
                JOIN users u ON c.author_id = u.id 
                WHERE c.id = ?
                "#,
                pid
            ).fetch_optional(&mut *tx).await?;
        }

        tx.commit().await?;

        Ok(CommentVO {
            id: comment_id,
            post_id: post_id.to_string(),
            author,
            content: req.content,
            parent_id: req.reply_to_comment_id,
            reply_to,
            stats: CommentStats { like_count: 0 },
            user_interaction: CommentInteraction { is_liked: false },
            created_at: now,
        })
    }

    // -------------------------------------------------------------------------
    // Comments - List
    // -------------------------------------------------------------------------
    pub async fn get_comments(
        pool: &DBPool, 
        post_id: &str, 
        user_id: Option<&str>, 
        _query: CommentQuery 
    ) -> Result<Pagination<CommentVO>, AppError> {
        
        let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
            r#"
            SELECT 
                c.id, c.content, c.created_at, c.parent_id,
                u.id as u_id, u.student_id as u_std_id, u.name as u_name, u.avatar_url as u_avatar, u.college as u_college,
                ru.id as r_u_id, ru.student_id as r_u_std_id, ru.name as r_u_name, ru.avatar_url as r_u_avatar, ru.college as r_u_college,
                (SELECT COUNT(*) FROM comment_likes cl WHERE cl.comment_id = c.id) as like_count,
                EXISTS(SELECT 1 FROM comment_likes cl WHERE cl.comment_id = c.id AND cl.user_id = 
            "#
        );

        match user_id {
            Some(uid) => { qb.push_bind(uid); },
            None => { qb.push("NULL"); }
        }
        qb.push(") as is_liked ");

        qb.push(r#"
            FROM comments c
            JOIN users u ON c.author_id = u.id
            LEFT JOIN comments parent_c ON c.parent_id = parent_c.id
            LEFT JOIN users ru ON parent_c.author_id = ru.id
            WHERE c.post_id = 
        "#);
        qb.push_bind(post_id);
        qb.push(" AND c.is_deleted = 0 ORDER BY c.created_at ASC");

        let rows = qb.build().fetch_all(pool).await?;

        let mut list = Vec::new();
        for row in rows {
            let reply_to = if let Some(rid) = row.get::<Option<String>, _>("r_u_id") {
                 Some(UserLite {
                    id: rid,
                    student_id: row.get("r_u_std_id"),
                    name: row.get("r_u_name"),
                    avatar_url: row.get("r_u_avatar"),
                    college: row.get("r_u_college"),
                 })
            } else {
                None
            };

            list.push(CommentVO {
                id: row.get("id"),
                post_id: post_id.to_string(),
                content: row.get("content"),
                parent_id: row.get("parent_id"),
                author: UserLite {
                    id: row.get("u_id"),
                    student_id: row.get("u_std_id"),
                    name: row.get("u_name"),
                    avatar_url: row.get("u_avatar"),
                    college: row.get("u_college"),
                },
                reply_to,
                stats: CommentStats {
                    like_count: row.get::<i64, _>("like_count") as i32,
                },
                user_interaction: CommentInteraction {
                    is_liked: row.get::<i64, _>("is_liked") == 1,
                },
                created_at: row.get("created_at"),
            });
        }

        let total = list.len() as i64;
        Ok(Pagination {
            list,
            pagination: PageInfo {
                total,
                page: 1,
                page_size: total.max(1),
                pages: 1,
            }
        })
    }

    // -------------------------------------------------------------------------
    // Comments - Delete
    // -------------------------------------------------------------------------
    pub async fn delete_comment(pool: &DBPool, comment_id: &str, user_id: &str) -> Result<(), AppError> {
        let mut tx = pool.begin().await?;

        let comment = sqlx::query!(
            "SELECT post_id, author_id FROM comments WHERE id = ? AND is_deleted = 0",
            comment_id
        ).fetch_optional(&mut *tx).await?
         .ok_or(AppError::NotFound("Comment not found".into()))?;

        if comment.author_id != user_id {
            return Err(AppError::Forbidden("Not the author".into()));
        }

        sqlx::query!("UPDATE comments SET is_deleted = 1 WHERE id = ?", comment_id)
            .execute(&mut *tx).await?;

        sqlx::query!(
            "UPDATE post_stats SET comment_count = GREATEST(comment_count - 1, 0) WHERE post_id = ?",
            comment.post_id
        ).execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Comments - Like
    // -------------------------------------------------------------------------
    pub async fn toggle_like_comment(pool: &DBPool, comment_id: &str, user_id: &str, action: &str) -> Result<(i64, bool), AppError> {
        let mut tx = pool.begin().await?;
        
        let is_liked = if action == "like" {
            sqlx::query!("INSERT IGNORE INTO comment_likes (comment_id, user_id) VALUES (?, ?)", comment_id, user_id)
                .execute(&mut *tx).await?;
            true
        } else {
            sqlx::query!("DELETE FROM comment_likes WHERE comment_id = ? AND user_id = ?", comment_id, user_id)
                .execute(&mut *tx).await?;
            false
        };

        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM comment_likes WHERE comment_id = ?", 
            comment_id
        ).fetch_one(&mut *tx).await?;

        tx.commit().await?;
        Ok((count, is_liked))
    }

    // -------------------------------------------------------------------------
    // Reports & Admin
    // -------------------------------------------------------------------------
    pub async fn create_report(pool: &DBPool, user_id: &str, req: CreateReportRequest) -> Result<String, AppError> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO reports (id, reporter_id, target_type, target_id, reason, description, status) VALUES (?, ?, ?, ?, ?, ?, 'new')",
            id, user_id, req.target_type, req.target_id, req.reason, req.description
        ).execute(pool).await?;
        Ok(id)
    }

    pub async fn admin_list_reports(pool: &DBPool, query: AdminReportQuery) -> Result<Pagination<serde_json::Value>, AppError> {
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;

        let mut qb: QueryBuilder<MySql> = QueryBuilder::new(
            r#"
            SELECT 
                r.id, r.target_type, r.target_id, r.reason, r.status, r.created_at,
                p.content as post_content, 
                pu.name as post_author_name,
                c.content as comment_content,
                cu.name as comment_author_name,
                COUNT(*) OVER(PARTITION BY r.target_type, r.target_id) as target_report_count
            FROM reports r
            LEFT JOIN posts p ON r.target_type = 'post' AND r.target_id = p.id
            LEFT JOIN users pu ON p.author_id = pu.id
            LEFT JOIN comments c ON r.target_type = 'comment' AND r.target_id = c.id
            LEFT JOIN users cu ON c.author_id = cu.id
            WHERE 1=1
            "#
        );

        if let Some(status) = &query.status {
            qb.push(" AND r.status = ");
            qb.push_bind(status);
        }

        if let Some(tt) = &query.target_type {
            qb.push(" AND r.target_type = ");
            qb.push_bind(tt);
        }

        qb.push(" ORDER BY r.created_at DESC LIMIT ");
        qb.push_bind(page_size);
        qb.push(" OFFSET ");
        qb.push_bind(offset);

        let rows = qb.build().fetch_all(pool).await?;
        
        let mut list = Vec::new();
        for row in rows {
            let target_type: String = row.get("target_type");
            
            let (snippet, author) = if target_type == "post" {
                let content: String = row.get::<Option<String>, _>("post_content").unwrap_or_default();
                let name: String = row.get::<Option<String>, _>("post_author_name").unwrap_or_default();
                (content, name)
            } else {
                let content: String = row.get::<Option<String>, _>("comment_content").unwrap_or_default();
                let name: String = row.get::<Option<String>, _>("comment_author_name").unwrap_or_default();
                (content, name)
            };

            let short_snippet = if snippet.chars().count() > 20 { 
                let truncated: String = snippet.chars().take(20).collect();
                format!("{}...", truncated)
            } else {
                snippet
            };

            list.push(serde_json::json!({
                "id": row.get::<String, _>("id"),
                "target_id": row.get::<String, _>("target_id"),
                "target_type": target_type,
                "reason": row.get::<String, _>("reason"),
                "status": row.get::<String, _>("status"),
                "created_at": row.get::<DateTime<Local>, _>("created_at"),
                "snippet": short_snippet,
                "author_name": author,
                "report_count_on_target": row.get::<i64, _>("target_report_count")
            }));
        }

        let total = list.len() as i64;

        Ok(Pagination {
            list,
            pagination: PageInfo { total, page, page_size, pages: 1 }
        })
    }

    pub async fn admin_audit_post(pool: &DBPool, post_id: &str, req: AdminPostStatusRequest) -> Result<(), AppError> {
        let new_status = req.status.to_lowercase();
        
        let mut tx = pool.begin().await?;

        // 1. Update Post Status
        sqlx::query!("UPDATE posts SET status = ? WHERE id = ?", new_status, post_id)
            .execute(&mut *tx).await?;

        // 2. Resolve related reports
        sqlx::query!(
            "UPDATE reports SET status = 'resolved' WHERE target_type = 'post' AND target_id = ?", 
            post_id
        ).execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }
}