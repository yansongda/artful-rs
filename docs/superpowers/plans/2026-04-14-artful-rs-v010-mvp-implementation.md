# Artful-Rs v0.1.0 MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现 Artful-Rs v0.1.0 MVP - 一个基于洋葱模型的 Rust HTTP 客户端框架

**Architecture:** 洋葱模型（FlowCtrl + Plugin）+ 全局 HTTP Client 单例 + 类型安全的 RocketConfig

**Tech Stack:** Rust 1.85, tokio, async-trait, reqwest, serde, serde_json, tracing, thiserror

---

## File Structure

```
artful/
├── Cargo.toml                    # 项目配置
├── src/
│   ├── lib.rs                    # 框架入口，导出公共 API
│   ├── error.rs                  # ArtfulError enum
│   ├── config.rs                 # Config, LoggerConfig
│   ├── http.rs                   # HTTP 客户端单例
│   ├── packer.rs                 # Packer trait
│   ├── packers/
│   │   ├── mod.rs                # 导出 JsonPacker
│   │   └── json.rs               # JsonPacker 实现
│   ├── rocket.rs                 # Rocket, RocketConfig, HttpOptions
│   ├── direction.rs              # Direction trait, DirectionKind, Destination
│   ├── directions/
│   │   ├── mod.rs                # 导出内置 Direction
│   │   ├── collection.rs         # CollectionDirection
│   │   ├── response.rs           # ResponseDirection
│   │   ├── no_http.rs            # NoHttpRequestDirection
│   │   └── origin.rs             # OriginResponseDirection
│   ├── plugin.rs                 # Plugin trait
│   ├── flow_ctrl.rs              # FlowCtrl, Next
│   ├── shortcut.rs               # Shortcut trait
│   ├── artful.rs                 # Artful 主入口
│   └── plugins/
│       ├── mod.rs                # 导出内置插件
│       ├── start.rs              # StartPlugin
│       ├── add_payload_body.rs   # AddPayloadBodyPlugin
│       ├── add_radar.rs          # AddRadarPlugin
│       ├── parser.rs             # ParserPlugin
│       └── log.rs                # LogPlugin
├── tests/
│   ├── rocket_test.rs            # Rocket 测试
│   ├── flow_ctrl_test.rs         # FlowCtrl 测试
│   ├── plugin_test.rs            # Plugin 测试
│   ├── direction_test.rs         # Direction 测试
│   ├── packer_test.rs            # Packer 测试
│   ├── integration_test.rs       # 集成测试
│   ├── shortcut_test.rs          # Shortcut 测试
│   └── artful_test.rs            # Artful 主入口测试
└── README.md                     # 项目文档
```

---

## Phase 1: 骨架搭建

### Task 1: 创建 Cargo.toml

**Files:**
- Create: `Cargo.toml`

- [ ] **Step 1: 创建 Cargo.toml**

```toml
[package]
name = "artful"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
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

- [ ] **Step 2: 初始化项目结构**

```bash
cargo init --lib
```

Expected: 创建基本项目结构

- [ ] **Step 3: 验证项目可构建**

```bash
cargo check
```

Expected: 无错误（可能有未实现警告）

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml Cargo.lock src/lib.rs
git commit -m "feat: initialize artful project"
```

---

### Task 2: 实现 error.rs

**Files:**
- Create: `src/error.rs`
- Modify: `src/lib.rs:1-10`

- [ ] **Step 1: 创建 src/error.rs**

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

- [ ] **Step 2: 更新 src/lib.rs 导出 error**

```rust
pub mod error;

pub use error::{ArtfulError, Result};
```

- [ ] **Step 3: 验证编译**

```bash
cargo check
```

Expected: 无错误

- [ ] **Step 4: Commit**

```bash
git add src/error.rs src/lib.rs
git commit -m "feat: add error types"
```

---

### Task 3: 实现 config.rs

**Files:**
- Create: `src/config.rs`
- Modify: `src/lib.rs:1-10`

- [ ] **Step 1: 创建 src/config.rs**

```rust
use crate::direction::DirectionKind;

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

- [ ] **Step 2: 创建 src/direction.rs 占位**

由于 config.rs 依赖 DirectionKind，需要先创建 direction.rs 占位：

```rust
use std::sync::Arc;

