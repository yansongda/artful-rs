use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::direction::Destination;
use crate::flow_ctrl::FlowCtrl;
use crate::http::get_client;
use crate::plugin::Plugin;
use crate::shortcut::Shortcut;
use crate::rocket::{Rocket, RocketConfig};
use crate::Result;

/// Artful 主类 - 框架入口
pub struct Artful;

impl Artful {
    /// 执行插件链
    pub async fn artful(
        config: RocketConfig,
        payload: HashMap<String, Value>,
        plugins: Vec<Arc<dyn Plugin>>,
    ) -> Result<Destination> {
        let mut rocket = Rocket::new(config, payload);
        let mut ctrl = FlowCtrl::new(plugins);

        ctrl.call_next(&mut rocket).await;

        Ok(rocket.destination.unwrap_or_default())
    }

    /// 使用 Shortcut 快捷方式
    pub async fn shortcut<S: Shortcut + Default>(
        config: RocketConfig,
        payload: HashMap<String, Value>,
    ) -> Result<Destination> {
        let shortcut = S::default();
        let plugins = shortcut.get_plugins(&config, &payload);
        Self::artful(config, payload, plugins).await
    }

    /// 直接调用 HTTP（跳过插件链）
    pub async fn raw(request: reqwest::Request) -> Result<reqwest::Response> {
        let client = get_client();
        client
            .execute(request)
            .await
            .map_err(crate::error::ArtfulError::RequestFailed)
    }
}
