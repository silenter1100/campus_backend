USE forum_db;

-- =================================================================================
-- 1. 初始化用户 (Users)
-- 包含：你的测试账号、活跃用户、普通用户、潜在违规用户
-- =================================================================================
INSERT INTO users (id, student_id, name, avatar_url, college) VALUES
('test-user-001', '20230001', '张三', 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhangSan', '软件学院'),
('test-user-002', '20230002', '李四', 'https://api.dicebear.com/7.x/avataaars/svg?seed=LiSi', '设计创意学院'),
('test-user-003', '20230003', '王五', 'https://api.dicebear.com/7.x/avataaars/svg?seed=WangWu', '土木工程学院'),
('test-user-004', '20230004', '赵六', 'https://api.dicebear.com/7.x/avataaars/svg?seed=ZhaoLiu', '汽车学院'),
('test-user-bad', '20239999', '广告哥', 'https://api.dicebear.com/7.x/avataaars/svg?seed=Spam', '未知学院');

-- =================================================================================
-- 2. 初始化板块 (Boards)
-- =================================================================================
INSERT INTO boards (id, name, icon, description, type, sort_order) VALUES
('board-life', '校园生活', '', '分享吃喝玩乐、嘉定/四平校区生活指南', 'general', 1),
('board-study', '学术交流', '', '选课攻略、作业讨论、考研保研经验分享', 'study', 2),
('board-market', '跳蚤市场', '', '二手书、数码产品、闲置物品交易', 'market', 3),
('board-confession', '表白墙', '', '大声说出你的爱 (匿名)', 'general', 4);

-- =================================================================================
-- 3. 初始化帖子 (Posts)
-- 场景：包含热门贴、普通贴、带图贴、待审核贴、已拒绝贴
-- =================================================================================
INSERT INTO posts (id, board_id, author_id, title, content, status, created_at, last_replied_at) VALUES
-- 帖子1：张三发在生活版，带图，很火
('post-001', 'board-life', 'test-user-001', '嘉定校区新开的食堂太好吃了！', '强烈推荐二楼的那个烧腊饭，量大管饱，只要15块钱！附图证明。', 'approved', DATE_SUB(NOW(), INTERVAL 2 DAY), NOW()),

-- 帖子2：李四发在市场版，纯文本
('post-002', 'board-market', 'test-user-002', '【出】闲置 Switch 游戏卡带', '塞尔达传说旷野之息，200元出，同济北楼面交。卡带保存完好，箱说全。', 'approved', DATE_SUB(NOW(), INTERVAL 1 DAY), DATE_SUB(NOW(), INTERVAL 1 DAY)),

-- 帖子3：王五发在学习版，求助
('post-003', 'board-study', 'test-user-003', '关于 Rust 语言的所有权机制', '在写大作业，Borrow Checker 报错太折磨了，求学长指点一下声明周期的用法。', 'approved', DATE_SUB(NOW(), INTERVAL 5 HOUR), NOW()),

-- 帖子4：张三发的，还在审核中 (Pending) -> 用于测试普通列表看不到它
('post-004', 'board-life', 'test-user-001', '测试一下审核机制', '这条帖子应该处于 Pending 状态。', 'pending', NOW(), NOW()),

-- 帖子5：广告哥发的违规贴 (Rejected) -> 用于测试管理员功能
('post-005', 'board-market', 'test-user-bad', '兼职刷单，日入过万', '加V信：xxxxxx，懂的来。', 'rejected', DATE_SUB(NOW(), INTERVAL 10 DAY), DATE_SUB(NOW(), INTERVAL 10 DAY));

-- =================================================================================
-- 4. 初始化统计数据 (Post Stats)
-- ⚠️ 必须与 Posts 表一一对应，否则 JOIN 会漏数据
-- =================================================================================
INSERT INTO post_stats (post_id, view_count, like_count, comment_count) VALUES
('post-001', 1024, 5, 3), -- 热门贴
('post-002', 45, 1, 1),   -- 市场贴
('post-003', 128, 2, 2),  -- 学习贴
('post-004', 0, 0, 0),    -- 审核贴
('post-005', 5, 0, 0);    -- 违规贴

-- =================================================================================
-- 5. 初始化标签 (Tags)
-- =================================================================================
INSERT INTO post_tags (post_id, tag_name) VALUES
('post-001', '美食'), ('post-001', '嘉定'), ('post-001', '探店'),
('post-002', '二手'), ('post-002', 'Switch'), ('post-002', '面交'),
('post-003', 'Rust'), ('post-003', '求助'), ('post-003', '后端');

-- =================================================================================
-- 6. 初始化媒体 (Medias)
-- JSON 格式存储元数据
-- =================================================================================
INSERT INTO post_medias (post_id, type, url, thumbnail_url, meta) VALUES
-- 帖子1的美食图
('post-001', 'image', 'https://images.unsplash.com/photo-1504674900247-0877df9cc836', 'https://images.unsplash.com/photo-1504674900247-0877df9cc836?w=200', '{"width": 1024, "height": 768, "size": "2MB"}'),
-- 帖子2的游戏卡带图
('post-002', 'image', 'https://images.unsplash.com/photo-1578303512597-81e6cc155b3e', 'https://images.unsplash.com/photo-1578303512597-81e6cc155b3e?w=200', '{"width": 800, "height": 600, "size": "1.5MB"}');

-- =================================================================================
-- 7. 初始化评论 (Comments) - 构建一个嵌套回复树
-- 结构：
-- Post-001
--   ├── Comment-1 (李四): 确实好吃！
--   │     └── Comment-2 (张三回复李四): 哈哈，英雄所见略同。
--   └── Comment-3 (王五): 有没有忌口的？
-- =================================================================================
INSERT INTO comments (id, post_id, author_id, content, parent_id, created_at) VALUES
('c-001', 'post-001', 'test-user-002', '确实好吃！我也经常去吃那家。', NULL, DATE_SUB(NOW(), INTERVAL 20 HOUR)),
('c-002', 'post-001', 'test-user-001', '哈哈，英雄所见略同。下次一起拼桌？', 'c-001', DATE_SUB(NOW(), INTERVAL 19 HOUR)),
('c-003', 'post-001', 'test-user-003', '那里排队人多吗？有没有忌口的推荐？', NULL, DATE_SUB(NOW(), INTERVAL 18 HOUR)),

-- Post-002 的评论
('c-004', 'post-002', 'test-user-003', '150包邮出不出？学生党没钱。', NULL, NOW()),

-- Post-003 的评论
('c-005', 'post-003', 'test-user-002', '去看官方文档的第4章，讲得很详细。', NULL, NOW()),
('c-006', 'post-003', 'test-user-001', '生命周期其实就是作用域的延伸，多写写就懂了。', NULL, NOW());

-- =================================================================================
-- 8. 初始化点赞与收藏 (Likes & Collections)
-- =================================================================================
-- 谁点赞了 Post-001
INSERT INTO post_likes (post_id, user_id) VALUES 
('post-001', 'test-user-002'), 
('post-001', 'test-user-003'),
('post-001', 'test-user-004');

-- 谁收藏了 Post-003 (Rust教程)
INSERT INTO post_collections (post_id, user_id) VALUES 
('post-003', 'test-user-001'),
('post-003', 'test-user-004');

-- 谁点赞了 评论 c-001
INSERT INTO comment_likes (comment_id, user_id) VALUES
('c-001', 'test-user-001');

-- =================================================================================
-- 9. 初始化举报记录 (Reports)
-- =================================================================================
INSERT INTO reports (id, reporter_id, target_type, target_id, reason, description, status, created_at) VALUES
-- 举报那个违规广告贴
('rep-001', 'test-user-001', 'post', 'post-005', 'spam', '发布虚假兼职广告，建议封禁', 'resolved', DATE_SUB(NOW(), INTERVAL 9 DAY)),
-- 举报王五的砍价评论（测试用）
('rep-002', 'test-user-002', 'comment', 'c-004', 'harassment', '恶意砍价，扰乱市场秩序', 'new', NOW());