/// 响应解析器 trait
#[async_trait::async_trait]
pub trait Direction: Send + Sync {
    /// 解析响应
    async fn parse(&self, rocket: &mut crate::Rocket) -> crate::Result<Destination>;
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
    Collection(serde_json::Value),
    /// 原始响应
    Response(reqwest::Response),
    /// Rocket 本身（用于调试）
    Rocket(Box<crate::Rocket>),
    /// 空结果
    None,
}

impl Default for Destination {
    fn default() -> Self {
        Destination::None
    }
}
```

- [ ] **Step 3: 创建 src/rocket.rs 占位**

由于 direction.rs 依赖 Rocket，需要先创建 rocket.rs 占位：

```rust
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;

use crate::direction::DirectionKind;
use crate::packer::Packer;
use crate::packers::JsonPacker;

/// Rocket 配置（所有字段可在 plugin 中动态修改）
#[derive(Debug, Clone)]
pub struct RocketConfig {
    /// HTTP 方法（默认 POST）
    pub method: reqwest::Method,
    
    /// 请求 URL（必填）
    pub url: String,
    
    /// 请求头
    pub headers: HashMap<String, String>,
    
    /// 请求体
    pub body: Option<String>,
    
    /// HTTP 选项
    pub http: HttpOptions,
    
    /// 是否返回 Rocket（调试用）
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

/// HTTP 请求选项
#[derive(Debug, Clone, Default)]
pub struct HttpOptions {
    /// 请求超时（秒）
    pub timeout: Option<u64>,
    
    /// 连接超时（秒）
    pub connect_timeout: Option<u64>,
}

/// 请求载体 - 携带整个请求生命周期中的所有数据
pub struct Rocket {
    /// Rocket 配置
    pub config: RocketConfig,
    
    /// 业务参数
    pub payload: HashMap<String, Value>,
    
    /// HTTP 请求对象
    pub radar: Option<reqwest::Request>,
    
    /// HTTP 原始响应
    pub destination_origin: Option<reqwest::Response>,
    
    /// 最终解析结果
    pub destination: Option<crate::direction::Destination>,
    
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
}
```

- [ ] **Step 4: 创建 src/packer.rs 占位**

```rust
use std::collections::HashMap;
use serde_json::Value;

use crate::Result;

/// 序列化器 trait
pub trait Packer: Send + Sync {
    /// 序列化数据
    fn pack(&self, data: &HashMap<String, Value>) -> Result<String>;
    
    /// 反序列化数据
    fn unpack(&self, data: &str) -> Result<Value>;
}
```

- [ ] **Step 5: 创建 src/packers/mod.rs 和 json.rs**

`src/packers/mod.rs`:

```rust
mod json;

pub use json::JsonPacker;
```

`src/packers/json.rs`:

```rust
use std::collections::HashMap;
use serde_json::Value;

use crate::packer::Packer;
use crate::Result;

/// JSON 序列化器
pub struct JsonPacker;

impl Packer for JsonPacker {
    fn pack(&self, data: &HashMap<String, Value>) -> Result<String> {
        serde_json::to_string(data).map_err(Into::into)
    }
    
