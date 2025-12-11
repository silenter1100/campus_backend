-- 创建活动模块相关表

-- 活动表
CREATE TABLE IF NOT EXISTS activities (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    cover_url VARCHAR(500),
    activity_type INT NOT NULL DEFAULT 1,  -- 1:讲座, 2:社团, 3:竞赛
    location VARCHAR(255) NOT NULL,
    organizer VARCHAR(255) NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    quota INT NOT NULL DEFAULT 100,
    current_enrollments INT NOT NULL DEFAULT 0,
    need_sign_in BOOLEAN NOT NULL DEFAULT FALSE,
    status INT NOT NULL DEFAULT 1,  -- 1:已发布, 2:已结束, 3:已撤销
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX idx_activities_status ON activities(status);
CREATE INDEX idx_activities_type ON activities(activity_type);
CREATE INDEX idx_activities_start_time ON activities(start_time);

-- 报名记录表
CREATE TABLE IF NOT EXISTS activity_enrollments (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    activity_id VARCHAR(36) NOT NULL,
    user_name VARCHAR(100) NOT NULL,
    student_id VARCHAR(50) NOT NULL,
    major VARCHAR(100) NOT NULL,
    phone_number VARCHAR(20),
    enroll_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    attendance_status INT NOT NULL DEFAULT 1,  -- 1:未签到, 2:已签到
    status INT NOT NULL DEFAULT 1,  -- 1:已报名, 2:已取消
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_enrollment_activity FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
);

-- 创建唯一索引和其他索引
CREATE UNIQUE INDEX uk_enrollment_user_activity ON activity_enrollments(user_id, activity_id);
CREATE INDEX idx_enrollments_activity ON activity_enrollments(activity_id);
CREATE INDEX idx_enrollments_user ON activity_enrollments(user_id);
CREATE INDEX idx_enrollments_status ON activity_enrollments(status);

-- 收藏记录表
CREATE TABLE IF NOT EXISTS activity_collections (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    activity_id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_collection_activity FOREIGN KEY (activity_id) REFERENCES activities(id) ON DELETE CASCADE
);

-- 创建唯一索引和其他索引
CREATE UNIQUE INDEX uk_collection_user_activity ON activity_collections(user_id, activity_id);
CREATE INDEX idx_collections_user ON activity_collections(user_id);
CREATE INDEX idx_collections_activity ON activity_collections(activity_id);

-- 插入测试数据
INSERT IGNORE INTO activities (id, title, content, cover_url, activity_type, location, organizer, start_time, end_time, quota, status)
VALUES 
    ('550e8400-e29b-41d4-a716-446655440001', 'AI Lecture', 'AI trends and applications', 'https://example.com/cover1.jpg', 1, 'Library Hall', 'CS Department', '2024-02-01 14:00:00', '2024-02-01 16:00:00', 200, 1),
    ('550e8400-e29b-41d4-a716-446655440002', 'Basketball Match', 'Spring basketball league', 'https://example.com/cover2.jpg', 2, 'Gymnasium', 'Sports Department', '2024-02-15 09:00:00', '2024-02-15 18:00:00', 100, 1),
    ('550e8400-e29b-41d4-a716-446655440003', 'Programming Contest', 'ACM programming contest', 'https://example.com/cover3.jpg', 3, 'Lab Building', 'CS Department', '2024-03-01 13:00:00', '2024-03-01 17:00:00', 50, 1);
