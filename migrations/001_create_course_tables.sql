USE campus_db;
-- 创建学期表
CREATE TABLE IF NOT EXISTS semesters (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL COMMENT '学期名称，如"2024-2025学年第一学期"',
    start_date VARCHAR(10) NOT NULL COMMENT '学期开始日期 YYYY-MM-DD',
    end_date VARCHAR(10) NOT NULL COMMENT '学期结束日期 YYYY-MM-DD',
    is_current BOOLEAN NOT NULL DEFAULT FALSE COMMENT '是否当前学期',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_is_current (is_current)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='学期信息表';

-- 创建全校课程表
CREATE TABLE IF NOT EXISTS public_courses (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    semester_id BIGINT NOT NULL COMMENT '学期ID',
    course_name VARCHAR(255) NOT NULL COMMENT '课程名称',
    teacher_name VARCHAR(100) NOT NULL COMMENT '教师姓名',
    teacher_id BIGINT COMMENT '教师工号',
    location VARCHAR(100) NOT NULL COMMENT '上课地点',
    day_of_week INT NOT NULL COMMENT '星期几 1-7',
    start_section INT NOT NULL COMMENT '开始节次',
    end_section INT NOT NULL COMMENT '结束节次',
    weeks_range JSON NOT NULL COMMENT '周次范围，如 [1,2,3,4,5]',
    type VARCHAR(20) NOT NULL COMMENT '课程类型：compulsory(必修)/elective(选修)',
    credits INT COMMENT '学分',
    description TEXT COMMENT '课程描述',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_semester (semester_id),
    INDEX idx_course_name (course_name),
    INDEX idx_teacher_name (teacher_name),
    INDEX idx_day_time (day_of_week, start_section, end_section),
    FOREIGN KEY (semester_id) REFERENCES semesters(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='全校课程表';

-- 创建用户课表项表
CREATE TABLE IF NOT EXISTS schedule_items (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id BIGINT NOT NULL COMMENT '用户ID',
    semester_id BIGINT NOT NULL COMMENT '学期ID',
    source_id BIGINT COMMENT '关联的全校课程ID，自定义课程为NULL',
    course_name VARCHAR(255) NOT NULL COMMENT '课程名称',
    teacher_name VARCHAR(100) COMMENT '教师姓名',
    location VARCHAR(100) COMMENT '上课地点',
    day_of_week INT NOT NULL COMMENT '星期几 1-7',
    start_section INT NOT NULL COMMENT '开始节次',
    end_section INT NOT NULL COMMENT '结束节次',
    weeks_range JSON NOT NULL COMMENT '周次范围，如 [1,2,3,4,5]',
    type VARCHAR(20) COMMENT '课程类型：compulsory(必修)/elective(选修)',
    credits INT COMMENT '学分',
    description TEXT COMMENT '课程描述',
    color_hex VARCHAR(7) NOT NULL DEFAULT '#9E9E9E' COMMENT '课程颜色 #RRGGBB',
    is_custom BOOLEAN NOT NULL DEFAULT FALSE COMMENT '是否用户自定义课程',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_user_semester (user_id, semester_id),
    INDEX idx_source (source_id),
    INDEX idx_day_time (day_of_week, start_section, end_section),
    FOREIGN KEY (semester_id) REFERENCES semesters(id) ON DELETE CASCADE,
    FOREIGN KEY (source_id) REFERENCES public_courses(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户课表项表';

-- 插入示例学期数据
INSERT INTO semesters (name, start_date, end_date, is_current) VALUES
('2024-2025学年第一学期', '2024-09-01', '2025-01-15', TRUE),
('2023-2024学年第二学期', '2024-02-20', '2024-07-10', FALSE);

-- 插入示例全校课程数据
INSERT INTO public_courses (semester_id, course_name, teacher_name, teacher_id, location, day_of_week, start_section, end_section, weeks_range, type, credits, description) VALUES
(1, '高等数学A', '张教授', 10001, '教学楼A-101', 1, 1, 2, '[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]', 'compulsory', 4, '高等数学基础课程'),
(1, '大学英语', '李老师', 10002, '教学楼B-201', 2, 3, 4, '[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]', 'compulsory', 3, '大学英语综合课程'),
(1, '计算机网络', '王教授', 10003, '实验楼C-301', 3, 5, 7, '[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]', 'compulsory', 3, '计算机网络原理与应用'),
(1, '数据结构', '赵老师', 10004, '教学楼A-202', 4, 1, 3, '[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]', 'compulsory', 4, '数据结构与算法'),
(1, '体育选修-篮球', '刘教练', 10005, '体育馆', 5, 8, 9, '[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]', 'elective', 1, '篮球基础与提高');
