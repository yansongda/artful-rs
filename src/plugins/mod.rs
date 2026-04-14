//! 内置插件模块
//!
//! 导出所有内置插件实现。
//!
//! # 内置插件列表
//!
//! | 插件 | 功能 |
//! |------|------|
//! | [`StartPlugin`] | 初始化（占位） |
//! | [`AddPayloadBodyPlugin`] | 将 payload 序列化为请求体 |
//! | [`AddRadarPlugin`] | 构建 HTTP Request |
//! | [`ParserPlugin`] | 执行请求并解析响应 |

mod add_payload_body;
mod add_radar;
mod parser;
mod start;

pub use add_payload_body::AddPayloadBodyPlugin;
pub use add_radar::AddRadarPlugin;
pub use parser::ParserPlugin;
pub use start::StartPlugin;
