# Artisan

> Api RequesT Framework U Like - 你喜欢的 Rust API 请求框架

基于洋葱模型的 Rust HTTP 客户端框架，灵感来自 [yansongda/artful](https://github.com/yansongda/artful)。

## Workspace 结构

```
artisan/
├── Cargo.toml              # Workspace 配置
├── src/lib.rs              # Facade（Feature 控制的 re-export）
└── artisan-http/           # HTTP 实现
    ├── src/                # 核心实现
    ├── tests/              # 测试（59 个）
    ├── examples/           # 示例
    └── docs/               # 架构文档
```

## Crate 说明

| Crate | 版本 | 职责 |
|-------|------|------|
| [`artisan`](.) | 0.11.0 | Facade，Feature 控制的 re-export |
| [`artisan-http`](./artisan-http) | 0.12.0 | HTTP 客户端、洋葱模型、插件系统 |

## 安装

```toml
# 推荐：通过 facade（默认包含 HTTP 功能）
[dependencies]
artisan = "0.12"

# 直接依赖实现层
[dependencies]
artisan-http = "0.1"

# 纯 facade（禁用 HTTP 功能）
[dependencies]
artisan = { version = "0.12", default-features = false }
```

## 快速入口

- **快速开始**: [artisan-http/README.md](./artisan-http/README.md#快速开始)
- **架构设计**: [artisan-http/docs/ARCHITECTURE.md](./artisan-http/docs/ARCHITECTURE.md)
- **API 文档**: [docs.rs/artisan](https://docs.rs/artisan)

## 示例

```bash
cargo run -p artisan-http --example basic
cargo run -p artisan-http --example config
cargo run -p artisan-http --example shortcut
cargo run -p artisan-http --example custom_plugin
cargo run -p artisan-http --example direction
```

## 许可证

MIT License
