//! 框架配置模块
//!
//! 定义框架级别的配置，包括：
//! - [`Config`] - 框架主配置
//! - [`LoggerConfig`] - 日志配置

use serde_json::Value;
use std::collections::HashMap;

use crate::rocket::HttpOptions;

#[derive(Debug, Clone)]
pub struct Config {
    /// 是否强制覆盖已存在的配置
    pub _force: bool,

    pub logger: LoggerConfig,
    pub http: HttpOptions,

    /// 扩展配置：支持任意渠道/模块参数
    ///
    /// # 示例
    ///
    /// ```rust
    /// use artful::Config;
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

impl Default for Config {
    fn default() -> Self {
        Self {
            _force: false,
            logger: LoggerConfig::default(),
            http: HttpOptions::default(),
            extra: HashMap::new(),
        }
    }
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
