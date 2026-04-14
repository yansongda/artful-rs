# Artful-Rs 架构设计文档

> Api RequesT Framework U Like - 你喜欢的 Rust API 请求框架
> 
> 基于 [yansongda/artful](https://github.com/yansongda/artful) 的架构理念，使用 Rust 实现的 HTTP 客户端框架

## 一、设计理念

### 1.1 核心原则

- **洋葱模型**: 所有请求处理通过 Pipeline（插件链）实现，请求层层穿透，响应层层返回
- **插件化**: 每个请求都是一个插件组合，高度灵活可定制
- **类型安全**: 使用 Rust 类型系统确保配置和参数的类型安全
- **符合标准**: 遵循 Rust async/await 最佳实践

### 1.2 与 PHP 版本的对比

| 特性 | PHP (yansongda/artful) | Rust (artful-rs) |
|------|------------------------|------------------|
| 洋葱模型 | Pipeline + Closure | FlowCtrl + async |
| 数据载体 | Rocket | Rocket |
| 配置参数 | `_` 开头参数在 HashMap | RocketConfig struct（类型安全） |
| 插件 | PluginInterface | Plugin trait |
| HTTP 客户端 | Guzzle | reqwest |
| 序列化 | JsonPacker | serde_json |
| 类型系统 | 动态类型 | 静态类型 + 泛型 |

---

## 二、核心概念

### 2.1 Rocket - 请求载体

Rocket 是整个请求生命周期中的数据载体。

```rust
/// 请求载体 - 携带整个请求生命周期中的所有数据
pub struct Rocket {
    /// 原始参数（不变）
    params: HashMap<String, Value>,
    
    /// 业务参数（动态）
    pub payload: HashMap<String, Value>,
    
    /// Rocket 配置（可修改）
    pub config: RocketConfig,
    
    /// HTTP 请求对象（最终发送的请求）
    pub radar: Option<reqwest::Request>,
    
    /// HTTP 原始响应
    pub destination_origin: Option<reqwest::Response>,
    
    /// 最终解析结果
    pub destination: Option<Destination>,
    
    /// 序列化器
    pub packer: Arc<dyn Packer>,
}
```

**设计说明**：
- `params` - 原始参数，整个生命周期中保持不变
- `payload` - 业务参数，动态 HashMap
- `config` - 请求配置，包含 method、url、headers、direction 等
- `radar` - 最终构建的 HTTP Request
- `destination_origin` - HTTP 响应
- `destination` - 解析后的结果
- `packer` - 序列化器

### 2.2 RocketConfig - 配置参数

RocketConfig 将配置参数封装为 struct，提供类型安全和 IDE 类型提示。所有字段可在 plugin 中动态修改。

```rust
/// Rocket 配置（所有字段可在 plugin 中动态修改）
#[derive(Debug, Clone)]
pub struct RocketConfig {
    /// HTTP 方法（默认 POST，可动态修改）
    pub method: reqwest::Method,
    
    /// 请求 URL（必填，可动态修改，如添加 query 参数）
    pub url: String,
    
    /// 请求头（可动态添加/修改）
    pub headers: HashMap<String, String>,
    
    /// 请求体（可动态设置）
    pub body: Option<String>,
    
    /// HTTP 选项（可动态修改）
    pub http: HttpOptions,
    
    /// 是否返回 Rocket（调试用，可动态设置）
    pub return_rocket: bool,
    
    /// 响应解析策略（默认 JsonDirection，可动态修改）
    pub direction: DirectionKind,
}

/// HTTP 请求选项
#[derive(Debug, Clone, Default)]
pub struct HttpOptions {
    /// 请求超时（秒）
    pub timeout: Option<u64>,
    
    /// 连接超时（秒）
    pub connect_timeout: Option<u64>,
}
```

**与 PHP 版本的对应关系**：

| PHP `_` 参数 | Rust RocketConfig 字段 |
|-------------|----------------------|
| `_method` | `config.method` |
| `_url` | `config.url` |
| `_headers` | `config.headers` |
| `_body` | `config.body` |
| `_http.timeout` | `config.http.timeout` |
| `_return_rocket` | `config.return_rocket` |
| `_direction` | `config.direction` |

**优势**：
- 类型安全：字段类型明确，编译时检查
- IDE 类型提示：自动补全、类型提示
- 清晰分离：配置参数与业务参数分离
- 可扩展：添加新配置只需修改 RocketConfig

### 2.3 Config - 全局框架配置

Config 是框架级别的全局配置，通过 `Artful::config()` 初始化，支持任意扩展参数。

```rust
/// 框架全局配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 是否强制覆盖已存在的配置
    pub _force: bool,
    
    /// 日志配置
    pub logger: LoggerConfig,
    
    /// HTTP 默认选项
    pub http: HttpOptions,
    
    /// 扩展配置：支持任意渠道/模块参数
    pub extra: HashMap<String, Value>,
}

/// 日志配置
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub enable: bool,
    pub level: String,
}
```

**extra 字段用途**：
- 存储任意渠道配置（如支付宝、微信支付配置）
- 支持动态扩展，无需修改 Config 结构
- 与 PHP 版本的灵活配置模式兼容

**使用示例**：

```rust
use artful::{Artful, Config, HttpOptions};
use serde_json::json;
use std::collections::HashMap;

let mut extra = HashMap::new();
extra.insert("alipay".to_string(), json!({
    "app_id": "2016082000295641",
    "app_secret_cert": "...",
}));
extra.insert("wechat".to_string(), json!({
    "mch_id": "...",
    "mch_secret_key": "...",
}));

let config = Config {
    http: HttpOptions {
        timeout: Some(5),
        connect_timeout: Some(3),
    },
    extra,
    ..Default::default()
};

Artful::config(config);

// 后续获取配置
let global_config = Artful::get_config();
if let Some(alipay) = global_config.extra.get("alipay") {
    let app_id = alipay.get("app_id");
}
```

### 2.4 Plugin - 插件

插件是洋葱模型的核心。

```rust
/// 插件 trait - 洋葱模型核心
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    /// 组装请求
    /// 
    /// # Arguments
    /// * `rocket` - 请求载体，包含所有数据
    /// * `next` - 下一个插件（闭包）
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>);
}

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

**插件编写模式**:

```rust
pub struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // ===== 前向逻辑 =====
        // 修改 config、payload 等
        
        rocket.config.headers.insert(
            "Authorization".to_string(),
            "Bearer token".to_string()
        );
        
        // ===== 调用下一层 =====
        next.call(rocket).await;
        
        // ===== 后向逻辑 =====
        // 处理响应、错误处理等
        if let Some(resp) = rocket.destination_origin.as_ref() {
            // 检查响应状态码等
        }
    }
}
```

### 2.5 FlowCtrl - 流向控制器

FlowCtrl 控制洋葱模型的执行流程，借鉴 [salvo](https://github.com/salvo-rs/salvo) 的设计。

```rust
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

