use artful::FlowCtrl;
use artful::Rocket;
use artful::direction::{Destination, DirectionKind};
use artful::plugins::{AddRadarPlugin, ParserPlugin, StartPlugin};
use artful::{Artful, Plugin, flow_ctrl::Next};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// 设置 HTTP 方法和 URL 的插件
struct MethodUrlPlugin {
    method: reqwest::Method,
    url: String,
}

#[async_trait]
impl Plugin for MethodUrlPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        rocket.config.method = self.method.clone();
        rocket.config.url = self.url.clone();
        next.call(rocket).await;
    }
}

#[tokio::test]
async fn test_artful_basic() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"success": true})))
        .mount(&mock_server)
        .await;

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(MethodUrlPlugin {
            method: reqwest::Method::POST,
            url: mock_server.uri() + "/test",
        }),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(HashMap::new(), plugins).await.unwrap();

    assert!(matches!(result, Destination::Json(_)));
}

#[tokio::test]
async fn test_artful_with_response_direction() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/raw"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("raw response", "text/plain"))
        .mount(&mock_server)
        .await;

    let mut rocket = Rocket::new(HashMap::new());
    rocket.config.method = reqwest::Method::GET;
    rocket.config.url = mock_server.uri() + "/raw";
    rocket.config.direction = DirectionKind::ResponseDirection;

    let plugins: Vec<Arc<dyn Plugin>> = vec![Arc::new(AddRadarPlugin), Arc::new(ParserPlugin)];

    let mut ctrl = FlowCtrl::new(plugins);
    ctrl.call_next(&mut rocket).await;

    assert!(matches!(rocket.destination, Some(Destination::Response(_))));
}
