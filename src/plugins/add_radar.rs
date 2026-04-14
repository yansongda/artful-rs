use async_trait::async_trait;
use std::time::Duration;

use crate::flow_ctrl::Next;
use crate::http::get_client;
use crate::plugin::Plugin;
use crate::Rocket;

/// 构建 HTTP Request 插件
pub struct AddRadarPlugin;

#[async_trait]
impl Plugin for AddRadarPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        let method = rocket.config.method.clone();
        let url = rocket.config.url.clone();

        let client = get_client();
        let mut request_builder = client.request(method, &url);

        for (key, value) in &rocket.config.headers {
            request_builder = request_builder.header(key, value);
        }

        if let Some(body) = &rocket.config.body {
            request_builder = request_builder.body(body.clone());
        } else if !rocket.payload.is_empty() {
            if let Ok(body) = rocket.packer.pack(&rocket.payload) {
                request_builder = request_builder.body(body);
            }
        }

        if let Some(timeout) = rocket.config.http.timeout {
            request_builder = request_builder.timeout(Duration::from_secs(timeout));
        }

        if let Ok(request) = request_builder.build() {
            rocket.radar = Some(request);
        }

        next.call(rocket).await;
    }
}
