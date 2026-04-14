//! JSON 序列化器实现
//!
//! 实现 [`Packer`] trait，提供 JSON 序列化/反序列化功能。

use serde_json::Value;
use std::collections::HashMap;

use crate::packer::Packer;
use crate::Result;

#[derive(Debug)]
pub struct JsonPacker;

impl Packer for JsonPacker {
    fn pack(&self, data: &HashMap<String, Value>) -> Result<String> {
        serde_json::to_string(data).map_err(Into::into)
    }

    fn unpack(&self, data: &str) -> Result<Value> {
        serde_json::from_str(data).map_err(|e| crate::error::ArtfulError::JsonDeserializeError {
            message: e.to_string(),
            source: Some(e),
        })
    }
}
