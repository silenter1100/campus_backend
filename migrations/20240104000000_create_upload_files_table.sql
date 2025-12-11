-- 创建文件上传记录表（简化版）
CREATE TABLE IF NOT EXISTS upload_files (
    id VARCHAR(36) PRIMARY KEY COMMENT '文件ID (UUID)',
    user_id VARCHAR(50) NOT NULL COMMENT '上传用户ID',
    file_path VARCHAR(500) NOT NULL COMMENT 'OSS对象键 (完整路径，用于删除)',
    file_url VARCHAR(1000) NOT NULL COMMENT '文件访问URL',
    file_size BIGINT NOT NULL COMMENT '文件大小 (字节)',
    is_used BOOLEAN DEFAULT FALSE COMMENT '是否被业务引用（帖子/活动等）',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    
    INDEX idx_user_id (user_id),
    INDEX idx_is_used_created (is_used, created_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='文件上传记录表';