**执行流程示意**:

```
插件列表: [Start, Sign, AddRadar, Parser]

执行顺序（洋葱模型）:
┌─────────────────────────────────────────────────────────┐
│ Start.assembly()                                        │
│   ├─ 前向逻辑: 初始化                                    │
│   ├─ next.call()                                        │
│   │   └─────────────────────────────────────────────────│
│   │   │ Sign.assembly()                                 │
│   │   │   ├─ 前向逻辑: 添加签名                          │
│   │   │   ├─ next.call()                                │
│   │   │   │   └─────────────────────────────────────────│
│   │   │   │   │ AddRadar.assembly()                     │
│   │   │   │   │   ├─ 前向逻辑: 构建 Request              │
│   │   │   │   │   ├─ next.call()                        │
│   │   │   │   │   │   ┌─────────────────────────────────│
│   │   │   │   │   │   │ Parser.assembly()               │
│   │   │   │   │   │   │   ├─ 前向: 无                   │
│   │   │   │   │   │   │   ├─ HTTP 请求执行              │
│   │   │   │   │   │   │   ├─ 后向: 解析响应             │
│   │   │   │   │   │   └─────────────────────────────────│
│   │   │   │   │   ├─ 后向逻辑: 无                       │
│   │   │   │   └─────────────────────────────────────────│
│   │   │   ├─ 后向逻辑: 验签（可选）                      │
│   │   │   └─────────────────────────────────────────────│
│   │   └─────────────────────────────────────────────────│
│   ├─ 后向逻辑: 日志记录等                                │
│   └─────────────────────────────────────────────────────┘
```

### 2.6 Shortcut - 快捷方式

Shortcut 是一系列插件的组合，方便快速调用特定 API。

