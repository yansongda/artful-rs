use async_trait::async_trait;

use crate::flow_ctrl::Next;
use crate::plugin::Plugin;
use crate::Rocket;

/// 添加 payload body 插件
pub struct AddPayloadBodyPlugin;

#[async_trait]
impl Plugin for AddPayloadBodyPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        if rocket.config.body.is_none() && !rocket.payload.is_empty() {
            if let Ok(body) = rocket.packer.pack(&rocket.payload) {
                rocket.config.body = Some(body);
            }
        }

        next.call(rocket).await;
    }
}
