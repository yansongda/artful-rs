# Artful-Rs

> Api RequesT Framework U Like - 你喜欢的 Rust API 请求框架

基于洋葱模型的 Rust HTTP 客户端框架，灵感来自 [yansongda/artful](https://github.com/yansongda/artful)。

## 特性

- 🔄 **洋葱模型**: 请求层层穿透，响应层层返回
- 🔌 **插件化**: 每个请求都是一个插件组合，高度灵活可定制
- 🛡️ **类型安全**: Rust 类型系统确保配置和参数的类型安全
- ⚡ **高性能**: 全局 HTTP Client 单例，共享连接池
- 📦 **零依赖冲突**: 仅使用主流稳定依赖

## 安装

```toml
[dependencies]
artful = "0.1.0"
```

## 快速开始

### 基础使用

```rust
use artful::{Artful, RocketConfig};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> artful::Result<()> {
    let config = RocketConfig {
        method: reqwest::Method::POST,
        url: "https://api.example.com/orders".to_string(),
        ..Default::default()
    };

    let payload = HashMap::from([
        ("order_id", json!("123")),
        ("amount", json!(100)),
    ]);

    let plugins: Vec<Arc<dyn artful::Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(AddPayloadBodyPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(config, payload, plugins).await?;
    
    if let artful::Destination::Collection(json) = result {
        println!("Response: {}", json);
    }

    Ok(())
}
```

### 使用 Shortcut 快捷方式

```rust
use artful::{Artful, Shortcut, RocketConfig, Plugin};
use artful::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Default)]
struct MyApiShortcut;

impl Shortcut for MyApiShortcut {
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

let result = Artful::shortcut::<MyApiShortcut>(
    RocketConfig {
        url: "https://api.example.com".to_string(),
        ..Default::default()
    },
    HashMap::new(),
).await?;
```

### 自定义插件

```rust
use artful::{Plugin, Rocket, flow_ctrl::Next};
use async_trait::async_trait;

/// 签名插件
pub struct SignaturePlugin {
    api_key: String,
}

#[async_trait]
impl Plugin for SignaturePlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) {
        // 前向：添加签名头
        rocket.config.headers.insert(
            "X-Signature".to_string(),
            sign(&self.api_key, &rocket.payload),
        );
        
        // 调用下一层
        next.call(rocket).await;
        
        // 后向：可选的响应验证
        // ...
    }
}
```

## 核心概念

### Rocket - 请求载体

`Rocket` 是整个请求生命周期中的数据载体：

```rust
pub struct Rocket {
    pub config: RocketConfig,      // 配置（method、url、headers 等）
    pub payload: HashMap<String, Value>, // 业务参数
    pub radar: Option<Request>,    // HTTP 请求对象
    pub destination: Option<Destination>, // 解析结果
    pub direction: DirectionKind,  // 响应解析策略
    pub packer: Arc<dyn Packer>,   // 序列化器
}
```

### Plugin - 插件（洋葱模型）

插件是洋葱模型的核心，每个插件可以在请求前向和后向阶段执行操作：

```rust
#[async_trait]
pub trait Plugin: Send + Sync + 'static {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>);
}
```

执行流程：
```
请求 → Plugin1 前向 → Plugin2 前向 → Plugin3 前向 → HTTP 请求
响应 ← Plugin1 后向 ← Plugin2 后向 ← Plugin3 后向 ← HTTP 响应
```

### Direction - 响应解析策略

```rust
pub enum DirectionKind {
    CollectionDirection,     // 解析为 JSON（默认）
    ResponseDirection,       // 返回原始 Response
    NoHttpRequestDirection,  // 不发起 HTTP 请求
    OriginResponseDirection, // 返回 Rocket（调试用）
    Custom(Arc<dyn Direction>), // 自定义解析器
}
```

## 内置插件

| 插件 | 功能 |
|------|------|
| `StartPlugin` | 初始化（占位） |
| `AddPayloadBodyPlugin` | 将 payload 序列化为请求体 |
| `AddRadarPlugin` | 构建 HTTP Request |
| `ParserPlugin` | 执行请求并解析响应 |
| `LogPlugin` | 日志记录 |

## 文档

详细架构设计请参阅 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)。

## 许可证

MIT License