```rust
/// 快捷方式 trait
pub trait Shortcut {
    /// 返回插件列表
    fn get_plugins(&self, params: &HashMap<String, Value>) 
        -> Vec<Arc<dyn Plugin>>;
}

// 示例实现
#[derive(Default)]
pub struct QueryOrderShortcut;

impl Shortcut for QueryOrderShortcut {
    fn get_plugins(&self, params: &HashMap<String, Value>) 
        -> Vec<Arc<dyn Plugin>> 
    {
        vec![
            Arc::new(StartPlugin),
            Arc::new(QueryOrderPlugin),  // 业务插件
            Arc::new(AddSignaturePlugin),
            Arc::new(AddRadarPlugin),
            Arc::new(ParserPlugin),
        ]
    }
}
```

---

## 三、核心模块设计

### 3.1 Artful - 主入口

```rust
/// Artful 主类 - 框架入口
pub struct Artful;

impl Artful {
    /// 初始化框架全局配置
    pub fn config(config: Config) -> bool {
        // 首次调用时设置配置，后续调用返回 false（除非 config._force = true）
    }
    
    /// 获取全局配置
    pub fn get_config() -> &'static Config;
    
    /// 检查是否已初始化配置
    pub fn has_config() -> bool;
    
    /// 执行插件链
    pub async fn artful(
        params: HashMap<String, Value>,
        plugins: Vec<Arc<dyn Plugin>>,
    ) -> Result<Destination> {
        // 构建载体（params 存储原始参数，payload 初始为空）
        let mut rocket = Rocket::new(params);
        
        // 构建流向控制器
        let mut ctrl = FlowCtrl::new(plugins);
        
        // 启动洋葱流程
        ctrl.call_next(&mut rocket).await;
        
        // 返回结果
        Ok(rocket.destination.unwrap_or_default())
    }
    
    /// 使用快捷方式执行请求
    pub async fn shortcut<S: Shortcut + Default>(
        params: HashMap<String, Value>,
    ) -> Result<Destination> {
        let shortcut = S::default();
        let plugins = shortcut.get_plugins(&params);
        Self::artful(params, plugins).await
    }
    
    /// 直接调用 HTTP（跳过插件）
    pub async fn raw(request: reqwest::Request) -> Result<reqwest::Response> {
        let client = get_client();
        client.execute(request).await.map_err(ArtfulError::RequestFailed)
    }
}
```

### 3.2 HTTP 客户端设计

**核心设计决策**：HTTP Client 与 Config 解耦

**原因**：
- reqwest::Client 内部维护连接池（hyper 管理），per-instance
- Client 配置（timeout、headers、proxy）构建时固定，不可修改
- Per-request timeout 通过 `RocketConfig.http` 设置
- 全局单例共享连接池，性能最优

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

**Per-request timeout 应用**（在 AddRadarPlugin 中）：

```rust
// 应用 timeout
if let Some(timeout) = rocket.config.http.timeout {
    request_builder = request_builder.timeout(
        Duration::from_secs(timeout)
    );
}
```

### 3.3 Direction - 响应解析器

```rust
/// 响应解析器 trait
#[async_trait]
pub trait Direction: Send + Sync {
    /// 解析响应
    async fn parse(&self, rocket: &mut Rocket) -> Result<Destination>;
}

/// 响应解析策略
#[derive(Clone)]
pub enum DirectionKind {
    /// 解析为 JSON（默认）
    JsonDirection,
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
    /// JSON 值（默认）
    Json(Value),
    /// 原始响应
    Response(reqwest::Response),
    /// Rocket 本身（用于调试）
    Rocket(Box<Rocket>),
    /// 空结果
    None,
}
```

---

## 四、内置插件

### 4.1 StartPlugin - 初始化

```rust
/// 初始化插件
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

### 4.2 AddPayloadBodyPlugin - 添加请求体

```rust
/// 添加 payload body 插件
pub struct AddPayloadBodyPlugin;

#[async_trait]
impl Plugin for AddPayloadBodyPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // 如果未手动指定 body，将 payload 序列化为 JSON
        if rocket.config.body.is_none() && !rocket.payload.is_empty() {
            let body = rocket.packer.pack(&rocket.payload)?;
            rocket.config.body = Some(body);
        }
        
        next.call(rocket).await;
    }
}
```

### 4.3 AddRadarPlugin - 构建 HTTP 请求

```rust
/// 构建 HTTP Request 插件
pub struct AddRadarPlugin;

