-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(36) PRIMARY KEY COMMENT '用户ID（UUID）',
    student_id VARCHAR(20) NOT NULL UNIQUE COMMENT '学号',
    username VARCHAR(100) NOT NULL COMMENT '用户名',
    password VARCHAR(255) NOT NULL COMMENT '密码哈希',
    gender VARCHAR(10) DEFAULT '' COMMENT '性别',
    college VARCHAR(100) NOT NULL COMMENT '学院',
    major VARCHAR(100) NOT NULL COMMENT '专业',
    class_name VARCHAR(50) DEFAULT '' COMMENT '班级',
    phone VARCHAR(20) NOT NULL COMMENT '电话',
    email VARCHAR(100) DEFAULT '' COMMENT '邮箱',
    avatar_url VARCHAR(255) DEFAULT '' COMMENT '头像URL',
    role VARCHAR(20) DEFAULT 'student' COMMENT '角色：student/admin',
    wechat_id VARCHAR(50) DEFAULT '' COMMENT '微信号',
    bio TEXT COMMENT '个人简介',
    collection_count INT DEFAULT 0 COMMENT '收藏数',
    forum_activity_score BIGINT DEFAULT 0 COMMENT '论坛活跃度',
    weekly_course_count BIGINT DEFAULT 0 COMMENT '本周课时数',
    grade VARCHAR(10) DEFAULT '' COMMENT '年级',
    setting_privacy_course VARCHAR(20) DEFAULT 'public' COMMENT '课表隐私设置',
    setting_notification_switch BOOLEAN DEFAULT TRUE COMMENT '通知开关',
    setting_theme VARCHAR(20) DEFAULT 'light' COMMENT '主题设置',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    last_login_at TIMESTAMP NULL COMMENT '最后登录时间',
    
    INDEX idx_student_id (student_id),
    INDEX idx_username (username),
    INDEX idx_college_major (college, major)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户信息表';

-- 插入测试用户数据
INSERT INTO users (
    id, student_id, username, password, college, major, phone, role
) VALUES 
(
    '1', 
    '2021001001', 
    '张三', 
    'password123',  -- 实际应用中应该是哈希后的密码
    '计算机学院', 
    '计算机科学与技术', 
    '13800138001',
    'student'
),
(
    '2', 
    '2021001002', 
    '李四', 
    'password123', 
    '计算机学院', 
    '软件工程', 
    '13800138002',
    'student'
),
(
    '3', 
    '2021001003', 
    '王五', 
    'password123', 
    '电子工程学院', 
    '电子信息工程', 
    '13800138003',
    'student'
),
(
    '999', 
    'admin001', 
    '管理员', 
    'admin123', 
    '管理部门', 
    '系统管理', 
    '13800138999',
    'admin'
);