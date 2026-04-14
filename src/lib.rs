//! Artful-Rs - Api RequesT Framework U Like
//!
//! 基于洋葱模型的 Rust HTTP 客户端框架。
//!
//! # 核心概念
//!
//! - [`Rocket`] - 请求载体，携带整个请求生命周期的数据
//! - [`Plugin`] - 插件 trait，洋葱模型的核心
//! - [`FlowCtrl`] - 流向控制器，管理插件执行顺序
//! - [`Artful`] - 框架主入口

pub mod direction;
pub mod error;
pub mod directions;
pub use directions::JsonDirection;
pub mod artful;
pub mod config;
pub mod flow_ctrl;
pub mod http;
pub mod packer;
pub mod packers;
pub mod plugin;
pub mod plugins;
pub mod rocket;
pub mod shortcut;

pub use artful::Artful;
pub use config::{Config, LoggerConfig};
pub use direction::{Destination, Direction, DirectionKind};
pub use error::{ArtfulError, Result};
pub use flow_ctrl::{FlowCtrl, Next};
pub use http::get_client;
pub use packer::Packer;
pub use packers::JsonPacker;
pub use plugin::Plugin;
pub use plugins::{AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, StartPlugin};
pub use rocket::{HttpOptions, Rocket, RocketConfig};
pub use shortcut::Shortcut;