    fn unpack(&self, data: &str) -> Result<Value> {
        serde_json::from_str(data).map_err(|e| crate::error::ArtfulError::JsonDeserializeError {
            message: e.to_string(),
        })
    }
}
```

- [ ] **Step 6: 更新 src/lib.rs 导出所有模块**

```rust
pub mod error;
pub mod direction;
pub mod rocket;
pub mod packer;
pub mod packers;
pub mod config;

pub use error::{ArtfulError, Result};
pub use direction::{Direction, DirectionKind, Destination};
pub use rocket::{Rocket, RocketConfig, HttpOptions};
pub use packer::Packer;
pub use packers::JsonPacker;
pub use config::{Config, LoggerConfig};
```

- [ ] **Step 7: 验证编译**

```bash
cargo check
```

Expected: 无错误

- [ ] **Step 8: Commit**

```bash
git add src/config.rs src/direction.rs src/rocket.rs src/packer.rs src/packers/mod.rs src/packers/json.rs src/lib.rs
git commit -m "feat: add config, direction, rocket, packer skeleton"
```

---

### Task 4: 实现 http.rs

**Files:**
- Create: `src/http.rs`
- Modify: `src/lib.rs:1-10`

- [ ] **Step 1: 创建 src/http.rs**

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

- [ ] **Step 2: 更新 src/lib.rs 导出 http**

```rust
pub mod http;

pub use http::get_client;
```

- [ ] **Step 3: 验证编译**

```bash
cargo check
```

Expected: 无错误

- [ ] **Step 4: Commit**

```bash
git add src/http.rs src/lib.rs
git commit -m "feat: add global HTTP client singleton"
```

---

## Phase 2: 核心类型

### Task 5: 完善 packer 和测试

**Files:**
- Modify: `src/packers/json.rs`
- Create: `tests/packer_test.rs`

- [ ] **Step 1: 写 packer 测试**

`tests/packer_test.rs`:

```rust
use artful::packers::JsonPacker;
use artful::packer::Packer;
use serde_json::json;
use std::collections::HashMap;

#[test]
fn test_json_packer_pack() {
    let packer = JsonPacker;
    let mut data = HashMap::new();
    data.insert("key".to_string(), json!("value"));
    
    let result = packer.pack(&data).unwrap();
    assert_eq!(result, r#"{"key":"value"}"#);
}

#[test]
fn test_json_packer_pack_empty() {
    let packer = JsonPacker;
    let data = HashMap::new();
    
    let result = packer.pack(&data).unwrap();
    assert_eq!(result, "{}");
}

#[test]
fn test_json_packer_unpack() {
    let packer = JsonPacker;
    let json = r#"{"key":"value"}"#;
    
    let result = packer.unpack(json).unwrap();
    assert_eq!(result["key"], json!("value"));
}

#[test]
fn test_json_packer_unpack_invalid() {
    let packer = JsonPacker;
    let invalid_json = "not json";
    
    let result = packer.unpack(invalid_json);
    assert!(result.is_err());
}
```

- [ ] **Step 2: 运行测试验证失败**

```bash
cargo test --test packer_test
```

Expected: 测试通过（已实现）

- [ ] **Step 3: Commit**

```bash
git add tests/packer_test.rs
git commit -m "test: add packer tests"
```

---

### Task 6: 完善 direction 和实现 CollectionDirection

**Files:**
- Create: `src/directions/mod.rs`
- Create: `src/directions/collection.rs`
- Modify: `src/direction.rs`
- Create: `tests/direction_test.rs`

- [ ] **Step 1: 创建 src/directions/mod.rs**

```rust
mod collection;
mod response;
mod no_http;
mod origin;

pub use collection::CollectionDirection;
pub use response::ResponseDirection;
pub use no_http::NoHttpRequestDirection;
pub use origin::OriginResponseDirection;
```

- [ ] **Step 2: 创建 src/directions/collection.rs**

```rust
use async_trait::async_trait;
use serde_json::Value;

use crate::direction::{Direction, Destination};
use crate::Result;
use crate::Rocket;

/// JSON Collection 解析器
pub struct CollectionDirection;

#[async_trait]
impl Direction for CollectionDirection {
    async fn parse(&self, rocket: &mut Rocket) -> Result<Destination> {
        if let Some(resp) = rocket.destination_origin.as_mut() {
            let text = resp.text().await?;
            let json: Value = rocket.packer.unpack(&text)?;
            Ok(Destination::Collection(json))
        } else {
            Ok(Destination::None)
        }
    }
}
```

- [ ] **Step 3: 创建 src/directions/response.rs**

```rust
use async_trait::async_trait;

use crate::direction::{Direction, Destination};
use crate::Result;
use crate::Rocket;

/// 原始 Response 解析器
pub struct ResponseDirection;

#[async_trait]
impl Direction for ResponseDirection {
    async fn parse(&self, rocket: &mut Rocket) -> Result<Destination> {
        if let Some(resp) = rocket.destination_origin.take() {
            Ok(Destination::Response(resp))
        } else {
            Ok(Destination::None)
        }
    }
}
```

- [ ] **Step 4: 创建 src/directions/no_http.rs**

```rust
use async_trait::async_trait;

use crate::direction::{Direction, Destination};
use crate::Result;
use crate::Rocket;

/// 不发起 HTTP 请求解析器
pub struct NoHttpRequestDirection;

#[async_trait]
impl Direction for NoHttpRequestDirection {
    async fn parse(&self, rocket: &mut Rocket) -> Result<Destination> {
        Ok(Destination::None)
    }
}
```

- [ ] **Step 5: 创建 src/directions/origin.rs**

```rust
use async_trait::async_trait;

use crate::direction::{Direction, Destination};
use crate::Result;
use crate::Rocket;

/// 返回 Rocket 解析器
pub struct OriginResponseDirection;

#[async_trait]
impl Direction for OriginResponseDirection {
    async fn parse(&self, rocket: &mut Rocket) -> Result<Destination> {
        if rocket.config.return_rocket {
            // TODO: Rocket 需要 Clone trait
            Ok(Destination::None)
        } else {
            Ok(Destination::None)
        }
    }
}
```

- [ ] **Step 6: 更新 src/lib.rs 导出 directions**

```rust
pub mod directions;

pub use directions::{CollectionDirection, ResponseDirection, NoHttpRequestDirection, OriginResponseDirection};
```

- [ ] **Step 7: 添加 async_trait 依赖**

```rust
// 在文件顶部添加
use async_trait::async_trait;
```

到 `src/direction.rs`

- [ ] **Step 8: 写 direction 测试**

`tests/direction_test.rs`:

```rust
use artful::direction::{DirectionKind, Destination};
use artful::directions::CollectionDirection;

#[test]
fn test_direction_kind_default() {
    let kind = DirectionKind::CollectionDirection;
    assert!(matches!(kind, DirectionKind::CollectionDirection));
}

#[test]
fn test_destination_default() {
    let dest = Destination::default();
    assert!(matches!(dest, Destination::None));
}
```

- [ ] **Step 9: 运行测试**

```bash
cargo test --test direction_test
```

Expected: 测试通过

- [ ] **Step 10: Commit**

```bash
git add src/directions/mod.rs src/directions/collection.rs src/directions/response.rs src/directions/no_http.rs src/directions/origin.rs src/lib.rs tests/direction_test.rs
git commit -m "feat: add direction implementations"
```

---

### Task 7: 实现 plugin.rs

**Files:**
- Create: `src/plugin.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: 创建 src/plugin.rs**

```rust
use async_trait::async_trait;

use crate::Rocket;
use crate::flow_ctrl::Next;

/// 插件 trait - 洋葱模型核心
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    /// 组装请求
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>);
}
```

- [ ] **Step 2: 创建 src/flow_ctrl.rs**

```rust
use std::sync::Arc;

use crate::plugin::Plugin;
use crate::Rocket;

/// 洋葱模型流向控制器
pub struct FlowCtrl {
    /// 当前执行位置
    cursor: usize,
    
    /// 插件列表
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

- [ ] **Step 3: 更新 src/lib.rs 导出**

```rust
pub mod plugin;
pub mod flow_ctrl;

pub use plugin::Plugin;
pub use flow_ctrl::{FlowCtrl, Next};
```

- [ ] **Step 4: 验证编译**

```bash
cargo check
```

Expected: 无错误

- [ ] **Step 5: Commit**

```bash
git add src/plugin.rs src/flow_ctrl.rs src/lib.rs
git commit -m "feat: add plugin trait and flow_ctrl"
```

---

### Task 8: 实现 shortcut.rs

**Files:**
- Create: `src/shortcut.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: 创建 src/shortcut.rs**

```rust
use std::sync::Arc;

use crate::plugin::Plugin;
use crate::rocket::RocketConfig;
use serde_json::Value;
use std::collections::HashMap;

/// 快捷方式 trait
pub trait Shortcut {
    /// 返回插件列表
    fn get_plugins(&self, config: &RocketConfig, payload: &HashMap<String, Value>) 
        -> Vec<Arc<dyn Plugin>>;
}
```

- [ ] **Step 2: 更新 src/lib.rs 导出**

```rust
pub mod shortcut;

pub use shortcut::Shortcut;
```

- [ ] **Step 3: 验证编译**

```bash
cargo check
```

Expected: 无错误

- [ ] **Step 4: Commit**

```bash
git add src/shortcut.rs src/lib.rs
git commit -m "feat: add shortcut trait"
```

---

## Phase 3: 主入口

### Task 9: 实现 artful.rs

**Files:**
- Create: `src/artful.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: 创建 src/artful.rs**

```rust
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

use crate::rocket::{Rocket, RocketConfig};
use crate::flow_ctrl::FlowCtrl;
use crate::plugin::Plugin;
use crate::shortcut::Shortcut;
use crate::direction::Destination;
use crate::Result;
use crate::http::get_client;

/// Artful 主类 - 框架入口
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
        client.execute(request).await.map_err(crate::error::ArtfulError::RequestFailed)
    }
}
```

- [ ] **Step 2: 更新 src/lib.rs 导出**

```rust
pub mod artful;

