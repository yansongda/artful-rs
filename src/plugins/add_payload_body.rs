//! Payload Body 插件
//!
//! 将 payload 序列化为 HTTP 请求体。
//!
//! # 行为
//!
//! - 仅在 `rocket.config.body` 未设置时生效
//! - 使用 rocket.packer 序列化 payload
//! - 设置结果到 `rocket.config.body`

use async_trait::async_trait;

use crate::Rocket;
use crate::flow_ctrl::Next;
use crate::plugin::Plugin;

/// 添加 payload body 插件
pub struct AddPayloadBodyPlugin;

#[async_trait]
impl Plugin for AddPayloadBodyPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> crate::Result<()> {
        if rocket.config.body.is_none() && !rocket.payload.is_empty() {
            rocket.config.body = Some(rocket.packer.pack(&rocket.payload)?);
        }

        next.call(rocket).await
    }
}
