CREATE DATABASE IF NOT EXISTS forum_db DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
USE forum_db;

-- ==========================================
-- 1. 用户表 (Users)
-- 代码中多次 JOIN 此表，必须存在
-- ==========================================
CREATE TABLE users (
    id VARCHAR(36) NOT NULL PRIMARY KEY COMMENT 'UUID',
    student_id VARCHAR(20) NOT NULL COMMENT '学号',
    name VARCHAR(50) NOT NULL COMMENT '姓名',
    avatar_url VARCHAR(255) DEFAULT '' COMMENT '头像',
    college VARCHAR(50) DEFAULT '' COMMENT '学院',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- 2. 板块表 (Boards)
-- ==========================================
CREATE TABLE boards (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    icon VARCHAR(255) DEFAULT '',
    description TEXT,
    type VARCHAR(20) DEFAULT 'general' COMMENT '对应代码中的 board_type',
    sort_order INT DEFAULT 0,
    is_deleted TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ==========================================
-- 3. 帖子主表 (Posts)
-- ==========================================
CREATE TABLE posts (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    board_id VARCHAR(36) NOT NULL,
    author_id VARCHAR(36) NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL, -- 或者 MEDIUMTEXT，视需求而定
    status VARCHAR(20) DEFAULT 'pending' COMMENT 'approved, pending, rejected, hidden',
    is_deleted TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    last_replied_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_board (board_id),
    INDEX idx_author (author_id),
    INDEX idx_created (created_at),
    INDEX idx_last_replied (last_replied_at)
);

-- ==========================================
-- 4. 帖子统计表 (Post Stats)
-- 将高频更新的计数分离，对应代码逻辑
-- ==========================================
CREATE TABLE post_stats (
    post_id VARCHAR(36) NOT NULL PRIMARY KEY,
    view_count INT DEFAULT 0,
    like_count INT DEFAULT 0,
    comment_count INT DEFAULT 0
);

-- ==========================================
-- 5. 帖子附属信息 (Tags & Medias)
-- ==========================================
CREATE TABLE post_tags (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    post_id VARCHAR(36) NOT NULL,
    tag_name VARCHAR(50) NOT NULL,
    INDEX idx_post (post_id),
    INDEX idx_tag (tag_name)
);

CREATE TABLE post_medias (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    post_id VARCHAR(36) NOT NULL,
    type VARCHAR(20) NOT NULL COMMENT 'image, video',
    url VARCHAR(255) NOT NULL,
    thumbnail_url VARCHAR(255),
    meta JSON COMMENT '存储宽、高、大小等 JSON 数据',
    INDEX idx_post (post_id)
);

-- ==========================================
-- 6. 交互关系表 (Likes & Collections)
-- 代码中使用了 INSERT IGNORE，依赖唯一索引
-- ==========================================
CREATE TABLE post_likes (
    post_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (post_id, user_id) -- 联合主键防止重复点赞
);

CREATE TABLE post_collections (
    post_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (post_id, user_id)
);

-- ==========================================
-- 7. 评论表 (Comments)
-- ==========================================
CREATE TABLE comments (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    post_id VARCHAR(36) NOT NULL,
    author_id VARCHAR(36) NOT NULL,
    parent_id VARCHAR(36) DEFAULT NULL COMMENT '回复某条评论的ID',
    content TEXT NOT NULL,
    is_deleted TINYINT(1) DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_post (post_id),
    INDEX idx_author (author_id)
);

CREATE TABLE comment_likes (
    comment_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (comment_id, user_id)
);

-- ==========================================
-- 8. 举报表 (Reports)
-- ==========================================
CREATE TABLE reports (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    reporter_id VARCHAR(36) NOT NULL,
    target_type VARCHAR(20) NOT NULL COMMENT 'post, comment',
    target_id VARCHAR(36) NOT NULL,
    reason VARCHAR(50) NOT NULL,
    description TEXT,
    status VARCHAR(20) DEFAULT 'new' COMMENT 'new, resolved',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_target (target_type, target_id)
);