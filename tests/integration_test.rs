use artful::direction::Destination;
use artful::plugins::{AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, StartPlugin};
use artful::{Artful, Plugin, RocketConfig};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_full_pipeline() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/orders"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"code": 0, "data": "success"})),
        )
        .mount(&mock_server)
        .await;

    let config = RocketConfig {
        method: reqwest::Method::POST,
        url: mock_server.uri() + "/api/orders",
        ..Default::default()
    };

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(AddPayloadBodyPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(config, HashMap::new(), plugins)
        .await
        .unwrap();

    assert!(matches!(result, Destination::Collection(_)));
}

#[tokio::test]
async fn test_pipeline_with_payload() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "ok"})))
        .mount(&mock_server)
        .await;

    let config = RocketConfig {
        method: reqwest::Method::POST,
        url: mock_server.uri() + "/api/test",
        ..Default::default()
    };

    let payload = HashMap::from([
        ("order_id".to_string(), json!("123")),
        ("amount".to_string(), json!(100)),
    ]);

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(AddPayloadBodyPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(config, payload, plugins).await.unwrap();

    assert!(matches!(result, Destination::Collection(_)));
}
