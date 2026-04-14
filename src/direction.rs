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
//! - `JsonDirection` - 解析为 JSON（默认）
//! - `ResponseDirection` - 返回原始 Response
//! - `NoHttpRequestDirection` - 不发起 HTTP 请求
//! - `Custom` - 自定义解析器

use std::sync::Arc;

#[async_trait::async_trait]
pub trait Direction: Send + Sync + std::fmt::Debug {
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination>;
}

/// JSON 解析方向
#[derive(Debug, Clone)]
pub struct JsonDirection;

#[async_trait::async_trait]
impl Direction for JsonDirection {
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination> {
        if let Some(response) = rocket.destination_origin.take() {
            let text = response.text().await
                .map_err(crate::error::ArtfulError::RequestFailed)?;
            let json: serde_json::Value = serde_json::from_str(&text)?;
            Ok(Destination::Json(json))
        } else {
            Err(crate::error::ArtfulError::MissingResponse)
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirectionKind {
    JsonDirection,
    ResponseDirection,
    NoHttpRequestDirection,
    Custom(Arc<dyn Direction>),
}

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
