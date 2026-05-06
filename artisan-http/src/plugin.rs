//! 插件 trait 定义
//!
//! 定义插件接口，是洋葱模型的核心。
//!
//! # 插件编写模式
//!
//! ```rust
//! use artisan_http::{Plugin, Rocket, flow_ctrl::Next};
//! use async_trait::async_trait;
//!
//! pub struct MyPlugin;
//!
//! #[async_trait]
//! impl Plugin for MyPlugin {
//!     async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> artisan_http::Result<()> {
//!         // 前向逻辑：修改 rocket
//!         
//!         next.call(rocket).await?;  // 调用下一层
//!         
//!         // 后向逻辑：处理响应
//!         Ok(())
//!     }
//! }
//! ```

use async_trait::async_trait;

use crate::Rocket;
use crate::flow_ctrl::Next;

/// 插件 trait - 洋葱模型核心
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    /// 返回插件名称，用于调试和错误信息
    fn name(&self) -> &'static str {
        "UnknownPlugin"
    }

    /// 组装请求
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> crate::Result<()>;
}