pub use artful::Artful;
```

- [ ] **Step 3: 验证编译**

```bash
cargo check
```

Expected: 无错误

- [ ] **Step 4: Commit**

```bash
git add src/artful.rs src/lib.rs
git commit -m "feat: add Artful main entry"
```

---

## Phase 4: 内置插件

### Task 10: 实现内置插件

**Files:**
- Create: `src/plugins/mod.rs`
- Create: `src/plugins/start.rs`
- Create: `src/plugins/add_payload_body.rs`
- Create: `src/plugins/add_radar.rs`
- Create: `src/plugins/parser.rs`
- Create: `src/plugins/log.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: 创建 src/plugins/mod.rs**

```rust
mod start;
mod add_payload_body;
mod add_radar;
mod parser;
mod log;

pub use start::StartPlugin;
pub use add_payload_body::AddPayloadBodyPlugin;
pub use add_radar::AddRadarPlugin;
pub use parser::ParserPlugin;
pub use log::LogPlugin;
```

- [ ] **Step 2: 创建 src/plugins/start.rs**

```rust
use async_trait::async_trait;

use crate::plugin::Plugin;
use crate::Rocket;
use crate::flow_ctrl::Next;

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

- [ ] **Step 3: 创建 src/plugins/add_payload_body.rs**

```rust
use async_trait::async_trait;

