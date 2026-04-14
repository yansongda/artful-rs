//! 内置序列化器模块
//!
//! 导出所有内置的序列化器实现。

mod json;

pub use json::JsonPacker;
