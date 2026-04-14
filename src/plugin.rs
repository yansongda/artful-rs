use async_trait::async_trait;

use crate::Rocket;
use crate::flow_ctrl::Next;

/// 插件 trait - 洋葱模型核心
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    /// 组装请求
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>);
}
