//! Shortcut 快捷方式示例

use artful::plugins::{AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, StartPlugin};
use artful::{Artful, Plugin, Rocket, Shortcut, flow_ctrl::Next};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

/// 设置 HTTP 方法和 URL 的插件
struct MethodUrlPlugin {
    method: reqwest::Method,
    url: String,
}

#[async_trait]
impl Plugin for MethodUrlPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> artful::Result<()> {
        rocket.config.method = self.method.clone();
        rocket.config.url = self.url.clone();
        next.call(rocket).await
    }
}

/// HTTPBin POST 快捷方式
#[derive(Default)]
struct HttpbinPostShortcut;

impl Shortcut for HttpbinPostShortcut {
    fn get_plugins(&self, _params: &HashMap<String, serde_json::Value>) -> Vec<Arc<dyn Plugin>> {
        vec![
            Arc::new(StartPlugin),
            Arc::new(MethodUrlPlugin {
                method: reqwest::Method::POST,
                url: "https://httpbin.org/post".to_string(),
            }),
            Arc::new(AddPayloadBodyPlugin),
            Arc::new(AddRadarPlugin),
            Arc::new(ParserPlugin),
        ]
    }
}

/// HTTPBin GET 快捷方式
#[derive(Default)]
struct HttpbinGetShortcut;

impl Shortcut for HttpbinGetShortcut {
    fn get_plugins(&self, _params: &HashMap<String, serde_json::Value>) -> Vec<Arc<dyn Plugin>> {
        vec![
            Arc::new(StartPlugin),
            Arc::new(MethodUrlPlugin {
                method: reqwest::Method::GET,
                url: "https://httpbin.org/get".to_string(),
            }),
            Arc::new(AddRadarPlugin),
            Arc::new(ParserPlugin),
        ]
    }
}

#[tokio::main]
async fn main() -> artful::Result<()> {
    // 使用 POST 快捷方式
    let mut params = HashMap::new();
    params.insert("data".to_string(), json!("hello world"));

    let result = Artful::shortcut::<HttpbinPostShortcut>(params).await?;

    if let artful::Destination::Json(json) = result {
        println!("POST Response: {}", json);
    }

    // 使用 GET 快捷方式
    let result = Artful::shortcut::<HttpbinGetShortcut>(HashMap::new()).await?;

    if let artful::Destination::Json(json) = result {
        println!("GET Response: {}", json);
    }

    Ok(())
}
