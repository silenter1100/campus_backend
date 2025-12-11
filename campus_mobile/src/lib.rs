use std::sync::Arc;
use aho_corasick::AhoCorasick;
use uniffi;
use chrono::{DateTime, Local, TimeZone, Utc};

#[derive(uniffi::Object)]
pub struct ContentSecurity {
    matcher: AhoCorasick,
}

#[uniffi::export]
impl ContentSecurity {

    #[uniffi::constructor]
    pub fn new(mut external_words: Vec<String>) -> Arc<Self> {
        // ✨ 魔法在这里！
        // include_str! 会在编译时把 words.txt 的内容读成一个巨大的字符串
        let file_content = include_str!("words.txt");

        // 我们把这个大字符串按“换行符”切开，处理一下，变成 Vec<String>
        let mut my_hardcoded_list: Vec<String> = file_content
            .lines() // 按行切割
            .map(|line| line.trim().to_string()) // 去掉空格并转成 String
            .filter(|line| !line.is_empty()) // 去掉空行
            .collect();

        // 合并：外部传入的 + 我们文件里读取的
        external_words.append(&mut my_hardcoded_list);

        // 兜底防止为空
        if external_words.is_empty() {
            external_words.push("impossible_placeholder".to_string());
        }

        let matcher = AhoCorasick::new(external_words).unwrap();
        Arc::new(Self { matcher })
    }

    pub fn has_sensitive_word(&self, text: &str) -> bool {
        self.matcher.find(text).is_some()
    }

    pub fn censor_text(&self, text: &str) -> String {
        let mut result = String::new();
        self.matcher.replace_all_with(text, &mut result, |_mat, _part, dst| {
            dst.push_str("***");
            true
        });
        result
    }
}

#[derive(uniffi::Object)]
pub struct TimeFormatter;

#[uniffi::export]
impl TimeFormatter {

    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }

    /// 将 Unix 时间戳 (秒) 转换为相对时间字符串
    /// 例如：1698765432 -> "5分钟前"
    pub fn format_relative_time(&self, timestamp_secs: i64) -> String {
        // 1. 获取当前时间
        let now = Local::now();

        // 2. 将传入的时间戳转换为本地时间
        // 如果时间戳无效（比如0或负数），直接显示“未知时间”
        let event_time = match Local.timestamp_opt(timestamp_secs, 0).single() {
            Some(t) => t,
            None => return "未知时间".to_string(),
        };

        // 3. 计算时间差 (Duration)
        let diff = now.signed_duration_since(event_time);

        // 4. 逻辑判断
        // 如果是未来的时间（服务器时间可能比手机快一点点），统称“刚刚”
        if diff.num_seconds() < 0 {
            return "刚刚".to_string();
        }

        let seconds = diff.num_seconds();

        match seconds {
            0..=60 => "刚刚".to_string(),
            61..=3600 => format!("{}分钟前", seconds / 60),
            3601..=86400 => format!("{}小时前", seconds / 3600),
            _ => {
                // 超过24小时，检查是不是昨天或前天
                let days = diff.num_days();
                match days {
                    1 => "昨天".to_string(),
                    2 => "前天".to_string(),
                    3..=30 => format!("{}天前", days),
                    _ => {
                        // 超过30天，直接显示具体日期 (例如: 2023-12-12)
                        event_time.format("%Y-%m-%d").to_string()
                    }
                }
            }
        }
    }
}

// 别忘了底部的测试代码也要改一下，测一下 txt 里的词
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_loaded_words() {
        let filter = ContentSecurity::new(vec![]);

        // 测试 txt 文件里的词
        assert!(filter.has_sensitive_word("千万不要碰赌博和六合彩"));
        assert_eq!(filter.censor_text("比特币是blockchain技术"), "***是***技术");

        // 打印一下看看效果
        println!("测试通过：成功识别了 words.txt 里的词汇");
    }
}

#[test]
fn test_time_formatter() {
    let formatter = TimeFormatter::new();
    let now = Local::now().timestamp();

    // 测试：刚刚
    assert_eq!(formatter.format_relative_time(now - 10), "刚刚");

    // 测试：5分钟前
    assert_eq!(formatter.format_relative_time(now - 300), "5分钟前");

    // 测试：2小时前
    assert_eq!(formatter.format_relative_time(now - 7200), "2小时前");

    // 测试：很久以前 (比如一年前)
    let old_time = now - 31536000;
    // 这里断言具体的格式可能受当前日期影响，我们只打印看看
    println!("很久以前: {}", formatter.format_relative_time(old_time));
}

uniffi::setup_scaffolding!();