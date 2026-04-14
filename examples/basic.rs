//! 基础使用示例

use artful::{Artful, Plugin, Rocket, flow_ctrl::Next};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::json;

/// 设置 HTTP 方法和 URL 的插件
struct MethodUrlPlugin {
    method: reqwest::Method,
    url: String,
}

#[async_trait]
impl Plugin for MethodUrlPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        rocket.config.method = self.method.clone();
        rocket.config.url = self.url.clone();
        next.call(rocket).await;
    }
}

#[tokio::main]
async fn main() -> artful::Result<()> {
    let mut params = HashMap::new();
    params.insert("order_id".to_string(), json!("123"));
    params.insert("amount".to_string(), json!(100));

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(MethodUrlPlugin {
            method: reqwest::Method::POST,
            url: "https://httpbin.org/post".to_string(),
        }),
        Arc::new(AddPayloadBodyPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(params, plugins).await?;
    
    if let artful::Destination::Json(json) = result {
        println!("Response: {}", json);
    }

    Ok(())
}