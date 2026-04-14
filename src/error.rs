//! 错误类型定义
//!
//! 定义框架中所有可能出现的错误类型，包括：
//! - HTTP 请求错误（RequestFailed, Timeout, NetworkError）
//! - 序列化错误（JsonSerializeError, JsonDeserializeError）
//! - 插件错误（PluginExecutionError）
//! - 参数错误（MissingParameter, InvalidParameter）
//! - 响应解析错误（DirectionParseError）

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtfulError {
    #[error("HTTP 请求失败: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("请求超时")]
    Timeout,

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("无效的 URL: {0}")]
    InvalidUrl(String),

    #[error("JSON 序列化失败: {0}")]
    JsonSerializeError(#[from] serde_json::Error),

    #[error("JSON 反序列化失败: {message}")]
    JsonDeserializeError { message: String },

    #[error("插件执行错误: {plugin_name} - {message}")]
    PluginExecutionError {
        plugin_name: String,
        message: String,
    },

    #[error("缺少必要参数: {0}")]
    MissingParameter(String),

    #[error("参数无效: {param} - {message}")]
    InvalidParameter { param: String, message: String },

    #[error("响应解析失败: {0}")]
    DirectionParseError(String),

    #[error("缺少 HTTP Request")]
    MissingRequest,

    #[error("缺少 HTTP Response")]
    MissingResponse,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, ArtfulError>;
