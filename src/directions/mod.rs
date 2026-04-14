//! 内置解析方向模块
//!
//! 导出所有内置的响应解析器实现。
//!
//! # 内置解析器
//!
//! | 解析器 | 功能 |
//! |--------|------|
//! | [`JsonDirection`] | 解析响应为 JSON |

mod json;

pub use json::JsonDirection;