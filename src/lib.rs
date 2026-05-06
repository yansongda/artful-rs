//! Artisan workspace facade，通过 feature 控制 re-export。
//!
//! # Features
//!
//! - `http`（默认启用）- re-export [`artisan_http`] 的核心类型和插件
//!
//! # 核心类型（需启用 `http` feature）
//!
//! - [`Rocket`] - 请求载体，携带整个请求生命周期的数据
//! - [`Plugin`] - 插件 trait，洋葱模型的核心
//! - [`FlowCtrl`] - 流向控制器，管理插件执行顺序
//! - [`Artful`] - 框架主入口

// 核心类型
#[cfg(feature = "http")]
pub use artisan_http::Artful;
#[cfg(feature = "http")]
pub use artisan_http::Plugin;
#[cfg(feature = "http")]
pub use artisan_http::Rocket;
#[cfg(feature = "http")]
pub use artisan_http::RocketConfig;
#[cfg(feature = "http")]
pub use artisan_http::{ArtfulError, Result};
#[cfg(feature = "http")]
pub use artisan_http::{FlowCtrl, Next};

// 配置和选项
#[cfg(feature = "http")]
pub use artisan_http::Config;
#[cfg(feature = "http")]
pub use artisan_http::HttpOptions;

// 响应解析
#[cfg(feature = "http")]
pub use artisan_http::{Destination, Direction, DirectionKind};

// 序列化
#[cfg(feature = "http")]
pub use artisan_http::Packer;

// 快捷方式
#[cfg(feature = "http")]
pub use artisan_http::Shortcut;

// 内置插件
#[cfg(feature = "http")]
pub use artisan_http::plugins::{AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, StartPlugin};

// 模块（供高级用户使用）
#[cfg(feature = "http")]
pub use artisan_http::{
    config, direction, directions, error, flow_ctrl, http, packer, packers, plugin, plugins,
    rocket, shortcut,
};
