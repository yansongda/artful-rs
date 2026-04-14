use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::plugin::Plugin;
use crate::rocket::RocketConfig;

/// 快捷方式 trait
pub trait Shortcut {
    /// 返回插件列表
    fn get_plugins(
        &self,
        config: &RocketConfig,
        payload: &HashMap<String, Value>,
    ) -> Vec<Arc<dyn Plugin>>;
}