use crate::plugin::Plugin;
use crate::Rocket;
use crate::flow_ctrl::Next;

/// 添加 payload body 插件
pub struct AddPayloadBodyPlugin;

#[async_trait]
impl Plugin for AddPayloadBodyPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // 如果未手动指定 body，将 payload 序列化为 JSON
        if rocket.config.body.is_none() && !rocket.payload.is_empty() {
            if let Ok(body) = rocket.packer.pack(&rocket.payload) {
                rocket.config.body = Some(body);
            }
        }
        
        next.call(rocket).await;
    }
}
```

- [ ] **Step 4: 创建 src/plugins/add_radar.rs**

```rust
use async_trait::async_trait;
use std::time::Duration;

use crate::plugin::Plugin;
use crate::Rocket;
use crate::flow_ctrl::Next;
use crate::http::get_client;

/// 构建 HTTP Request 插件
pub struct AddRadarPlugin;

#[async_trait]
impl Plugin for AddRadarPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        let method = rocket.config.method.clone();
        let url = rocket.config.url.clone();
        
        let client = get_client();
        let mut request_builder = client.request(method, &url);
        
        // 添加 headers
        for (key, value) in &rocket.config.headers {
            request_builder = request_builder.header(key, value);
        }
        
        // 添加 body
        if let Some(body) = &rocket.config.body {
            request_builder = request_builder.body(body.clone());
        } else if !rocket.payload.is_empty() {
            if let Ok(body) = rocket.packer.pack(&rocket.payload) {
                request_builder = request_builder.body(body);
            }
        }
        
        // 应用 timeout
        if let Some(timeout) = rocket.config.http.timeout {
            request_builder = request_builder.timeout(
                Duration::from_secs(timeout)
            );
        }
        
        if let Ok(request) = request_builder.build() {
            rocket.radar = Some(request);
        }
        
        next.call(rocket).await;
    }
}
```

- [ ] **Step 5: 创建 src/plugins/parser.rs**

```rust
use async_trait::async_trait;

use crate::plugin::Plugin;
use crate::Rocket;
use crate::flow_ctrl::Next;
use crate::http::get_client;
use crate::direction::{DirectionKind, Destination};
use crate::directions::CollectionDirection;

