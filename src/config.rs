//! 框架配置模块
//!
//! 定义框架级别的配置，包括：
//! - [`Config`] - 框架主配置
//! - [`LoggerConfig`] - 日志配置
//!
//! 注意：HTTP 相关配置通过 [`RocketConfig`] 设置，而非此模块。

use crate::direction::DirectionKind;

#[derive(Debug, Clone)]
pub struct Config {
    pub logger: LoggerConfig,
    pub default_direction: DirectionKind,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logger: LoggerConfig::default(),
            default_direction: DirectionKind::CollectionDirection,
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
