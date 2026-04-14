use artful::{HttpOptions, Rocket, RocketConfig};
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_rocket_config_default() {
    let config = RocketConfig::default();
    assert_eq!(config.method, reqwest::Method::POST);
    assert_eq!(config.url, "");
    assert!(config.headers.is_empty());
    assert!(config.body.is_none());
}

#[test]
fn test_rocket_new() {
    let mut params = HashMap::new();
    params.insert("key".to_string(), json!("value"));

    let rocket = Rocket::new(params);

    // config 使用默认值
    assert_eq!(rocket.config.method, reqwest::Method::POST);
    assert_eq!(rocket.config.url, "");
    assert!(rocket.radar.is_none());
    assert!(rocket.destination_origin.is_none());
    assert!(rocket.destination.is_none());
}

#[test]
fn test_http_options_default() {
    let http = HttpOptions::default();
    assert!(http.timeout.is_none());
    assert!(http.connect_timeout.is_none());
}

#[test]
fn test_http_options_with_timeout() {
    let http = HttpOptions {
        timeout: Some(30),
        connect_timeout: Some(10),
        ..Default::default()
    };
    assert_eq!(http.timeout, Some(30));
    assert_eq!(http.connect_timeout, Some(10));
}
