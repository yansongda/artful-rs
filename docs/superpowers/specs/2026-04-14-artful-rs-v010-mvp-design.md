# Artful-Rs v0.1.0 MVP 实现规范

> 基于 docs/ARCHITECTURE.md 架构设计文档的 v0.1.0 MVP 实现详细规范

## 一、实现范围

### 1.1 核心架构模块

| 模块文件 | 功能 | 主要类型 |
|---------|------|---------|
| `src/lib.rs` | 框架入口，导出公共 API | - |
| `src/artful.rs` | 主入口 | `Artful` struct |
| `src/rocket.rs` | 请求载体 + 配置 | `Rocket` struct, `RocketConfig`, `Method`, `HttpOptions` |
| `src/flow_ctrl.rs` | 流向控制器 | `FlowCtrl` struct, `Next` struct |
| `src/plugin.rs` | 插件 trait 定义 | `Plugin` trait |
| `src/config.rs` | 配置管理 | `Config`, `LoggerConfig` |
| `src/error.rs` | 错误定义 | `ArtfulError` enum |
| `src/http.rs` | HTTP 客户端封装 | reqwest wrapper |
| `src/shortcut.rs` | 快捷方式 trait | `Shortcut` trait |

**变更说明**：
- 移除 `src/payload.rs` - payload 直接使用 `HashMap<String, Value>`
- `src/rocket.rs` 增加 `RocketConfig`、`Method`、`HttpOptions` 类型
- `src/config.rs` 移除 `HttpConfig` - HTTP 配置通过 `RocketConfig.http` 传递

### 1.2 辅助模块

| 模块文件 | 功能 | 主要类型 |
|---------|------|---------|
| `src/direction.rs` | 解析策略 trait + 枚举 | `Direction` trait, `DirectionKind`, `Destination` |
| `src/directions/mod.rs` | 导出内置 Direction | - |
| `src/directions/collection.rs` | JSON Collection 解析器 | `CollectionDirection` |
| `src/directions/response.rs` | 原始 Response 解析器 | `ResponseDirection` |
| `src/directions/no_http.rs` | 不发起请求解析器 | `NoHttpRequestDirection` |
| `src/directions/origin.rs` | 返回 Rocket 解析器 | `OriginResponseDirection` |
| `src/packer.rs` | 序列化 trait | `Packer` trait |
| `src/packers/mod.rs` | 导出内置 Packer | - |
| `src/packers/json.rs` | JSON 序列化器 | `JsonPacker` |

### 1.3 内置插件

| 插件文件 | 功能 | 插件名称 |
|---------|------|---------|
| `src/plugins/mod.rs` | 导出所有内置插件 | - |
| `src/plugins/start.rs` | 初始化插件 | `StartPlugin` |
| `src/plugins/add_radar.rs` | 构建 HTTP Request | `AddRadarPlugin` |
| `src/plugins/parser.rs` | 解析响应 | `ParserPlugin` |
| `src/plugins/add_payload_body.rs` | 添加 payload body | `AddPayloadBodyPlugin` |
| `src/plugins/log.rs` | 日志记录 | `LogPlugin` |

### 1.4 测试覆盖

| 测试文件 | 测试内容 |
|---------|---------|
| `tests/rocket_test.rs` | Rocket 创建、RocketConfig、payload 操作 |
| `tests/flow_ctrl_test.rs` | FlowCtrl 洋葱模型流程控制 |
| `tests/plugin_test.rs` | Plugin trait 实现、插件链执行 |
| `tests/direction_test.rs` | Direction 解析策略测试 |
| `tests/packer_test.rs` | Packer 序列化测试 |
| `tests/integration_test.rs` | 完整插件链集成测试 |
| `tests/shortcut_test.rs` | Shortcut 快捷方式测试 |
| `tests/artful_test.rs` | Artful 主入口测试 |

## 二、错误分类设计

