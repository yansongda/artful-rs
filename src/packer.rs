//! 序列化器 trait 定义
//!
//! 定义数据序列化/反序列化的抽象接口。
//!
//! # 内置实现
//!
//! - [`JsonPacker`] - JSON 序列化器（默认）

use serde_json::Value;
use std::collections::HashMap;

use crate::Result;

pub trait Packer: Send + Sync + std::fmt::Debug {
    fn pack(&self, data: &HashMap<String, Value>) -> Result<String>;
    fn unpack(&self, data: &str) -> Result<Value>;
}
