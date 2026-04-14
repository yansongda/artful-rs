//! 日志记录插件
//!
//! 记录请求生命周期日志。
//!
//! # 行为
//!
//! - 前向：记录请求开始（method, url）
//! - 后向：记录请求完成（status, elapsed_ms）

use async_trait::async_trait;
use std::time::Instant;

use crate::Rocket;
use crate::flow_ctrl::Next;
use crate::plugin::Plugin;

/// 日志记录插件
pub struct LogPlugin;

#[async_trait]
impl Plugin for LogPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        let start = Instant::now();

        tracing::info!(
            method = %rocket.config.method,
            url = %rocket.config.url,
            "Request started"
        );

        next.call(rocket).await;

        let elapsed = start.elapsed();

        if let Some(resp) = &rocket.destination_origin {
            tracing::info!(
                status = resp.status().as_u16(),
                elapsed_ms = elapsed.as_millis(),
                "Request completed"
            );
        }
    }
}
