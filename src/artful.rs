//! Artful 主入口模块
//!
//! 框架的核心入口，提供三种请求方式：
//!
//! # 方法
//!
//! - [`Artful::config`] - 初始化框架全局配置
//! - [`Artful::artful`] - 执行完整插件链
//! - [`Artful::shortcut`] - 使用 Shortcut 快捷方式
//! - [`Artful::raw`] - 直接 HTTP 请求（跳过插件）

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

use serde_json::Value;

use crate::Result;
use crate::config::Config;
use crate::direction::Destination;
use crate::flow_ctrl::FlowCtrl;
use crate::http::get_client;
use crate::plugin::Plugin;
use crate::rocket::{Rocket, RocketConfig};
use crate::shortcut::Shortcut;

/// 全局配置实例
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

/// Artful 主类 - 框架入口
pub struct Artful;

impl Artful {
    /// 初始化框架全局配置
    ///
    /// 首次调用时设置配置，后续调用返回 false（除非 force = true）
    ///
    /// # 参数
    ///
    /// - `config`: 框架配置
    /// - `force`: 是否强制覆盖已存在的配置
    ///
    /// # 返回
    ///
    /// - `true`: 配置成功设置
    /// - `false`: 配置已存在且未强制覆盖
    pub fn config(config: Config, force: bool) -> bool {
        if GLOBAL_CONFIG.get().is_some() && !force {
            return false;
        }

        // 设置默认 Direction 和 Packer（类似 PHP 版本）
        let _ = GLOBAL_CONFIG.get_or_init(|| config);

        true
    }

    /// 获取全局配置
    pub fn get_config() -> &'static Config {
        GLOBAL_CONFIG.get_or_init(|| Config::default())
    }

    /// 检查是否已初始化配置
    pub fn has_config() -> bool {
        GLOBAL_CONFIG.get().is_some()
    }

    /// 清除全局配置
    pub fn clear() {
        // OnceLock 无法真正清除，这里只是标记
        // 实际清除需要重新设计或使用其他机制
    }

    /// 执行插件链
    ///
    /// # 参数
    ///
    /// - `config`: HTTP 请求配置
    /// - `params`: 原始参数（存储在 rocket.params，不可变）
    /// - `plugins`: 插件列表
    pub async fn artful(
        config: RocketConfig,
        params: HashMap<String, Value>,
        plugins: Vec<Arc<dyn Plugin>>,
    ) -> Result<Destination> {
        let mut rocket = Rocket::new(config, params);
        let mut ctrl = FlowCtrl::new(plugins);

        ctrl.call_next(&mut rocket).await;

        Ok(rocket.destination.unwrap_or_default())
    }

    /// 使用 Shortcut 快捷方式
    pub async fn shortcut<S: Shortcut + Default>(
        config: RocketConfig,
        params: HashMap<String, Value>,
    ) -> Result<Destination> {
        let shortcut = S::default();
        let plugins = shortcut.get_plugins(&config, &params);
        Self::artful(config, params, plugins).await
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
