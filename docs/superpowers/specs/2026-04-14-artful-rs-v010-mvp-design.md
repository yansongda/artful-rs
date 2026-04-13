# Artful-Rs v0.1.0 MVP 实现规范

> 基于 docs/ARCHITECTURE.md 架构设计文档的 v0.1.0 MVP 实现详细规范

## 一、实现范围

### 1.1 核心架构模块

| 模块文件 | 功能 | 主要类型 |
|---------|------|---------|
| `src/lib.rs` | 框架入口，导出公共 API | - |
| `src/artful.rs` | 主入口 + 全局配置单例 | `Artful` struct |
| `src/rocket.rs` | 请求载体 | `Rocket` struct |
| `src/flow_ctrl.rs` | 流向控制器 | `FlowCtrl` struct, `Next` struct |
| `src/plugin.rs` | 插件 trait 定义 | `Plugin` trait |
| `src/config.rs` | 配置管理 | `Config`, `HttpConfig`, `LoggerConfig` |
| `src/error.rs` | 错误定义 | `ArtfulError` enum |
| `src/payload.rs` | 有效载荷 | `Payload` struct |
| `src/http.rs` | HTTP 客户端封装 | reqwest wrapper |
| `src/shortcut.rs` | 快捷方式 trait | `Shortcut` trait |

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
| `tests/rocket_test.rs` | Rocket 创建、参数传递、payload 操作 |
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
#[derive(Debug, thiserror::Error)]
pub enum ArtfulError {
    // 配置错误
    #[error("配置已初始化，无法重复设置")]
    ConfigAlreadySet,
    
    #[error("配置未初始化")]
    ConfigNotSet,
    
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
```

### 2.2 Result 类型别名

```rust
pub type Result<T> = std::result::Result<T, ArtfulError>;
```

## 三、核心类型设计

### 3.1 Rocket - 请求载体

```rust
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

/// 请求载体 - 携带整个请求生命周期中的所有数据
pub struct Rocket {
    /// 原始入参（不变）
    pub params: HashMap<String, Value>,
    
    /// 处理后的有效载荷（插件可修改）
    pub payload: Payload,
    
    /// HTTP 请求对象（最终发送的请求）
    pub radar: Option<reqwest::Request>,
    
    /// HTTP 原始响应
    pub destination_origin: Option<reqwest::Response>,
    
    /// 最终解析结果
    pub destination: Option<Destination>,
    
    /// 响应解析策略
    pub direction: DirectionKind,
    
    /// 序列化器
    pub packer: Arc<dyn Packer>,
    
    /// 共享状态（用于插件间传递自定义数据）
    pub state: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}
```

**Rocket 核心方法**：
- `new(params: HashMap<String, Value>) -> Self` - 创建 Rocket
- `is_ceased() -> bool` - 检查是否已终止
- `cease()` - 终止流程

### 3.2 Payload - 有效载荷

```rust
use std::collections::HashMap;
use serde_json::Value;

/// 有效载荷 - 请求参数的动态集合
pub struct Payload {
    pub data: HashMap<String, Value>,
}

impl Payload {
    /// 创建空 payload
    pub fn new() -> Self;
    
    /// 设置值
    pub fn set(&mut self, key: String, value: Value);
    
    /// 获取值
    pub fn get(&self, key: &str) -> Option<&Value>;
    
    /// 移除值
    pub fn remove(&mut self, key: &str) -> Option<Value>;
    
    /// 合并数据
    pub fn merge(&mut self, data: HashMap<String, Value>);
    
    /// 转 JSON
    pub fn to_json(&self) -> Value;
    
    /// 过滤特殊参数（_开头的）
    pub fn filter_special(&self) -> HashMap<String, Value>;
}
```

### 3.3 FlowCtrl - 流向控制器

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
    pub fn new(plugins: Vec<Arc<dyn Plugin>>) -> Self;
    
    /// 调用下一层插件（洋葱穿透）
    pub async fn call_next(&mut self, rocket: &mut Rocket);
    
    /// 检查是否还有下一层
    pub fn has_next(&self) -> bool;
    
    /// 跳过剩余所有插件
    pub fn skip_rest(&mut self);
    
    /// 终止并标记
    pub fn cease(&mut self);
    
    /// 检查是否已终止
    pub fn is_ceased(&self) -> bool;
}
```

### 3.4 Next - 闭包穿透

```rust
/// 下一个插件的闭包（洋葱穿透）
pub struct Next<'a> {
    ctrl: &'a mut FlowCtrl,
}

impl<'a> Next<'a> {
    /// 调用下一个插件
    pub async fn call(self, rocket: &mut Rocket);
}
```

### 3.5 Plugin - 插件 trait

```rust
use async_trait::async_trait;

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
```

### 3.6 Direction - 响应解析器

```rust
use async_trait::async_trait;

/// 响应解析器 trait（对应 PHP Direction）
#[async_trait]
pub trait Direction: Send + Sync {
    /// 解析响应
    async fn parse(&self, rocket: &mut Rocket) -> Result<Destination>;
}

/// 响应解析策略（对应 PHP Direction）
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
#[derive(Clone, Debug)]
pub enum Destination {
    /// JSON Collection（默认）
    Collection(Value),
    /// 原始响应
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

### 3.7 Packer - 序列化器

```rust
use serde_json::Value;

/// 序列化器 trait（对应 PHP Packer）
pub trait Packer: Send + Sync {
    /// 序列化数据
    fn pack(&self, data: &HashMap<String, Value>) -> Result<String>;
    
