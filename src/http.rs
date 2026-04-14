//! HTTP 客户端模块
//!
//! 提供全局 HTTP 客户端单例，基于 reqwest 实现。
//!
//! # 设计说明
//!
//! - 使用 [`OnceLock`] 实现全局单例，共享连接池
//! - 连接池配置：idle_timeout = 90s, max_idle_per_host = usize::MAX
//! - Per-request timeout 通过 [`RocketConfig::http`] 设置

use std::sync::OnceLock;
use std::time::Duration;

/// 全局 HTTP 客户端单例（共享连接池）
pub fn get_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(usize::MAX)
            .build()
            .expect("Failed to create HTTP client")
    })
}
