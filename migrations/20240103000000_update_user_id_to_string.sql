-- 更新用户ID类型从BIGINT到VARCHAR(36)以支持UUID

-- 首先备份现有数据（如果有的话）
-- 注意：这个迁移假设现有的user_id都是数字，将转换为字符串

-- 更新 schedule_items 表的 user_id 字段
ALTER TABLE schedule_items 
MODIFY COLUMN user_id VARCHAR(36) NOT NULL COMMENT '用户ID（UUID）';

-- 更新 public_courses 表的 teacher_id 字段
ALTER TABLE public_courses 
MODIFY COLUMN teacher_id VARCHAR(36) COMMENT '教师工号';

-- 如果需要，可以在这里添加数据转换逻辑
-- 例如：将现有的数字ID转换为字符串格式
-- UPDATE schedule_items SET user_id = CAST(user_id AS CHAR);
-- UPDATE public_courses SET teacher_id = CAST(teacher_id AS CHAR) WHERE teacher_id IS NOT NULL;