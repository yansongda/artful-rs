//! 响应解析模块
//!
//! 定义响应解析的抽象接口和解析策略。
//!
//! # 核心类型
//!
//! - [`Direction`] trait - 响应解析器接口
//! - [`DirectionKind`] - 解析策略枚举
//! - [`Destination`] - 解析结果类型
//!
//! # 解析策略
//!
//! - `Json` - 解析为 JSON（默认）
//! - `Response` - 返回原始 Response
//! - `NoRequest` - 不发起 HTTP 请求
//! - `Custom` - 自定义解析器

use std::sync::Arc;

/// 响应解析器 trait
#[async_trait::async_trait]
pub trait Direction: Send + Sync + std::fmt::Debug {
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination>;
}

/// 解析策略枚举
#[derive(Debug, Clone)]
pub enum DirectionKind {
    Json,
    Response,
    NoRequest,
    Custom(Arc<dyn Direction>),
}

/// 解析结果类型
#[derive(Default)]
pub enum Destination {
    Json(serde_json::Value),
    Response(reqwest::Response),
    #[default]
    None,
}

impl std::fmt::Debug for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Destination::Json(v) => f.debug_tuple("Json").field(v).finish(),
            Destination::Response(_) => f
                .debug_tuple("Response")
                .field(&"<reqwest::Response>")
                .finish(),
            Destination::None => write!(f, "None"),
        }
    }
}

impl std::fmt::Display for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Destination::Json(v) => write!(f, "{v}"),
            Destination::Response(_) => write!(f, "<HTTP Response>"),
            Destination::None => write!(f, "None"),
        }
    }
}

impl From<serde_json::Value> for Destination {
    fn from(value: serde_json::Value) -> Self {
        Destination::Json(value)
    }
}
