use std::sync::Arc;

#[async_trait::async_trait]
pub trait Direction: Send + Sync + std::fmt::Debug {
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination>;
}

#[derive(Debug, Clone)]
pub struct CollectionDirection;

#[async_trait::async_trait]
impl Direction for CollectionDirection {
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination> {
        let value = serde_json::to_value(&rocket.payload)?;
        Ok(Destination::Collection(value))
    }
}

#[derive(Debug, Clone)]
pub enum DirectionKind {
    CollectionDirection,
    ResponseDirection,
    NoHttpRequestDirection,
    OriginResponseDirection,
    Custom(Arc<dyn Direction>),
}

#[derive(Default)]
pub enum Destination {
    Collection(serde_json::Value),
    Response(reqwest::Response),
    Rocket(Box<crate::Rocket>),
    #[default]
    None,
}

impl std::fmt::Debug for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Destination::Collection(v) => f.debug_tuple("Collection").field(v).finish(),
            Destination::Response(_) => f.debug_tuple("Response").field(&"<reqwest::Response>").finish(),
            Destination::Rocket(r) => f.debug_tuple("Rocket").field(r).finish(),
            Destination::None => write!(f, "None"),
        }
    }
}
