// Mock 测试说明
// 
// 由于 Rust 的生命周期系统，mockall 与 &str 参数不兼容。
// 有以下几种解决方案：
//
// 1. 使用真实数据库测试（推荐）- 见 course_service_test.rs
// 2. 将所有 &str 改为 String（性能损失）
// 3. 使用其他 Mock 库（如 mockito）
// 4. 手动实现 Mock 结构体
//
// 目前项目使用方案 1：真实数据库测试
// 这种方式更接近集成测试，能发现更多实际问题

#[cfg(test)]
mod tests {
    #[test]
    fn mock_test_explanation() {
        // 这个文件保留作为说明
        // 实际测试请查看：
        // - tests/course_service_test.rs (Service 层测试)
        // - tests/course_api_test.rs (API 集成测试)
        assert!(true);
    }
}
