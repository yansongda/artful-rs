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
//! - `CollectionDirection` - 解析为 JSON（默认）
//! - `ResponseDirection` - 返回原始 Response
//! - `NoHttpRequestDirection` - 不发起 HTTP 请求
//! - `OriginResponseDirection` - 返回 Rocket（调试用）
//! - `Custom` - 自定义解析器

use std::sync::Arc;

#[async_trait::async_trait]
pub trait Direction: Send + Sync + std::fmt::Debug {
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination>;
}

#[derive(Debug, Clone)]
pub struct CollectionDirection;

#[async_trait::async_trait]
impl Direction for CollectionDirection {
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination> {
        let value = serde_json::to_value(&rocket.payload)?;
        Ok(Destination::Collection(value))
    }
}

#[derive(Debug, Clone)]
pub enum DirectionKind {
    CollectionDirection,
    ResponseDirection,
    NoHttpRequestDirection,
    OriginResponseDirection,
    Custom(Arc<dyn Direction>),
}

#[derive(Default)]
pub enum Destination {
    Collection(serde_json::Value),
    Response(reqwest::Response),
    Rocket(Box<crate::Rocket>),
    #[default]
    None,
}

impl std::fmt::Debug for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Destination::Collection(v) => f.debug_tuple("Collection").field(v).finish(),
            Destination::Response(_) => f
                .debug_tuple("Response")
                .field(&"<reqwest::Response>")
                .finish(),
            Destination::Rocket(r) => f.debug_tuple("Rocket").field(r).finish(),
            Destination::None => write!(f, "None"),
        }
    }
}
