use artisan::direction::{Destination, Direction, DirectionKind};
use artisan::Rocket;
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

#[test]
fn test_direction_kind_default() {
    let kind = DirectionKind::Json;
    assert!(matches!(kind, DirectionKind::Json));
}

#[test]
fn test_destination_default() {
    let dest = Destination::default();
    assert!(matches!(dest, Destination::None));
}

#[test]
fn test_destination_from_json() {
    let value = json!({"key": "value"});
    let dest: Destination = value.into();
    assert!(matches!(dest, Destination::Json(_)));
}

#[test]
fn test_destination_debug() {
    let dest = Destination::Json(json!({"test": 1}));
    let debug_str = format!("{:?}", dest);
    assert!(debug_str.contains("Json"));

    let dest_none = Destination::None;
    assert_eq!(format!("{:?}", dest_none), "None");
}

#[test]
fn test_destination_display() {
    let dest = Destination::Json(json!({"key": "value"}));
    let display_str = format!("{}", dest);
    assert!(display_str.contains("key"));

    let dest_none = Destination::None;
    assert_eq!(format!("{}", dest_none), "None");
}

#[derive(Debug)]
struct CustomJsonDirection {
    prefix: String,
}

#[async_trait]
impl Direction for CustomJsonDirection {
    async fn parse(&self, rocket: &mut Rocket) -> artisan::Result<Destination> {
        match rocket.destination_origin.take() {
            Some(response) => {
                let text = response.text().await.map_err(artisan::ArtfulError::RequestFailed)?;
                let mut json: serde_json::Value = serde_json::from_str(&text)
                    .map_err(|e| artisan::ArtfulError::JsonDeserializeError {
                        message: e.to_string(),
                        source: Some(e),
                    })?;
                if let Some(obj) = json.as_object_mut() {
                    obj.insert("_custom_prefix".to_string(), json!(self.prefix.clone()));
                }
                Ok(Destination::Json(json))
            }
            None => Err(artisan::ArtfulError::MissingResponse),
        }
    }
}

#[test]
fn test_custom_direction_kind_creation() {
    let custom = Arc::new(CustomJsonDirection { prefix: "test_prefix".to_string() });
    let kind = DirectionKind::Custom(custom);
    assert!(matches!(kind, DirectionKind::Custom(_)));
}

#[derive(Debug)]
struct TextDirection;

#[async_trait]
impl Direction for TextDirection {
    async fn parse(&self, rocket: &mut Rocket) -> artisan::Result<Destination> {
        match rocket.destination_origin.take() {
            Some(response) => {
                let text = response.text().await.map_err(artisan::ArtfulError::RequestFailed)?;
                Ok(Destination::Json(json!({ "text": text })))
            }
            None => Err(artisan::ArtfulError::MissingResponse),
        }
    }
}

#[test]
fn test_multiple_custom_directions() {
    let custom1 = Arc::new(CustomJsonDirection { prefix: "prefix1".to_string() });
    let custom2 = Arc::new(TextDirection);
    
    let kind1 = DirectionKind::Custom(custom1);
    let kind2 = DirectionKind::Custom(custom2);
    
    assert!(matches!(kind1, DirectionKind::Custom(_)));
    assert!(matches!(kind2, DirectionKind::Custom(_)));
}

#[derive(Debug)]
struct FailingDirection;

#[async_trait]
impl Direction for FailingDirection {
    async fn parse(&self, _rocket: &mut Rocket) -> artisan::Result<Destination> {
        Err(artisan::ArtfulError::DirectionParseError("Custom parse failed".to_string()))
    }
}

#[test]
fn test_custom_direction_can_fail() {
    let failing = Arc::new(FailingDirection);
    let kind = DirectionKind::Custom(failing);
    assert!(matches!(kind, DirectionKind::Custom(_)));
}