#[async_trait]
impl Plugin for AddRadarPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        let method = rocket.config.method;
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
        
        // 应用 timeout（per-request）
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

### 4.4 ParserPlugin - 解析响应

```rust
/// 解析响应插件 - 执行 HTTP 请求并解析结果
pub struct ParserPlugin;

#[async_trait]
impl Plugin for ParserPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // 不发起请求
        if rocket.config.direction == DirectionKind::NoHttpRequestDirection {
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
        match &rocket.config.direction {
            DirectionKind::JsonDirection => {
                let direction = JsonDirection;
                rocket.destination = Some(direction.parse(rocket).await?);
            }
            DirectionKind::ResponseDirection => {
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

---

## 五、使用示例

### 5.1 初始化框架

```rust
use artful::{Artful, Config};

// 初始化框架配置（可选）
// config._force = true 时强制覆盖已存在的配置
Artful::config(Config::default());
```

### 5.2 基础使用

```rust
use artful::{Artful, Plugin, Rocket, flow_ctrl::Next};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::json;

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

#[tokio::main]
async fn main() -> artful::Result<()> {
    let params = HashMap::from([
        ("order_id", json!("123")),
        ("amount", json!(100)),
    ]);

    let plugins: Vec<Arc<dyn artful::Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(MethodUrlPlugin {
            method: reqwest::Method::POST,
            url: "https://api.example.com/orders".to_string(),
        }),
        Arc::new(AddPayloadBodyPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(params, plugins).await?;
    
    if let artful::Destination::Json(json) = result {
        println!("Response: {}", json);
    }

    Ok(())
}
```

### 5.3 使用 Shortcut 快捷方式

```rust
use artful::{Artful, Shortcut, Plugin};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Default)]
struct MyApiShortcut {
    method: reqwest::Method,
    url: String,
}

impl Shortcut for MyApiShortcut {
    fn get_plugins(&self, _params: &HashMap<String, serde_json::Value>) 
        -> Vec<Arc<dyn Plugin>> 
    {
        vec![
            Arc::new(StartPlugin),
            Arc::new(MethodUrlPlugin {
                method: self.method.clone(),
                url: self.url.clone(),
            }),
            Arc::new(AddPayloadBodyPlugin),
            Arc::new(AddRadarPlugin),
            Arc::new(ParserPlugin),
        ]
    }
}

let result = Artful::shortcut::<MyApiShortcut>(HashMap::new()).await?;
```

### 5.4 自定义插件

```rust
use artful::{Plugin, Rocket, flow_ctrl::Next};
use async_trait::async_trait;

pub struct SignaturePlugin {
    api_key: String,
}

#[async_trait]
impl Plugin for SignaturePlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        rocket.config.headers.insert(
            "X-Signature".to_string(),
            sign(&self.api_key, &rocket.payload),
        );
        
        next.call(rocket).await;
    }
}
```

---

## 六、模块结构

采用 Rust 标准惯例：**Trait 定义放在对应模块顶层**。

```
rs-artful/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── .gitignore
├── src/
│   ├── lib.rs                  # 框架入口，导出公共 API
│   │
│   ├── artful.rs               # Artful 主入口
│   ├── rocket.rs               # Rocket + RocketConfig + HttpOptions
│   ├── flow_ctrl.rs            # FlowCtrl 流向控制 + Next 闭包
│   ├── config.rs               # Config + LoggerConfig
│   ├── error.rs                # ArtfulError 错误定义
│   │
│   ├── plugin.rs               # Plugin trait
│   ├── plugins/                # 内置插件实现
│   │   ├── mod.rs              # 导出所有内置插件
│   │   ├── start.rs            # StartPlugin
│   │   ├── add_radar.rs        # AddRadarPlugin
│   │   ├── parser.rs           # ParserPlugin
│   │   └── add_payload_body.rs # AddPayloadBodyPlugin
│   │
│   ├── shortcut.rs             # Shortcut trait
│   │
│   ├── direction.rs            # Direction trait + DirectionKind + Destination + JsonDirection
│   │
│   ├── packer.rs               # Packer trait
│   ├── packers/                # 内置 Packer 实现
│   │   ├── mod.rs              # 导出所有内置 Packer
│   │   └── json.rs             # JsonPacker
│   │
│   └── http.rs                 # HTTP 客户端封装（reqwest 单例）
│
├── examples/
│   ├── basic.rs                # 基础使用示例
│   ├── custom_plugin.rs        # 自定义插件示例
│   ├── config.rs               # 配置初始化示例
│   ├── shortcut.rs             # Shortcut 快捷方式示例
│   └── direction.rs            # Direction 响应解析策略示例
│
├── tests/
│   ├── artful_test.rs
│   ├── direction_test.rs
│   ├── flow_ctrl_test.rs
│   ├── integration_test.rs
│   ├── packer_test.rs
│   ├── rocket_test.rs
│   └── shortcut_test.rs
│
├── target/                     # 编译输出（gitignore）
│
└── docs/
    └── ARCHITECTURE.md         # 架构设计文档
