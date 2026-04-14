use artful::FlowCtrl;
use artful::Rocket;
use artful::direction::{Destination, DirectionKind};
use artful::plugins::{AddRadarPlugin, ParserPlugin, StartPlugin};
use artful::{Artful, RocketConfig};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_artful_basic() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success": true})))
        .mount(&mock_server)
        .await;

    let config = RocketConfig {
        url: mock_server.uri() + "/test",
        ..Default::default()
    };

    let plugins: Vec<Arc<dyn artful::Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(config, HashMap::new(), plugins)
        .await
        .unwrap();

    assert!(matches!(result, Destination::Collection(_)));
}

#[tokio::test]
async fn test_artful_with_response_direction() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/raw"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("raw response", "text/plain"))
        .mount(&mock_server)
        .await;

    let config = RocketConfig {
        method: reqwest::Method::GET,
        url: mock_server.uri() + "/raw",
        ..Default::default()
    };

    let mut rocket = Rocket::new(config, HashMap::new());
    rocket.direction = DirectionKind::ResponseDirection;

    let plugins: Vec<Arc<dyn artful::Plugin>> =
        vec![Arc::new(AddRadarPlugin), Arc::new(ParserPlugin)];

    let mut ctrl = FlowCtrl::new(plugins);
    ctrl.call_next(&mut rocket).await;

    assert!(matches!(rocket.destination, Some(Destination::Response(_))));
}
