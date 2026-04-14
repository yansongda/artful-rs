//! 初始化插件
//!
//! 请求链的起点插件，当前版本仅作为占位。
//!
//! # 后续迭代
//!
//! 未来版本可在此添加初始化逻辑，如：
//! - 参数验证
//! - 默认值设置
//! - 上下文初始化

use async_trait::async_trait;

use crate::Rocket;
use crate::flow_ctrl::Next;
use crate::plugin::Plugin;

/// 初始化插件
pub struct StartPlugin;

#[async_trait]
impl Plugin for StartPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        next.call(rocket).await;
    }
}