```

### 模块说明

| 模块 | 说明 | Trait/类型 |
|------|------|-----------|
| `src/lib.rs` | 框架入口 | 导出公共 API |
| `src/artful.rs` | 主入口 | `Artful` struct |
| `src/rocket.rs` | 请求载体 + 配置 | `Rocket`, `RocketConfig`, `HttpOptions` |
| `src/flow_ctrl.rs` | 流向控制器 | `FlowCtrl`, `Next` |
| `src/config.rs` | 全局配置 | `Config`, `LoggerConfig` |
| `src/plugin.rs` | 插件 trait | `Plugin` trait |
| `src/plugins/` | 内置插件 | `StartPlugin`, `AddRadarPlugin`, `ParserPlugin`, `AddPayloadBodyPlugin` |
| `src/shortcut.rs` | 快捷方式 trait | `Shortcut` trait |
| `src/direction.rs` | 解析策略 | `Direction`, `DirectionKind`, `Destination`, `JsonDirection` |
| `src/packer.rs` | 序列化 trait | `Packer` trait |
| `src/packers/` | 内置序列化器 | `JsonPacker` |
| `src/http.rs` | HTTP 客户端 | reqwest 全局单例 |
| `src/error.rs` | 错误 | `ArtfulError` enum |

---

## 七、依赖设计

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

---

## 八、设计决策记录

| 决策 | 原因 |
|------|------|
| HTTP Client 全局单例 | reqwest Client 连接池 per-instance，Clone 共享连接池，全局单例性能最优 |
| Config 与 Client 解耦 | Client 配置构建时固定，per-request timeout 通过 RocketConfig.http 设置 |
| RocketConfig struct | 类型安全 + IDE 类型提示，而非 `_` 参数分散在 HashMap |
| RocketConfig 所有字段可修改 | plugin 动态修改配置（headers、body、url 等），靠约定管理 |
| 复用 reqwest::Method | 减少重复定义，与 reqwest API 直接兼容 |
| payload 直接用 HashMap | 简化设计，payload 本身足够灵活，无需额外 Payload struct |
| 移除 state 字段 | payload 可承载插件间数据共享，无需额外的 state HashMap |
| API 入口用 RocketConfig | Rust 最佳实践，而非完全模仿 PHP HashMap |

---

## 九、后续迭代规划

### v0.1.0 - MVP

- [x] 核心架构设计
- [x] 核心架构实现（Rocket, FlowCtrl, Plugin）
- [x] 内置插件（Start, AddPayloadBody, AddRadar, Parser, Log）
- [x] reqwest HTTP 客户端单例封装
- [x] JSON Packer
- [x] Direction 解析策略（JsonDirection, ResponseDirection 等）
- [x] Artful 主入口（artful, shortcut, raw 方法）
- [x] Shortcut trait
- [x] 基础测试覆盖（18 tests）
- [x] README 文档

### v0.2.0 - 增强

- [ ] 事件系统（类似 PHP 版本）
- [ ] 错误处理插件
- [ ] 更多内置插件（Retry、Cache 等）

### v0.3.0 - 生态

- [ ] 支付宝支付插件包 `artful-alipay`
- [ ] 微信支付插件包 `artful-wechat`
- [ ] XML Packer 支持（可选）

---

## 十、参考资源

- [yansongda/artful](https://github.com/yansongda/artful) - PHP 版本框架
- [salvo-rs/salvo](https://github.com/salvo-rs/salvo) - Rust Web 框架（洋葱模型参考）
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP 客户端（连接池设计参考）
- [tower](https://github.com/tower-rs/tower) - Rust Service 抽象（可选参考）