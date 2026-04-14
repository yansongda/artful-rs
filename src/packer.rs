use serde_json::Value;
use std::collections::HashMap;

use crate::Result;

pub trait Packer: Send + Sync + std::fmt::Debug {
    fn pack(&self, data: &HashMap<String, Value>) -> Result<String>;
    fn unpack(&self, data: &str) -> Result<Value>;
}
