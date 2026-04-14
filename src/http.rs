//! HTTP 客户端模块
//!
//! 提供全局 HTTP 客户端单例，基于 reqwest 实现。
//!
//! # 设计说明
//!
//! - 使用 [`OnceLock`] 实现全局单例，共享连接池
//! - 连接池参数从全局 [`Config::http`] 读取
//! - Per-request timeout 通过 [`RocketConfig::http`] 设置

use std::sync::OnceLock;
use std::time::Duration;

use crate::artisan::Artful;
use crate::rocket::HttpOptions;

const DEFAULT_POOL_IDLE_TIMEOUT: u64 = 90;
const DEFAULT_POOL_MAX_IDLE_PER_HOST: usize = 20;

pub fn get_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    CLIENT.get_or_init(|| build_client(Artful::get_config().http))
}

fn build_client(http: HttpOptions) -> reqwest::Client {
    let pool_idle_timeout = http.pool_idle_timeout.unwrap_or(DEFAULT_POOL_IDLE_TIMEOUT);
    let pool_max_idle_per_host = http
        .pool_max_idle_per_host
        .unwrap_or(DEFAULT_POOL_MAX_IDLE_PER_HOST);

    reqwest::Client::builder()
        .pool_idle_timeout(Some(Duration::from_secs(pool_idle_timeout)))
        .pool_max_idle_per_host(pool_max_idle_per_host)
        .build()
        .expect("Failed to create HTTP client")
}
