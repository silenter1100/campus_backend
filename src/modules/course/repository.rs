// Repository 层暂时不使用 mockall，因为它与 &str 生命周期不兼容
// 如果需要 Mock 测试，可以使用真实数据库测试或重构为使用 String

// Repository 层 - 简化版本，直接在 service 层使用 SQLx
// 这个文件保留作为参考，实际查询在 service.rs 中

use sqlx::MySqlPool;

use crate::common::AppError;

/// 课程仓储（简化版）
/// 注意：实际项目中，数据库查询直接在 service.rs 中进行
/// 这个文件保留作为架构参考
pub struct CourseRepository {
    _pool: MySqlPool,
}

impl CourseRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { _pool: pool }
    }
}

