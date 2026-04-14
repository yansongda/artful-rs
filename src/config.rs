//! 框架配置模块
//!
//! 定义框架级别的配置，包括：
//! - [`Config`] - 框架主配置
//! - [`LoggerConfig`] - 日志配置

use serde_json::Value;
use std::collections::HashMap;

use crate::rocket::HttpOptions;

/// 框架全局配置
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// 日志配置
    pub logger: LoggerConfig,

    /// HTTP 默认选项
    pub http: HttpOptions,

    /// 扩展配置：支持任意渠道/模块参数
    ///
    /// # 示例
    ///
    /// ```rust
    /// use artisan::Config;
    /// use serde_json::json;
    /// use std::collections::HashMap;
    ///
    /// let mut extra = HashMap::new();
    /// extra.insert("name".to_string(), json!("yansongda"));
    /// extra.insert("http".to_string(), json!({"timeout": 5.0}));
    ///
    /// let config = Config {
    ///     extra,
    ///     ..Default::default()
    /// };
    /// ```
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub enable: bool,
    pub level: String,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            enable: true,
            level: "info".to_string(),
        }
    }
}
