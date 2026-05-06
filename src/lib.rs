//! Artisan - Api `RequesT` Framework U Like
//!
//! 基于洋葱模型的 Rust HTTP 客端框架。
//!
//! # 核心概念
//!
//! - [`Rocket`] - 请求载体，携带整个请求生命周期的数据
//! - [`Plugin`] - 插件 trait，洋葱模型的核心
//! - [`FlowCtrl`] - 流向控制器，管理插件执行顺序
//! - [`Artful`] - 框架主入口

#[cfg(feature = "http")]
pub use artisan_http::direction;
#[cfg(feature = "http")]
pub use artisan_http::directions;
#[cfg(feature = "http")]
pub use artisan_http::error;
#[cfg(feature = "http")]
pub use artisan_http::directions::JsonDirection;
#[cfg(feature = "http")]
pub use artisan_http::artisan;
#[cfg(feature = "http")]
pub use artisan_http::config;
#[cfg(feature = "http")]
pub use artisan_http::flow_ctrl;
#[cfg(feature = "http")]
pub use artisan_http::http;
#[cfg(feature = "http")]
pub use artisan_http::packer;
#[cfg(feature = "http")]
pub use artisan_http::packers;
#[cfg(feature = "http")]
pub use artisan_http::plugin;
#[cfg(feature = "http")]
pub use artisan_http::plugins;
#[cfg(feature = "http")]
pub use artisan_http::rocket;
#[cfg(feature = "http")]
pub use artisan_http::shortcut;

#[cfg(feature = "http")]
pub use artisan_http::artisan::Artful;
#[cfg(feature = "http")]
pub use artisan_http::config::Config;
#[cfg(feature = "http")]
pub use artisan_http::direction::{Destination, Direction, DirectionKind};
#[cfg(feature = "http")]
pub use artisan_http::error::{ArtfulError, Result};
#[cfg(feature = "http")]
pub use artisan_http::flow_ctrl::{FlowCtrl, Next};
#[cfg(feature = "http")]
pub use artisan_http::http::get_client;
#[cfg(feature = "http")]
pub use artisan_http::packer::Packer;
#[cfg(feature = "http")]
pub use artisan_http::packers::JsonPacker;
#[cfg(feature = "http")]
pub use artisan_http::plugin::Plugin;
#[cfg(feature = "http")]
pub use artisan_http::plugins::{AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, StartPlugin};
#[cfg(feature = "http")]
pub use artisan_http::rocket::{HttpOptions, Rocket, RocketConfig};
#[cfg(feature = "http")]
pub use artisan_http::shortcut::Shortcut;
