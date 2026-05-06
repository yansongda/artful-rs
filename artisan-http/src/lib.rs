//! artisan-http - HTTP implementation for artisan framework

pub mod direction;
pub mod directions;
pub mod error;
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
pub use config::Config;
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
