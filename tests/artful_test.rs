use artisan::FlowCtrl;
use artisan::Rocket;
use artisan::direction::{Destination, DirectionKind};
use artisan::plugins::{AddRadarPlugin, ParserPlugin, StartPlugin};
use artisan::{Artful, Plugin, flow_ctrl::Next, ArtfulError};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use wiremock::matchers::{method, path, header};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// 设置 HTTP 方法和 URL 的插件
struct MethodUrlPlugin {
    method: reqwest::Method,
    url: String,
}

#[async_trait]
impl Plugin for MethodUrlPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> artisan::Result<()> {
        rocket.config.method = self.method.clone();
        rocket.config.url = self.url.clone();
        next.call(rocket).await
    }
}

#[tokio::test]
async fn test_artisan_basic() {
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
async fn test_artisan_with_response_direction() {
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
    ctrl.call_next(&mut rocket).await.unwrap();

    assert!(matches!(rocket.destination, Some(Destination::Response(_))));
}

// ============ Artful::config 相关测试 ============

#[test]
fn test_artful_get_config_default() {
    // get_config 在未设置时返回默认配置
    let config = Artful::get_config();
    assert!(config.logger.enable);
    assert_eq!(config.logger.level, "info");
}

#[test]
fn test_artful_has_config() {
    // 注意：OnceLock 设置后无法更改，此测试可能在其他测试之后运行
    // 因此只测试返回值类型正确，不测试具体值
    let _has_config = Artful::has_config();
}

// ============ Artful::raw 测试 ============

#[tokio::test]
async fn test_artful_raw_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/raw-test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": "ok"})))
        .mount(&mock_server)
        .await;

    let client = artisan::get_client();
    let request = client
        .get(mock_server.uri() + "/raw-test")
        .build()
        .unwrap();

    let response = Artful::raw(request).await.unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_artful_raw_with_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/headers-test"))
        .and(header("X-Custom", "test-value"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = artisan::get_client();
    let request = client
        .post(mock_server.uri() + "/headers-test")
        .header("X-Custom", "test-value")
        .build()
        .unwrap();

    let response = Artful::raw(request).await.unwrap();
    assert_eq!(response.status(), 200);
}

// ============ 插件执行失败错误传播测试 ============

#[tokio::test]
async fn test_plugin_error_propagation() {
    struct ErrorPlugin {
        message: String,
    }

    #[async_trait]
    impl Plugin for ErrorPlugin {
        async fn assembly(
            &self,
            _rocket: &mut Rocket,
            _next: Next<'_>,
        ) -> artisan::Result<()> {
            Err(ArtfulError::PluginExecutionError {
                plugin_name: "ErrorPlugin".to_string(),
                message: self.message.clone(),
                source: None,
            })
        }
    }

    struct SuccessPlugin;

    #[async_trait]
    impl Plugin for SuccessPlugin {
        async fn assembly(
            &self,
            rocket: &mut Rocket,
            next: Next<'_>,
        ) -> artisan::Result<()> {
            rocket.payload.insert("success".to_string(), json!(true));
            next.call(rocket).await
        }
    }

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(SuccessPlugin),
        Arc::new(ErrorPlugin {
            message: "intentional failure".to_string(),
        }),
        Arc::new(SuccessPlugin), // 这个插件不会执行
    ];

    let result = Artful::artful(HashMap::new(), plugins).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(matches!(error, ArtfulError::PluginExecutionError { .. }));
}

#[tokio::test]
async fn test_plugin_chain_stops_on_error() {
    struct FirstPlugin;
    struct FailingPlugin;
    struct NeverRunPlugin;

    #[async_trait]
    impl Plugin for FirstPlugin {
        async fn assembly(
            &self,
            rocket: &mut Rocket,
            next: Next<'_>,
        ) -> artisan::Result<()> {
            rocket.payload.insert("first".to_string(), json!(1));
            next.call(rocket).await
        }
    }

    #[async_trait]
    impl Plugin for FailingPlugin {
        async fn assembly(
            &self,
            _rocket: &mut Rocket,
            _next: Next<'_>,
        ) -> artisan::Result<()> {
            Err(ArtfulError::Other("plugin failed".to_string()))
        }
    }

    #[async_trait]
    impl Plugin for NeverRunPlugin {
        async fn assembly(
            &self,
            rocket: &mut Rocket,
            next: Next<'_>,
        ) -> artisan::Result<()> {
            rocket.payload.insert("never_run".to_string(), json!(true));
            next.call(rocket).await
        }
    }

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(FirstPlugin),
        Arc::new(FailingPlugin),
        Arc::new(NeverRunPlugin),
    ];

    let mut rocket = Rocket::new(HashMap::new());
    let mut ctrl = FlowCtrl::new(plugins);
    
    let result = ctrl.call_next(&mut rocket).await;
    
    assert!(result.is_err());
    assert!(rocket.payload.contains_key("first"));
    assert!(!rocket.payload.contains_key("never_run"));
}

// ============ HTTP 请求失败测试 ============

#[tokio::test]
async fn test_http_404_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/not-found"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"error": "Not Found"})))
        .mount(&mock_server)
        .await;

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(MethodUrlPlugin {
            method: reqwest::Method::GET,
            url: mock_server.uri() + "/not-found",
        }),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    // 404 不会返回错误，而是正常解析响应
    let result = Artful::artful(HashMap::new(), plugins).await.unwrap();
    
    if let Destination::Json(json) = result {
        assert_eq!(json["error"], "Not Found");
    } else {
        panic!("Expected JSON destination");
    }
}

#[tokio::test]
async fn test_http_500_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/server-error"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({"error": "Internal Server Error"})))
        .mount(&mock_server)
        .await;

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(MethodUrlPlugin {
            method: reqwest::Method::POST,
            url: mock_server.uri() + "/server-error",
        }),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    // 500 不会返回错误，而是正常解析响应
    let result = Artful::artful(HashMap::new(), plugins).await.unwrap();
    
    if let Destination::Json(json) = result {
        assert_eq!(json["error"], "Internal Server Error");
    } else {
        panic!("Expected JSON destination");
    }
}

#[tokio::test]
async fn test_http_timeout_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/slow"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_secs(5)) // 5秒延迟
        )
        .mount(&mock_server)
        .await;

    struct TimeoutUrlPlugin {
        url: String,
        timeout: u64,
    }

    #[async_trait]
    impl Plugin for TimeoutUrlPlugin {
        async fn assembly(
            &self,
            rocket: &mut Rocket,
            next: Next<'_>,
        ) -> artisan::Result<()> {
            rocket.config.method = reqwest::Method::GET;
            rocket.config.url = self.url.clone();
            rocket.config.http.timeout = Some(self.timeout);
            next.call(rocket).await
        }
    }

    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(TimeoutUrlPlugin {
            url: mock_server.uri() + "/slow",
            timeout: 1, // 1秒超时
        }),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(HashMap::new(), plugins).await;
    
    // 请求应该超时失败
    assert!(result.is_err());
    let error = result.unwrap_err();
    // reqwest 超时错误会被包装成 RequestFailed
    assert!(matches!(error, ArtfulError::RequestFailed(_)));
}

#[tokio::test]
async fn test_http_invalid_url() {
    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(MethodUrlPlugin {
            method: reqwest::Method::GET,
            url: "not-a-valid-url".to_string(),
        }),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(HashMap::new(), plugins).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_http_nonexistent_host() {
    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(MethodUrlPlugin {
            method: reqwest::Method::GET,
            url: "http://nonexistent-host-12345.local/test".to_string(),
        }),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(HashMap::new(), plugins).await;
    
    assert!(result.is_err());
}
