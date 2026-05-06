# Artisan

> Api RequesT Framework U Like - 你喜欢的 Rust API 请求框架

基于洋葱模型的 Rust HTTP 客户端框架，灵感来自 [yansongda/artful](https://github.com/yansongda/artful)。

## 特性

- 🔄 **洋葱模型**: 请求层层穿透，响应层层返回
- 🔌 **插件化**: 每个请求都是一个插件组合，高度灵活可定制
- 🛡️ **类型安全**: Rust 类型系统确保配置和参数的类型安全
- ⚡ **高性能**: 全局 HTTP Client 单例，共享连接池
- 📦 **模块化**: Workspace 架构，HTTP 功能可选

## 架构

```
┌─────────────────────────────────────┐
│           artisan (facade)          │  ← 用户依赖这一层
│         Feature-controlled          │
│            re-export                │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│         artisan-http                │  ← 实现层
│    HTTP Client + Onion Model        │
│      Plugin System + Types          │
└─────────────────────────────────────┘
```

### Crate 说明

| Crate | 角色 | 说明 |
|-------|------|------|
| `artisan` | Facade | Feature 控制的 re-export，默认包含 HTTP 功能 |
| `artisan-http` | 实现 | HTTP 客户端、洋葱模型、插件系统 |

## 安装

### 推荐方式（通过 facade）

```toml
# 默认包含 HTTP 功能
[dependencies]
artisan = "0.12"
```

### 直接依赖实现层

```toml
# 直接使用 HTTP 实现
[dependencies]
artisan-http = "0.1"
```

### 禁用 HTTP 功能

```toml
# 纯 facade，不包含 HTTP 实现
[dependencies]
artisan = { version = "0.12", default-features = false }
```

## 快速开始

```rust
use artisan_http::{Artful, Plugin, Rocket, flow_ctrl::Next};
use artisan_http::plugins::{StartPlugin, AddPayloadBodyPlugin, AddRadarPlugin, ParserPlugin};
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> artisan_http::Result<()> {
    let params = HashMap::from([
        ("order_id".to_string(), json!("123")),
        ("amount".to_string(), json!(100)),
    ]);

    let plugins: Vec<Arc<dyn artisan_http::Plugin>> = vec![
        Arc::new(StartPlugin),
        Arc::new(AddPayloadBodyPlugin),
        Arc::new(AddRadarPlugin),
        Arc::new(ParserPlugin),
    ];

    let result = Artful::artful(params, plugins).await?;
    
    if let artisan_http::Destination::Json(json) = result {
        println!("Response: {}", json);
    }

    Ok(())
}
```

## 示例

```bash
cargo run -p artisan-http --example basic
cargo run -p artisan-http --example config
cargo run -p artisan-http --example shortcut
cargo run -p artisan-http --example custom_plugin
cargo run -p artisan-http --example direction
```

## Workspace 结构

```
artisan/                    # 根目录（facade crate）
├── Cargo.toml              # Workspace 配置
├── src/lib.rs              # Feature 控制的 re-export
└── artisan-http/           # HTTP 实现 crate
    ├── Cargo.toml
    ├── src/                # 所有实现代码
    ├── tests/              # 所有测试（59 个）
    ├── examples/           # 所有示例
    └── docs/               # 架构文档
```

## 文档

- **详细文档**: [artisan-http/README.md](artisan-http/README.md)
- **架构设计**: [artisan-http/docs/ARCHITECTURE.md](artisan-http/docs/ARCHITECTURE.md)
- **实现细节**: [artisan-http/AGENTS.md](artisan-http/AGENTS.md)

## 许可证

MIT License