    /// 反序列化数据
    fn unpack(&self, data: &str) -> Result<Value>;
}
```

### 3.8 Shortcut - 快捷方式

```rust
use std::sync::Arc;

/// 快捷方式 trait
pub trait Shortcut {
    /// 返回插件列表
    fn get_plugins(&self, params: &HashMap<String, Value>) -> Vec<Arc<dyn Plugin>>;
}
```

## 四、内置插件实现

### 4.1 StartPlugin

**功能**：将 params 合并入 payload（排除特殊参数）

**实现要点**：
- 遍历 `rocket.params`
- 过滤掉 `_` 开头的特殊参数
- 合并到 `rocket.payload.data`
- 调用 `next.call(rocket).await`

### 4.2 AddRadarPlugin

**功能**：构建 HTTP Request

**实现要点**：
- 从 payload 获取 `_method`（默认 POST）
- 从 payload 获取 `_url`（必须存在）
- 从 payload 获取 `_headers`
- 从 payload 获取 `_body` 或使用默认 JSON body
- 构建 `reqwest::Request`
- 存入 `rocket.radar`
- 调用 `next.call(rocket).await`

**特殊参数处理**：
- `_method` - HTTP 方法（GET/POST/PUT/DELETE 等）
- `_url` - 请求 URL（必填）
- `_headers` - 请求头（JSON object）
- `_body` - 请求体（字符串或 JSON）

### 4.3 ParserPlugin

**功能**：执行 HTTP 请求并解析响应

**实现要点**：
- 检查 `rocket.radar` 是否存在
- 获取全局 HTTP 客户端
- 执行请求，存入 `rocket.destination_origin`
- 根据 `rocket.direction` 解析响应：
  - `CollectionDirection` → 解析为 JSON
  - `ResponseDirection` → 返回原始 Response
  - `NoHttpRequestDirection` → 不发起请求
  - `OriginResponseDirection` → 返回 Rocket
  - `Custom` → 使用自定义解析器
- 存入 `rocket.destination`
- 调用 `next.call(rocket).await`

### 4.4 AddPayloadBodyPlugin

**功能**：将 payload.data 转换为请求体

**实现要点**：
- 检查 payload 中是否已有 `_body`
- 如果没有，将 `payload.filter_special()` 序列化
- 设置 `_body` 到 payload
- 调用 `next.call(rocket).await`

### 4.5 LogPlugin

**功能**：日志记录

**实现要点**：
- 前向：记录请求开始日志（URL、method、params）
- 调用 `next.call(rocket).await`
- 后向：记录响应日志（状态码、响应内容、耗时）

## 五、HTTP 客户端设计

### 5.1 全局单例客户端

```rust
use std::sync::OnceLock;
use reqwest::Client;

/// 全局 HTTP 客户端单例
static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

/// 获取或初始化 HTTP 客户端
pub fn get_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client")
    })
}

/// 根据配置初始化客户端
pub fn init_client(config: &HttpConfig) -> Result<()> {
    let mut builder = Client::builder();
    
    if let Some(timeout) = config.timeout {
        builder = builder.timeout(std::time::Duration::from_secs(timeout));
    }
    
    if let Some(connect_timeout) = config.connect_timeout {
        builder = builder.connect_timeout(std::time::Duration::from_secs(connect_timeout));
    }
    
    if let Some(proxy) = &config.proxy {
        builder = builder.proxy(reqwest::Proxy::all(proxy)?);
    }
    
    for (key, value) in &config.default_headers {
        builder = builder.default_header(key.as_str(), value.as_str());
    }
    
    let client = builder.build()?;
    
    // OnceLock 不支持强制覆盖，使用 get_or_init 的限制
    // 实际实现中需要考虑配置强制更新的场景
    
    Ok(())
}
```

### 5.2 HTTP 配置

```rust
pub struct HttpConfig {
    /// 请求超时（秒）
    pub timeout: Option<u64>,
    
    /// 连接超时（秒）
    pub connect_timeout: Option<u64>,
    
    /// 代理设置
    pub proxy: Option<String>,
    
    /// 默认请求头
    pub default_headers: HashMap<String, String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout: Some(30),
            connect_timeout: Some(10),
            proxy: None,
            default_headers: HashMap::new(),
        }
    }
}
```

## 六、Artful 主入口设计

### 6.1 全局配置单例

```rust
use std::sync::OnceLock;
use crate::config::Config;
use crate::error::ArtfulError;

/// 全局配置单例
static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Artful;

impl Artful {
    /// 初始化配置（单例）
    pub fn config(config: Config) -> Result<()> {
        CONFIG.set(config).map_err(|_| ArtfulError::ConfigAlreadySet)?;
        Ok(())
    }
    
