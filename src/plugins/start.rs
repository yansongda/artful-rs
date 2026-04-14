use async_trait::async_trait;

use crate::flow_ctrl::Next;
use crate::plugin::Plugin;
use crate::Rocket;

/// 初始化插件
pub struct StartPlugin;

#[async_trait]
impl Plugin for StartPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        next.call(rocket).await;
    }
}