/// 解析响应插件 - 执行 HTTP 请求并解析结果
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
        if let Ok(response) = client.execute(request).await {
            rocket.destination_origin = Some(response);
            
            // 解析响应
            match &rocket.direction {
                DirectionKind::CollectionDirection => {
                    let direction = CollectionDirection;
                    if let Ok(dest) = direction.parse(rocket).await {
                        rocket.destination = Some(dest);
                    }
                }
                DirectionKind::ResponseDirection => {
                    if let Some(resp) = rocket.destination_origin.take() {
                        rocket.destination = Some(Destination::Response(resp));
                    }
                }
                DirectionKind::OriginResponseDirection => {
                    if rocket.config.return_rocket {
                        // TODO: Rocket 需要 Clone trait
                    }
                }
                DirectionKind::Custom(d) => {
                    if let Ok(dest) = d.parse(rocket).await {
                        rocket.destination = Some(dest);
                    }
                }
                _ => {}
            }
        }
        
        next.call(rocket).await;
    }
}
```

- [ ] **Step 6: 创建 src/plugins/log.rs**

```rust
use async_trait::async_trait;
use std::time::Instant;

use crate::plugin::Plugin;
use crate::Rocket;
use crate::flow_ctrl::Next;

/// 日志记录插件
pub struct LogPlugin;

