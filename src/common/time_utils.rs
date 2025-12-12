// src/common/time_utils.rs

use chrono::{DateTime, Utc};

/// 将 DateTime<Utc> 转换为 ISO8601 字符串格式（与 serde 默认格式一致）
pub fn datetime_to_iso8601(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// 将 ISO8601 字符串格式转换为 DateTime<Utc>
pub fn iso8601_to_datetime(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(s).map(|dt| dt.with_timezone(&Utc))
}

/// 将可选的 DateTime<Utc> 转换为可选的 ISO8601 字符串
pub fn optional_datetime_to_iso8601(dt: &Option<DateTime<Utc>>) -> Option<String> {
    dt.as_ref().map(datetime_to_iso8601)
}

/// 将可选的 ISO8601 字符串转换为可选的 DateTime<Utc>
pub fn optional_iso8601_to_datetime(s: &Option<String>) -> Result<Option<DateTime<Utc>>, chrono::ParseError> {
    match s {
        Some(s) => iso8601_to_datetime(s).map(Some),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_datetime_conversion() {
        let dt = Utc.with_ymd_and_hms(2024, 2, 1, 14, 0, 0).unwrap();
        let iso_string = datetime_to_iso8601(&dt);
        assert_eq!(iso_string, "2024-02-01T14:00:00Z");
        
        let parsed_dt = iso8601_to_datetime(&iso_string).unwrap();
        assert_eq!(dt, parsed_dt);
    }

    #[test]
    fn test_optional_conversion() {
        let dt = Some(Utc.with_ymd_and_hms(2024, 2, 1, 14, 0, 0).unwrap());
        let iso_string = optional_datetime_to_iso8601(&dt);
        assert_eq!(iso_string, Some("2024-02-01T14:00:00Z".to_string()));
        
        let parsed_dt = optional_iso8601_to_datetime(&iso_string).unwrap();
        assert_eq!(dt, parsed_dt);
    }

    #[test]
    fn test_serde_compatibility() {
        use serde_json;
        
        let dt = Utc.with_ymd_and_hms(2024, 2, 1, 14, 0, 0).unwrap();
        
        // 测试 serde 默认序列化格式
        let serde_serialized = serde_json::to_string(&dt).unwrap();
        println!("Serde 序列化: {}", serde_serialized);
        
        // 测试我们的转换函数
        let our_format = datetime_to_iso8601(&dt);
        println!("我们的格式: {}", our_format);
        
        // 验证格式兼容性（去掉引号比较）
        let serde_without_quotes = serde_serialized.trim_matches('"');
        assert_eq!(our_format, serde_without_quotes);
    }
}