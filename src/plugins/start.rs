//! 初始化插件
//!
//! 请求链的起点插件，负责将原始参数初始化到 payload。
//!
//! # 行为
//!
//! 将 rocket.params 复制到 rocket.payload，使 payload 成为可修改的工作参数。

use async_trait::async_trait;

use crate::Rocket;
use crate::flow_ctrl::Next;
use crate::plugin::Plugin;

/// 初始化插件
pub struct StartPlugin;

#[async_trait]
impl Plugin for StartPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // 将原始参数合并到 payload
        rocket.merge_payload(rocket.get_params().clone());

        next.call(rocket).await;
    }
}
