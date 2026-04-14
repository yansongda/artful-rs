//! 解析响应插件
//!
//! 执行 HTTP 请求并解析响应。
//!
//! # 行为
//!
//! - 检查 rocket.config.direction，决定是否发起请求
//! - 执行 HTTP 请求，存入 rocket.destination_origin
//! - 根据 DirectionKind 解析响应
//! - 结果存入 rocket.destination

use async_trait::async_trait;

use crate::Rocket;
use crate::direction::{Destination, Direction, DirectionKind, JsonDirection};
use crate::flow_ctrl::Next;
use crate::http::get_client;
use crate::plugin::Plugin;

/// 解析响应插件
pub struct ParserPlugin;

#[async_trait]
impl Plugin for ParserPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        if let DirectionKind::NoHttpRequestDirection = rocket.config.direction {
            next.call(rocket).await;
            return;
        }

        if rocket.radar.is_none() {
            next.call(rocket).await;
            return;
        }

        let client = get_client();
        let request = rocket.radar.take().unwrap();

        if let Ok(response) = client.execute(request).await {
            rocket.destination_origin = Some(response);

            let direction_kind = rocket.config.direction.clone();

            match direction_kind {
                DirectionKind::JsonDirection => {
                    let direction = JsonDirection;
                    if let Ok(dest) = direction.parse(rocket).await {
                        rocket.destination = Some(dest);
                    }
                }
                DirectionKind::ResponseDirection => {
                    if let Some(resp) = rocket.destination_origin.take() {
                        rocket.destination = Some(Destination::Response(resp));
                    }
                }
                DirectionKind::OriginResponseDirection => {
                    if rocket.config.return_rocket {
                        // Rocket doesn't implement Clone yet
                    }
                }
                DirectionKind::Custom(d) => {
                    if let Ok(dest) = d.parse(rocket).await {
                        rocket.destination = Some(dest);
                    }
                }
                _ => {}
            }
        }

        next.call(rocket).await;
    }
}
