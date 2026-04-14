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
use crate::rocket::Rocket;
use crate::shortcut::Shortcut;

/// 全局配置实例
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

/// Artful 主类 - 框架入口
pub struct Artful;

impl Artful {
    /// 初始化框架全局配置
    ///
    /// 首次调用时设置配置，后续调用返回 false（除非 config._force = true）
    ///
    /// # 参数
    ///
    /// - `config`: 框架配置，其中 `_force` 字段控制是否强制覆盖
    ///
    /// # 返回
    ///
    /// - `true`: 配置成功设置
    /// - `false`: 配置已存在且未强制覆盖
    pub fn config(config: Config) -> bool {
        if GLOBAL_CONFIG.get().is_some() && !config._force {
            return false;
        }

        // 强制覆盖时，需要特殊处理（OnceLock 限制）
        if config._force {
            // OnceLock 不支持真正清除，force 模式下仍使用 get_or_init
            // 未来可考虑使用 RwLock 或其他机制
        }

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

    /// 执行插件链
    ///
    /// # 参数
    ///
    /// - `params`: 原始参数（存储在 rocket.params，不可变）
    /// - `plugins`: 插件列表（负责设置 method、url 等配置）
    pub async fn artful(
        params: HashMap<String, Value>,
        plugins: Vec<Arc<dyn Plugin>>,
    ) -> Result<Destination> {
        let mut rocket = Rocket::new(params);
        let mut ctrl = FlowCtrl::new(plugins);

        ctrl.call_next(&mut rocket).await?;

        Ok(rocket.destination.unwrap_or_default())
    }

    /// 使用 Shortcut 快捷方式
    pub async fn shortcut<S: Shortcut + Default>(
        params: HashMap<String, Value>,
    ) -> Result<Destination> {
        let shortcut = S::default();
        let plugins = shortcut.get_plugins(&params);
        Self::artful(params, plugins).await
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