    /// 获取配置
    pub fn get_config() -> Result<&Config> {
        CONFIG.get().ok_or(ArtfulError::ConfigNotSet)
    }
    
    /// 使用快捷方式执行请求
    pub async fn shortcut<S: Shortcut + Default>(
        params: HashMap<String, Value>,
    ) -> Result<Destination> {
        let shortcut = S::default();
        let plugins = shortcut.get_plugins(&params);
        Self::artful(plugins, params).await
    }
    
    /// 执行插件链
    pub async fn artful(
        plugins: Vec<Arc<dyn Plugin>>,
        params: HashMap<String, Value>,
    ) -> Result<Destination> {
        // 构建载体
        let mut rocket = Rocket::new(params);
        
        // 构建流向控制器
        let mut ctrl = FlowCtrl::new(plugins);
        
        // 启动洋葱流程
        ctrl.call_next(&mut rocket).await;
        
        // 返回结果
        Ok(rocket.destination.unwrap_or_default())
    }
    
    /// 直接调用 HTTP（跳过插件）
    pub async fn raw(request: reqwest::Request) -> Result<reqwest::Response> {
        let client = get_client();
        client.execute(request).await.map_err(ArtfulError::RequestFailed)
    }
}
```

## 七、测试策略

### 7.1 单元测试

每个核心模块的单元测试覆盖：

- **Rocket**：创建、参数传递、payload 操作、cease 状态
- **Payload**：set/get/remove/merge/filter_special
- **FlowCtrl**：cursor 移动、cease/skip_rest、has_next
- **Plugin**：trait 实现验证
- **Direction**：各种解析策略
- **Packer**：序列化/反序列化

### 7.2 插件测试

每个内置插件的独立测试：

- **StartPlugin**：params → payload 合并逻辑
- **AddRadarPlugin**：Request 构建正确性
- **ParserPlugin**：HTTP 执行 + Direction 解析
- **AddPayloadBodyPlugin**：body 生成逻辑
- **LogPlugin**：日志输出验证

### 7.3 集成测试

完整插件链流程测试：

- 基础插件链 `[Start, AddPayloadBody, AddRadar, Parser]`
- 自定义插件插入测试
- Shortcut 快捷方式测试
- 错误处理流程测试

### 7.4 测试辅助工具

使用 `wiremock` crate 模拟 HTTP 服务器：

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

// 测试中创建 mock server
let mock_server = MockServer::start();
mock_server.mount(Mock::given(method("POST"))
    .and(path("/api/test"))
    .respond_with(ResponseTemplate::new(200)
        .set_body_json(serde_json::json!({"code": 0, "data": "success"}))));
```

## 八、依赖版本

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

## 九、实现顺序

### 9.1 骨架搭建（Phase 1）

1. 创建 Cargo.toml
2. 创建 src/lib.rs（导出 API）
3. 创建 src/error.rs（错误类型）
4. 创建 src/config.rs（配置类型）
5. 创建 src/payload.rs（Payload）
6. 创建 src/rocket.rs（Rocket）
7. 创建 src/packer.rs + src/packers/（Packer）

### 9.2 核心架构（Phase 2）

1. 创建 src/direction.rs + src/directions/（Direction）
2. 创建 src/plugin.rs（Plugin trait）
3. 创建 src/flow_ctrl.rs（FlowCtrl + Next）
4. 创建 src/http.rs（HTTP 客户端）
5. 创建 src/shortcut.rs（Shortcut trait）
6. 创建 src/artful.rs（主入口）

### 9.3 内置插件（Phase 3）

1. 创建 src/plugins/mod.rs
2. 实现 StartPlugin
3. 实现 AddPayloadBodyPlugin
4. 实现 AddRadarPlugin
5. 实现 ParserPlugin
6. 实现 LogPlugin

### 9.4 测试覆盖（Phase 4）

1. 创建 tests/ 目录
2. 编写单元测试
3. 编写插件测试
4. 编写集成测试
5. 运行 cargo test 确保全部通过

### 9.5 文档完善（Phase 5）

1. 完善 README.md
2. 添加使用示例
3. cargo fmt 格式化所有代码

## 十、质量保证

### 10.1 代码格式化

- 所有代码经 `cargo fmt` 格式化
- 使用 rustfmt 默认配置

### 10.2 编译检查

- 实现 Phase 后运行 `cargo check`
- 确保 zero warnings（无编译警告）

### 10.3 测试验证

- 每个 Phase 后运行相关测试
- 最终运行 `cargo test --all` 确保全部通过

### 10.4 文档验证

- 运行 `cargo doc` 生成文档
- 确保 public API 文档完整

## 十一、后续迭代预留

v0.1.0 MVP 完成后，后续迭代扩展：

- **v0.2.0**：事件系统、更多内置插件、错误处理增强
- **v0.3.0**：支付宝支付插件包、微信支付插件包、XML Packer

架构设计已为后续迭代预留扩展点：
- Direction trait 支持自定义解析器
- Plugin trait 支持任意插件实现
- Shortcut trait 支持快捷方式扩展
- Packer trait 支持多种序列化格式