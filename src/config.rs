//! 框架配置模块
//!
//! 定义框架级别的配置，包括：
//! - [`Config`] - 框架主配置
//! - [`LoggerConfig`] - 日志配置

use crate::direction::DirectionKind;

#[derive(Debug, Clone)]
pub struct Config {
    /// 是否强制覆盖已存在的配置
    pub _force: bool,

    pub logger: LoggerConfig,
    pub default_direction: DirectionKind,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            _force: false,
            logger: LoggerConfig::default(),
            default_direction: DirectionKind::JsonDirection,
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
