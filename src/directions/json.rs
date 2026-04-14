//! JSON 解析方向
//!
//! 将响应解析为 JSON 格式。

use async_trait::async_trait;

use crate::direction::Direction;
use crate::error::ArtfulError;
use crate::{Destination, Rocket};

/// JSON 解析方向
#[derive(Debug, Clone)]
pub struct JsonDirection;

#[async_trait]
impl Direction for JsonDirection {
    async fn parse(&self, rocket: &mut Rocket) -> crate::Result<Destination> {
        match rocket.destination_origin.take() {
            Some(response) => {
                let text = response.text().await.map_err(ArtfulError::RequestFailed)?;
                Ok(Destination::Json(serde_json::from_str(&text)?))
            }
            None => Err(ArtfulError::MissingResponse),
        }
    }
}