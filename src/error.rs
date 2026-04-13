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

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, ArtfulError>;
