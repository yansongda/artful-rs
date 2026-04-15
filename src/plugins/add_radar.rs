//! 构建 HTTP Request 插件
//!
//! 根据 RocketConfig 构建 HTTP Request 对象。
//!
//! # 行为
//!
//! - 使用 config.method 和 config.url
//! - 添加 config.headers
//! - 设置请求体（config.body 或 payload）
//! - 应用 config.http.timeout
//! - 结果存入 rocket.radar

use async_trait::async_trait;
use std::time::Duration;

use crate::Rocket;
use crate::flow_ctrl::Next;
use crate::http::get_client;
use crate::plugin::Plugin;

/// 构建 HTTP Request 插件
#[derive(Clone, Copy, Debug, Default)]
pub struct AddRadarPlugin;

#[async_trait]
impl Plugin for AddRadarPlugin {
    fn name(&self) -> &'static str {
        "AddRadarPlugin"
    }

    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> crate::Result<()> {
        let mut request_builder =
            get_client().request(rocket.config.method.clone(), &rocket.config.url);

        for (key, value) in &rocket.config.headers {
            request_builder = request_builder.header(key, value);
        }

        if let Some(body) = &rocket.config.body {
            request_builder = request_builder.body(body.clone());
        } else if !rocket.payload.is_empty() {
            let body = rocket.packer.pack(&rocket.payload)?;
            request_builder = request_builder.body(body);
        }

        if let Some(timeout) = rocket.config.http.timeout {
            request_builder = request_builder.timeout(Duration::from_secs(timeout));
        }

        let request = request_builder
            .build()
            .map_err(|e| crate::error::ArtfulError::InvalidUrl { source: e })?;
        rocket.radar = Some(request);

        next.call(rocket).await
    }
}
