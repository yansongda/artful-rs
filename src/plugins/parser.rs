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
use crate::error::ArtfulError;
use crate::flow_ctrl::Next;
use crate::http::get_client;
use crate::plugin::Plugin;

/// 解析响应插件
pub struct ParserPlugin;

#[async_trait]
impl Plugin for ParserPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> crate::Result<()> {
        // NoHttpRequestDirection - 不发起请求，直接调用下一层
        if let DirectionKind::NoHttpRequestDirection = rocket.config.direction {
            return next.call(rocket).await;
        }

        // 检查 radar 是否存在
        let request = rocket.radar.take()
            .ok_or(ArtfulError::MissingRequest)?;

        // 发送 HTTP 请求
        let client = get_client();
        let response = client.execute(request).await
            .map_err(ArtfulError::RequestFailed)?;
        
        rocket.destination_origin = Some(response);

        // 解析响应
        let direction_kind = rocket.config.direction.clone();
        let destination = match direction_kind {
            DirectionKind::JsonDirection => {
                JsonDirection.parse(rocket).await?
            }
            DirectionKind::ResponseDirection => {
                rocket.destination_origin.take()
                    .map(Destination::Response)
                    .ok_or(ArtfulError::MissingResponse)?
            }
            DirectionKind::Custom(direction) => {
                direction.parse(rocket).await?
            }
            DirectionKind::NoHttpRequestDirection => {
                Destination::None
            }
        };

        rocket.destination = Some(destination);

        next.call(rocket).await
    }
}