#[async_trait]
impl Plugin for LogPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        let start = Instant::now();
        
        tracing::info!(
            method = %rocket.config.method,
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

- [ ] **Step 7: 更新 src/lib.rs 导出**

```rust
pub mod plugins;

pub use plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin, LogPlugin};
```

- [ ] **Step 8: 验证编译**

```bash
cargo check
```

Expected: 无错误

- [ ] **Step 9: Commit**

```bash
git add src/plugins/mod.rs src/plugins/start.rs src/plugins/add_payload_body.rs src/plugins/add_radar.rs src/plugins/parser.rs src/plugins/log.rs src/lib.rs
git commit -m "feat: add built-in plugins"
```

---

## Phase 5: 测试

### Task 11: 实现 FlowCtrl 测试

**Files:**
- Create: `tests/flow_ctrl_test.rs`

- [ ] **Step 1: 创建测试**

`tests/flow_ctrl_test.rs`:

```rust
use artful::flow_ctrl::FlowCtrl;
use artful::plugin::Plugin;
use artful::Rocket;
use artful::RocketConfig;
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

struct TestPlugin {
    name: String,
}

#[async_trait]
impl Plugin for TestPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: artful::flow_ctrl::Next<'_>) {
        rocket.payload.insert("visited".to_string(), serde_json::json!(self.name.clone()));
        next.call(rocket).await;
    }
}

#[tokio::test]
async fn test_flow_ctrl_basic() {
    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(TestPlugin { name: "plugin1".to_string() }),
        Arc::new(TestPlugin { name: "plugin2".to_string() }),
    ];
    
    let mut ctrl = FlowCtrl::new(plugins);
    let config = RocketConfig::default();
    let mut rocket = Rocket::new(config, HashMap::new());
    
    ctrl.call_next(&mut rocket).await;
    
    assert!(rocket.payload.contains_key("visited"));
}

#[tokio::test]
async fn test_flow_ctrl_cease() {
    struct CeasePlugin;
    
    #[async_trait]
    impl Plugin for CeasePlugin {
        async fn assembly(&self, rocket: &mut Rocket, next: artful::flow_ctrl::Next<'_>) {
            rocket.payload.insert("ceased".to_string(), serde_json::json!(true));
            // 不调用 next，停止流程
        }
    }
    
    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(CeasePlugin),
        Arc::new(TestPlugin { name: "should_not_run".to_string() }),
    ];
    
    let mut ctrl = FlowCtrl::new(plugins);
    let config = RocketConfig::default();
    let mut rocket = Rocket::new(config, HashMap::new());
    
    ctrl.call_next(&mut rocket).await;
    
    assert!(rocket.payload.contains_key("ceased"));
    assert!(!rocket.payload.contains_key("visited"));
}

#[test]
fn test_flow_ctrl_has_next() {
    let plugins: Vec<Arc<dyn Plugin>> = vec![
        Arc::new(TestPlugin { name: "p1".to_string() }),
    ];
    
    let ctrl = FlowCtrl::new(plugins);
    assert!(ctrl.has_next());
}
```

- [ ] **Step 2: 运行测试**

```bash
cargo test --test flow_ctrl_test
```

Expected: 测试通过

- [ ] **Step 3: Commit**

```bash
git add tests/flow_ctrl_test.rs
git commit -m "test: add flow_ctrl tests"
```

---

### Task 12: 实现 Rocket 测试

**Files:**
- Create: `tests/rocket_test.rs`

- [ ] **Step 1: 创建测试**

`tests/rocket_test.rs`:

```rust
use artful::{Rocket, RocketConfig, HttpOptions};
use std::collections::HashMap;
use serde_json::json;

#[test]
fn test_rocket_config_default() {
    let config = RocketConfig::default();
    assert_eq!(config.method, reqwest::Method::POST);
    assert_eq!(config.url, "");
    assert!(config.headers.is_empty());
    assert!(config.body.is_none());
    assert!(!config.return_rocket);
}

#[test]
fn test_rocket_new() {
    let config = RocketConfig {
        method: reqwest::Method::GET,
        url: "https://example.com".to_string(),
        ..Default::default()
    };
    
    let mut payload = HashMap::new();
    payload.insert("key".to_string(), json!("value"));
    
    let rocket = Rocket::new(config, payload);
    
    assert_eq!(rocket.config.method, reqwest::Method::GET);
    assert_eq!(rocket.config.url, "https://example.com");
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
    };
    assert_eq!(http.timeout, Some(30));
    assert_eq!(http.connect_timeout, Some(10));
}
```

- [ ] **Step 2: 运行测试**

```bash
cargo test --test rocket_test
```

Expected: 测试通过

- [ ] **Step 3: Commit**

```bash
git add tests/rocket_test.rs
git commit -m "test: add rocket tests"
```

---

### Task 13: 实现集成测试

**Files:**
- Create: `tests/integration_test.rs`

- [ ] **Step 1: 创建测试**

`tests/integration_test.rs`:

```rust
use artful::{Artful, RocketConfig, Plugin};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use artful::direction::Destination;
use std::sync::Arc;
use std::collections::HashMap;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde_json::json;

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
    
    let result = Artful::artful(config, HashMap::new(), plugins).await.unwrap();
    
    assert!(matches!(result, Destination::Collection(_)));
}

#[tokio::test]
async fn test_pipeline_with_payload() {
    let mock_server = MockServer::start();
    
    Mock::given(method("POST"))
        .and(path("/api/test"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({"status": "ok"})))
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
    
    if let Destination::Collection(json) = result {
        assert_eq!(json["status"], "ok");
    } else {
        panic!("Expected Collection destination");
    }
}
```

- [ ] **Step 2: 运行测试**

```bash
cargo test --test integration_test
```

Expected: 测试通过

- [ ] **Step 3: Commit**

```bash
git add tests/integration_test.rs
git commit -m "test: add integration tests"
```

---

### Task 14: 实现 Shortcut 测试

**Files:**
- Create: `tests/shortcut_test.rs`

- [ ] **Step 1: 创建测试**

`tests/shortcut_test.rs`:

```rust
use artful::{Artful, Shortcut, RocketConfig, Plugin};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Default)]
struct TestShortcut;

impl Shortcut for TestShortcut {
    fn get_plugins(&self, _config: &RocketConfig, _payload: &HashMap<String, serde_json::Value>) 
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

#[tokio::test]
async fn test_shortcut_basic() {
    // 注意：这个测试需要一个真实的 URL，或者我们可以用 mock
    // 这里仅验证 shortcut trait 可以正常工作
    let shortcut = TestShortcut::default();
    let config = RocketConfig::default();
    let plugins = shortcut.get_plugins(&config, &HashMap::new());
    
    assert_eq!(plugins.len(), 4);
}
```

- [ ] **Step 2: 运行测试**

```bash
cargo test --test shortcut_test
```

Expected: 测试通过

- [ ] **Step 3: Commit**

```bash
git add tests/shortcut_test.rs
git commit -m "test: add shortcut tests"
```

---

### Task 15: 实现 Artful 主入口测试

**Files:**
- Create: `tests/artful_test.rs`

- [ ] **Step 1: 创建测试**

`tests/artful_test.rs`:

```rust
use artful::{Artful, RocketConfig};
use artful::direction::{DirectionKind, Destination};
use artful::plugins::{StartPlugin, AddRadarPlugin, ParserPlugin};
use std::sync::Arc;
use std::collections::HashMap;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use serde_json::json;

#[tokio::test]
async fn test_artful_basic() {
    let mock_server = MockServer::start();
    
    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(json!({"success": true})))
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
    
    let result = Artful::artful(config, HashMap::new(), plugins).await.unwrap();
    
    assert!(matches!(result, Destination::Collection(_)));
}

#[tokio::test]
async fn test_artful_with_custom_direction() {
    let mock_server = MockServer::start();
    
    Mock::given(method("GET"))
        .and(path("/raw"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_raw("raw response", "text/plain"))
        .mount(&mock_server)
        .await;
    
    let config = RocketConfig {
        method: reqwest::Method::GET,
        url: mock_server.uri() + "/raw",
        ..Default::default()
    };
    
    let mut rocket = artful::Rocket::new(config, HashMap::new());
    rocket.direction = DirectionKind::ResponseDirection;
    
    // 手动构建 rocket 和 flow_ctrl
    let plugins: Vec<Arc<dyn artful::Plugin>> = vec![
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];
    
    let mut ctrl = artful::FlowCtrl::new(plugins);
    ctrl.call_next(&mut rocket).await;
    
    // ResponseDirection 应返回原始 Response
    assert!(matches!(rocket.destination, Some(Destination::Response(_))));
}
```

- [ ] **Step 2: 运行测试**

```bash
cargo test --test artful_test
```

Expected: 测试通过

- [ ] **Step 3: Commit**

```bash
git add tests/artful_test.rs
git commit -m "test: add artful main entry tests"
```

---

### Task 16: 运行所有测试

- [ ] **Step 1: 运行完整测试套件**

```bash
cargo test
```

Expected: 所有测试通过

- [ ] **Step 2: 检查代码格式**

```bash
cargo fmt
```

Expected: 格式化完成

- [ ] **Step 3: 检查 lint**

```bash
cargo clippy -- -D warnings
```

Expected: 无 warnings

- [ ] **Step 4: Commit（如果有格式更改）**

```bash
git add -A
git commit -m "chore: format and lint check"
```

---

## Phase 6: 文档

### Task 17: 创建 README.md

**Files:**
- Create: `README.md`

- [ ] **Step 1: 创建 README.md**

```markdown
# Artful-Rs

> Api RequesT Framework U Like - 你喜欢的 Rust API 请求框架

基于洋葱模型的 Rust HTTP 客户端框架，灵感来自 [yansongda/artful](https://github.com/yansongda/artful)。

## 特性

- 🔄 **洋葱模型**: 请求层层穿透，响应层层返回
- 🔌 **插件化**: 每个请求都是一个插件组合
- 🛡️ **类型安全**: Rust 类型系统确保配置安全
- ⚡ **高性能**: 全局 HTTP Client 单例，共享连接池

## 安装

```toml
[dependencies]
artful = "0.1.0"
```

## 快速开始

```rust
use artful::{Artful, RocketConfig};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use std::sync::Arc;
use std::collections::HashMap;

let config = RocketConfig {
    method: reqwest::Method::POST,
    url: "https://api.example.com/orders".to_string(),
    ..Default::default()
};

let plugins: Vec<Arc<dyn artful::Plugin>> = vec![
    Arc::new(StartPlugin),
    Arc::new(AddPayloadBodyPlugin),
    Arc::new(AddRadarPlugin),
    Arc::new(ParserPlugin),
];

let result = Artful::artful(config, HashMap::new(), plugins).await?;
```

## 文档

详细文档请参阅 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)。

## 许可证

MIT License
```

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: add README"
```

---

## 质量保证检查清单

- [ ] 所有 cargo test 通过
- [ ] cargo fmt 格式化完成
- [ ] cargo clippy 无 warnings
- [ ] cargo doc 生成的文档完整
- [ ] README.md 包含基本使用说明

---

## 实现完成

完成后，项目应包含：

1. ✅ 核心架构（Rocket, FlowCtrl, Plugin）
2. ✅ 内置插件（Start, AddPayloadBody, AddRadar, Parser, Log）
3. ✅ HTTP 客户端单例
4. ✅ JSON Packer
5. ✅ Direction 解析策略
6. ✅ 完整测试覆盖
7. ✅ 基础文档

---

## 备注

- `OriginResponseDirection` 的 Rocket Clone 实现需要后续完善
- 更多插件（Retry、Cache 等）在 v0.2.0 实现
- 支付宝/微信支付插件包在 v0.3.0 实现