use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::direction::DirectionKind;
use crate::packer::Packer;
use crate::packers::JsonPacker;

#[derive(Debug, Clone)]
pub struct RocketConfig {
    pub method: reqwest::Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub http: HttpOptions,
    pub return_rocket: bool,
}

impl Default for RocketConfig {
    fn default() -> Self {
        Self {
            method: reqwest::Method::POST,
            url: String::new(),
            headers: HashMap::new(),
            body: None,
            http: HttpOptions::default(),
            return_rocket: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct HttpOptions {
    pub timeout: Option<u64>,
    pub connect_timeout: Option<u64>,
}

pub struct Rocket {
    pub config: RocketConfig,
    pub payload: HashMap<String, Value>,
    pub radar: Option<reqwest::Request>,
    pub destination_origin: Option<reqwest::Response>,
    pub destination: Option<crate::direction::Destination>,
    pub direction: DirectionKind,
    pub packer: Arc<dyn Packer>,
}

impl std::fmt::Debug for Rocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rocket")
            .field("config", &self.config)
            .field("payload", &self.payload)
            .field("direction", &self.direction)
            .finish()
    }
}

impl Rocket {
    pub fn new(config: RocketConfig, payload: HashMap<String, Value>) -> Self {
        Self {
            config,
            payload,
            radar: None,
            destination_origin: None,
            destination: None,
            direction: DirectionKind::CollectionDirection,
            packer: Arc::new(JsonPacker),
        }
    }
}
