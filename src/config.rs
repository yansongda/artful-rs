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