### 2.1 ArtfulError 枚举

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtfulError {
    // HTTP 错误
    #[error("HTTP 请求失败: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("请求超时")]
    Timeout,
    
    #[error("网络错误: {0}")]
    NetworkError(String),
    
    #[error("无效的 URL: {0}")]
    InvalidUrl(String),
    
    // 序列化错误
    #[error("JSON 序列化失败: {0}")]
    JsonSerializeError(#[from] serde_json::Error),
    
    #[error("JSON 反序列化失败: {message}")]
    JsonDeserializeError { message: String },
    
    // 插件错误
    #[error("插件执行错误: {plugin_name} - {message}")]
    PluginExecutionError { plugin_name: String, message: String },
    
    // 参数错误
    #[error("缺少必要参数: {0}")]
    MissingParameter(String),
    
    #[error("参数无效: {param} - {message}")]
    InvalidParameter { param: String, message: String },
    
    // Direction 错误
    #[error("响应解析失败: {0}")]
    DirectionParseError(String),
    
    // 其他错误
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, ArtfulError>;
```

## 三、核心类型设计

### 3.1 RocketConfig - Rocket 配置

```rust
use std::collections::HashMap;

/// Rocket 配置（固定参数，类型安全）
#[derive(Debug, Clone)]
pub struct RocketConfig {
    /// HTTP 方法（默认 POST）
    pub method: Method,
    
    /// 请求 URL（必填）
    pub url: String,
    
    /// 请求头
    pub headers: HashMap<String, String>,
    
    /// 请求体（可选，手动指定）
    pub body: Option<String>,
    
    /// HTTP 选项
    pub http: HttpOptions,
    
    /// 是否返回 Rocket（调试用）
    pub return_rocket: bool,
}

impl Default for RocketConfig {
    fn default() -> Self {
        Self {
            method: Method::POST,
            url: String::new(),
            headers: HashMap::new(),
            body: None,
            http: HttpOptions::default(),
            return_rocket: false,
        }
    }
}
```

### 3.2 Method - HTTP 方法

```rust
/// HTTP 方法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::PATCH => "PATCH",
            Method::HEAD => "HEAD",
            Method::OPTIONS => "OPTIONS",
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

### 3.3 HttpOptions - HTTP 请求选项

```rust
/// HTTP 请求选项
#[derive(Debug, Clone, Default)]
pub struct HttpOptions {
    /// 请求超时（秒）
    pub timeout: Option<u64>,
    
    /// 连接超时（秒）
    pub connect_timeout: Option<u64>,
}
```

### 3.4 Rocket - 请求载体

```rust
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

/// 请求载体 - 携带整个请求生命周期中的所有数据
pub struct Rocket {
    /// Rocket 配置
    pub config: RocketConfig,
    
    /// 业务参数（动态）
    pub payload: HashMap<String, Value>,
    
    /// HTTP 请求对象
    pub radar: Option<reqwest::Request>,
    
    /// HTTP 原始响应
    pub destination_origin: Option<reqwest::Response>,
    
    /// 最终解析结果
    pub destination: Option<Destination>,
    
    /// 响应解析策略
    pub direction: DirectionKind,
    
    /// 序列化器
    pub packer: Arc<dyn Packer>,
}

impl Rocket {
    /// 创建 Rocket
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
    
    /// 检查是否已终止
    pub fn is_ceased(&self) -> bool {
        // 由 FlowCtrl 管理
        false
    }
}
```

### 3.5 FlowCtrl - 流向控制器

```rust
use std::sync::Arc;

/// 洋葱模型流向控制器
pub struct FlowCtrl {
    /// 当前执行位置
    cursor: usize,
    
    /// 插件列表（线性排列）
    plugins: Vec<Arc<dyn Plugin>>,
    
    /// 是否已终止
    is_ceased: bool,
}

impl FlowCtrl {
    /// 创建新的流向控制器
    pub fn new(plugins: Vec<Arc<dyn Plugin>>) -> Self {
        Self {
            cursor: 0,
            plugins,
            is_ceased: false,
        }
    }
    
    /// 调用下一层插件（洋葱穿透）
    pub async fn call_next(&mut self, rocket: &mut Rocket) {
        if self.is_ceased || !self.has_next() {
            return;
        }
        
        let plugin = self.plugins[self.cursor].clone();
        self.cursor += 1;
        
        let next = Next { ctrl: self };
        plugin.assembly(rocket, next).await;
    }
    
    /// 检查是否还有下一层
    pub fn has_next(&self) -> bool {
        self.cursor < self.plugins.len()
    }
    
    /// 跳过剩余所有插件
    pub fn skip_rest(&mut self) {
        self.cursor = self.plugins.len();
        self.is_ceased = true;
    }
    
    /// 终止并标记
    pub fn cease(&mut self) {
        self.is_ceased = true;
        self.skip_rest();
    }
    
    /// 检查是否已终止
    pub fn is_ceased(&self) -> bool {
        self.is_ceased
    }
}
```

### 3.6 Next - 闭包穿透

```rust
/// 下一个插件的闭包（洋葱穿透）
pub struct Next<'a> {
    ctrl: &'a mut FlowCtrl,
}

impl<'a> Next<'a> {
    /// 调用下一个插件
    pub async fn call(self, rocket: &mut Rocket) {
        self.ctrl.call_next(rocket).await;
    }
}
```

### 3.7 Plugin - 插件 trait

```rust
use async_trait::async_trait;

/// 插件 trait - 洋葱模型核心
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    /// 组装请求
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>);
}
```

### 3.8 Direction - 响应解析器

```rust
use async_trait::async_trait;
use serde_json::Value;

/// 响应解析器 trait
#[async_trait]
pub trait Direction: Send + Sync {
    /// 解析响应
    async fn parse(&self, rocket: &mut Rocket) -> Result<Destination>;
}

/// 响应解析策略
#[derive(Clone)]
pub enum DirectionKind {
    /// 解析为 JSON Collection（默认）
    CollectionDirection,
    /// 返回原始 Response
    ResponseDirection,
    /// 不发起 HTTP 请求
    NoHttpRequestDirection,
    /// 返回原始 Rocket（用于调试）
    OriginResponseDirection,
    /// 自定义解析器
    Custom(Arc<dyn Direction>),
}

/// 解析结果
#[derive(Debug)]
pub enum Destination {
    /// JSON Collection（默认）
    Collection(Value),
    /// 原始响应（注意：Response 不能 Clone）
    Response(reqwest::Response),
    /// Rocket 本身（用于调试）
    Rocket(Box<Rocket>),
    /// 空结果
    None,
}

impl Default for Destination {
    fn default() -> Self {
        Destination::None
    }
}
```

### 3.9 Packer - 序列化器

```rust
use serde_json::Value;

/// 序列化器 trait
pub trait Packer: Send + Sync {
    /// 序列化数据
    fn pack(&self, data: &HashMap<String, Value>) -> Result<String>;
    
    /// 反序列化数据
    fn unpack(&self, data: &str) -> Result<Value>;
}
```

### 3.10 Shortcut - 快捷方式

```rust
use std::sync::Arc;

/// 快捷方式 trait
pub trait Shortcut {
    /// 返回插件列表
    fn get_plugins(&self, config: &RocketConfig, payload: &HashMap<String, Value>) 
        -> Vec<Arc<dyn Plugin>>;
}
```

## 四、内置插件实现

### 4.1 StartPlugin

**功能**：初始化插件

**实现要点**：
- 当前 v0.1.0 版本，StartPlugin 主要作为占位
- payload 已经是纯净的业务参数，无需过滤
- 调用 `next.call(rocket).await`

```rust
pub struct StartPlugin;

#[async_trait]
impl Plugin for StartPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // v0.1.0: payload 已经是纯净业务参数
        // 后续版本可在此添加初始化逻辑
        next.call(rocket).await;
    }
}
```

### 4.2 AddPayloadBodyPlugin

**功能**：将 payload 转换为请求体

**实现要点**：
- 检查 `rocket.config.body` 是否已设置
- 如果未设置，将 payload 序列化为 JSON
- 设置到 `rocket.config.body`
- 调用 `next.call(rocket).await`

```rust
pub struct AddPayloadBodyPlugin;

#[async_trait]
impl Plugin for AddPayloadBodyPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        if rocket.config.body.is_none() && !rocket.payload.is_empty() {
            let body = rocket.packer.pack(&rocket.payload)?;
            rocket.config.body = Some(body);
        }
        
        next.call(rocket).await;
    }
}
```

### 4.3 AddRadarPlugin

**功能**：构建 HTTP Request

**实现要点**：
- 使用 `rocket.config.method`、`rocket.config.url`
- 添加 `rocket.config.headers`
- 设置请求体（从 `rocket.config.body` 或 payload）
- 应用 `rocket.config.http` timeout 设置
- 构建 `reqwest::Request`
- 存入 `rocket.radar`
- 调用 `next.call(rocket).await`

```rust
pub struct AddRadarPlugin;

#[async_trait]
impl Plugin for AddRadarPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        let method = rocket.config.method.as_str();
        let url = &rocket.config.url;
        
        let client = get_client();
        let mut request_builder = client.request(method, url);
        
        // 添加 headers
        for (key, value) in &rocket.config.headers {
            request_builder = request_builder.header(key, value);
        }
        
        // 添加 body
        if let Some(body) = &rocket.config.body {
            request_builder = request_builder.body(body.clone());
        } else if !rocket.payload.is_empty() {
            let body = rocket.packer.pack(&rocket.payload)?;
            request_builder = request_builder.body(body);
        }
        
        // 应用 timeout
        if let Some(timeout) = rocket.config.http.timeout {
            request_builder = request_builder.timeout(
                std::time::Duration::from_secs(timeout)
            );
        }
        
        rocket.radar = Some(request_builder.build()?);
        
        next.call(rocket).await;
    }
}
```

### 4.4 ParserPlugin

**功能**：执行 HTTP 请求并解析响应

**实现要点**：
- 检查 `rocket.radar` 是否存在
- 检查 `rocket.direction` 是否为 `NoHttpRequestDirection`
- 获取全局 HTTP 客户端
- 执行请求，存入 `rocket.destination_origin`
- 根据 `rocket.direction` 解析响应
- 存入 `rocket.destination`
- 调用 `next.call(rocket).await`

```rust
pub struct ParserPlugin;

#[async_trait]
impl Plugin for ParserPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // 不发起请求
        if rocket.direction == DirectionKind::NoHttpRequestDirection {
            next.call(rocket).await;
            return;
        }
        
        // 检查 radar
        if rocket.radar.is_none() {
            next.call(rocket).await;
            return;
        }
        
        let client = get_client();
        let request = rocket.radar.take().unwrap();
        
        // 发送请求
        let response = client.execute(request).await?;
        rocket.destination_origin = Some(response);
        
        // 解析响应
        match &rocket.direction {
            DirectionKind::CollectionDirection => {
                let direction = CollectionDirection;
                rocket.destination = Some(direction.parse(rocket).await?);
            }
            DirectionKind::ResponseDirection => {
                // Response 不能 Clone，需要特殊处理
                let response = rocket.destination_origin.take().unwrap();
                rocket.destination = Some(Destination::Response(response));
            }
            DirectionKind::OriginResponseDirection => {
                if rocket.config.return_rocket {
                    rocket.destination = Some(Destination::Rocket(
                        Box::new(rocket.clone())
                    ));
                }
            }
            DirectionKind::Custom(d) => {
                rocket.destination = Some(d.parse(rocket).await?);
            }
            _ => {}
        }
        
        next.call(rocket).await;
    }
}
```

### 4.5 LogPlugin

**功能**：日志记录

**实现要点**：
- 前向：记录请求开始日志（URL、method、payload）
- 调用 `next.call(rocket).await`
- 后向：记录响应日志（状态码、耗时）

```rust
use std::time::Instant;

pub struct LogPlugin;

#[async_trait]
impl Plugin for LogPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        let start = Instant::now();
        
        tracing::info!(
            method = rocket.config.method.as_str(),
            url = &rocket.config.url,
            "Request started"
        );
        
        next.call(rocket).await;
        
        let elapsed = start.elapsed();
        
        if let Some(resp) = &rocket.destination_origin {
            tracing::info!(
                status = resp.status().as_u16(),
                elapsed_ms = elapsed.as_millis(),
                "Request completed"
            );
        }
    }
}
```

## 五、HTTP 客户端设计

### 5.1 全局单例客户端

```rust
use std::sync::OnceLock;
use std::time::Duration;

/// 全局 HTTP 客户端单例（共享连接池）
pub fn get_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_idle_timeout(Some(Duration::from_secs(90)))
            .pool_max_idle_per_host(usize::MAX)
            .build()
            .expect("Failed to create HTTP client")
    })
}
```

**设计说明**：
- reqwest::Client 内部维护连接池（hyper 管理）
- Client 配置（timeout、headers）构建时固定，不可修改
- Per-request timeout 通过 `RocketConfig.http` 设置
- 全局单例共享连接池，Clone Client 共享连接池
- Config 与 Client 解耦，HTTP 配置通过 request-level 设置

## 六、Artful 主入口设计

```rust
use std::sync::Arc;

pub struct Artful;

impl Artful {
    /// 执行插件链
    pub async fn artful(
        config: RocketConfig,
        payload: HashMap<String, Value>,
        plugins: Vec<Arc<dyn Plugin>>,
    ) -> Result<Destination> {
        let mut rocket = Rocket::new(config, payload);
        let mut ctrl = FlowCtrl::new(plugins);
        ctrl.call_next(&mut rocket).await;
        Ok(rocket.destination.unwrap_or_default())
    }
    
    /// 使用 Shortcut 快捷方式
    pub async fn shortcut<S: Shortcut + Default>(
        config: RocketConfig,
        payload: HashMap<String, Value>,
    ) -> Result<Destination> {
        let shortcut = S::default();
        let plugins = shortcut.get_plugins(&config, &payload);
        Self::artful(config, payload, plugins).await
    }
    
    /// 直接调用 HTTP（跳过插件链）
    pub async fn raw(request: reqwest::Request) -> Result<reqwest::Response> {
        let client = get_client();
        client.execute(request).await.map_err(ArtfulError::RequestFailed)
    }
}
```

## 七、Config 配置设计

```rust
/// 框架配置（非 HTTP 配置）
#[derive(Debug, Clone)]
pub struct Config {
    /// 日志配置
    pub logger: LoggerConfig,
    
    /// 默认解析策略
    pub default_direction: DirectionKind,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logger: LoggerConfig::default(),
            default_direction: DirectionKind::CollectionDirection,
        }
    }
}

/// 日志配置
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// 是否启用日志
    pub enable: bool,
    
    /// 日志级别
    pub level: String,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            enable: true,
            level: "info".to_string(),
        }
    }
}
```

## 八、使用示例

### 8.1 基础使用

```rust
use artful::{Artful, RocketConfig, Method};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use std::sync::Arc;
use serde_json::json;

let config = RocketConfig {
    method: Method::POST,
    url: "https://api.example.com/orders".to_string(),
    headers: HashMap::from([
        ("Authorization".to_string(), "Bearer token".to_string()),
    ]),
    http: HttpOptions {
        timeout: Some(30),
        ..Default::default()
    },
    ..Default::default()
};

let payload = HashMap::from([
    ("order_id", json!("123")),
    ("amount", json!("100")),
]);

let plugins: Vec<Arc<dyn Plugin>> = vec![
    Arc::new(StartPlugin),
    Arc::new(AddPayloadBodyPlugin),
    Arc::new(AddRadarPlugin),
    Arc::new(ParserPlugin),
];

let result = Artful::artful(config, payload, plugins).await?;

if let Destination::Collection(json) = result {
    println!("Response: {}", json);
}
```

### 8.2 使用 Shortcut

```rust
use artful::{Artful, Shortcut, RocketConfig, Method};

#[derive(Default)]
struct QueryOrderShortcut;

impl Shortcut for QueryOrderShortcut {
    fn get_plugins(&self, config: &RocketConfig, payload: &HashMap<String, Value>) 
        -> Vec<Arc<dyn Plugin>> 
    {
        vec![
            Arc::new(StartPlugin),
            Arc::new(AddPayloadBodyPlugin),
            Arc::new(AddRadarPlugin),
            Arc::new(ParserPlugin),
        ]
    }
}

let result = Artful::shortcut::<QueryOrderShortcut>(
    RocketConfig {
        method: Method::GET,
        url: "https://api.example.com/orders/123".to_string(),
        ..Default::default()
    },
    HashMap::new(),
).await?;
```

### 8.3 自定义插件

```rust
use artful::{Plugin, Rocket, Next};

/// 签名插件
pub struct SignaturePlugin {
    api_key: String,
}

#[async_trait]
impl Plugin for SignaturePlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // 前向：生成签名
        let payload_json = rocket.packer.pack(&rocket.payload)?;
        let signature = sign(&payload_json, &self.api_key);
        
        rocket.config.headers.insert("X-Signature".to_string(), signature);
        
        next.call(rocket).await;
        
        // 后向：验签（可选）
        if !rocket.is_ceased() {
            if let Some(resp) = &rocket.destination_origin {
                // 验证响应签名
            }
        }
    }
}
```

## 九、测试策略

### 9.1 单元测试

- **RocketConfig**：默认值、字段设置
- **Rocket**：创建、字段访问
- **FlowCtrl**：cursor 移动、cease/skip_rest、has_next
- **Plugin**：trait 实现验证
- **Direction**：各种解析策略
- **Packer**：序列化/反序列化

### 9.2 插件测试

- **StartPlugin**：基本流程
- **AddPayloadBodyPlugin**：body 生成逻辑
- **AddRadarPlugin**：Request 构建正确性、timeout 应用
- **ParserPlugin**：HTTP 执行 + Direction 解析
- **LogPlugin**：日志输出验证

### 9.3 集成测试

使用 wiremock 模拟 HTTP 服务器：

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_full_pipeline() {
    let mock_server = MockServer::start();
    
    Mock::given(method("POST"))
        .and(path("/api/orders"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({"code": 0, "data": "success"})))
        .mount(&mock_server)
        .await;
    
    let config = RocketConfig {
        method: Method::POST,
        url: mock_server.uri() + "/api/orders",
        ..Default::default()
    };
    
    let plugins = vec![
        Arc::new(StartPlugin),
        Arc::new(AddPayloadBodyPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];
    
    let result = Artful::artful(config, HashMap::new(), plugins).await?;
    
    assert!(matches!(result, Destination::Collection(_)));
}
```

## 十、依赖版本

```toml
[package]
name = "artful"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
description = "Api RequesT Framework U Like - 你喜欢的 Rust API 请求框架"
license = "MIT"
repository = "https://github.com/yansongda/artful-rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
thiserror = "2"

[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.6"
```

## 十一、实现顺序

### Phase 1：骨架搭建

1. Cargo.toml
2. src/lib.rs
3. src/error.rs
4. src/config.rs
5. src/http.rs
6. src/packer.rs + src/packers/

### Phase 2：核心类型

1. src/rocket.rs（RocketConfig, Method, HttpOptions, Rocket）
2. src/direction.rs + src/directions/
3. src/plugin.rs
4. src/flow_ctrl.rs
5. src/shortcut.rs

### Phase 3：主入口

1. src/artful.rs

### Phase 4：内置插件

1. src/plugins/mod.rs
2. StartPlugin
3. AddPayloadBodyPlugin
4. AddRadarPlugin
5. ParserPlugin
6. LogPlugin

### Phase 5：测试

1. tests/ 所有测试文件
2. cargo test

### Phase 6：文档

1. README.md
2. cargo fmt

## 十二、质量保证

- cargo fmt 格式化所有代码
- cargo check 确保 zero warnings
- cargo test 确保全部通过
- cargo doc 确保 public API 文档完整

## 十三、设计决策记录

| 决策 | 原因 |
|------|------|
| HTTP Client 全局单例 | reqwest Client 连接池 per-instance，Clone 共享连接池 |
| Config 与 Client 解耦 | Client 配置构建时固定，per-request timeout 通过 RocketConfig 设置 |
| RocketConfig struct | 类型安全 + IDE 类型提示 |
| payload 直接用 HashMap | 简化设计，payload 本身足够灵活 |
| 移除 state | payload 可承载插件间数据共享 |
| 移除便捷方法 | 保持 API 简洁 |
| API 入口用 RocketConfig | Rust 最佳实践，而非完全模仿 PHP HashMap |