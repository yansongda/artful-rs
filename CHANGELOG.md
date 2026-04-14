# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0] - 2026-04-14

### Changed

- 重命名包名 `artful` → `artisan`，避免 crates.io 冲突
  ([0e7f91a](https://github.com/yansongda/artful-rs/commit/0e7f91a))
- 简化 `DirectionKind` 枚举命名
  ([42a37d9](https://github.com/yansongda/artful-rs/commit/42a37d9))
- 改进 API 设计和错误处理
  ([9eb78e6](https://github.com/yansongda/artful-rs/commit/9eb78e6))

### Fixed

- 修复 `JsonDirection` 错误类型映射，为零大小插件添加 `Clone + Copy` trait
  ([4e1ab9d](https://github.com/yansongda/artful-rs/commit/4e1ab9d))

### Added

- 添加完整测试覆盖 (59 tests)
  ([325641e](https://github.com/yansongda/artful-rs/commit/325641e))
- 添加 `AGENTS.md` 指导文件
  ([28f7afb](https://github.com/yansongda/artful-rs/commit/28f7afb))

### Documentation

- 更新 `AGENTS.md` 强调提交前验证流程
  ([7cb0db4](https://github.com/yansongda/artful-rs/commit/7cb0db4))

### Style

- `cargo fmt` 格式化代码
  ([7cb0db4](https://github.com/yansongda/artful-rs/commit/7cb0db4))

## [0.9.0] - 2025-XX-XX

Initial release with core onion model architecture.

### Added

- 洋葱模型 HTTP 客户端框架
- `Plugin` trait 中间件系统
- `Rocket` 请求载体
- `Direction` 响应解析策略
- `Packer` 序列化接口
- `Shortcut` 插件预设
- 内置插件: `StartPlugin`, `AddPayloadBodyPlugin`, `AddRadarPlugin`, `ParserPlugin`
- 全局 HTTP Client 单例 (OnceLock)
- `JsonDirection`, `JsonPacker` 默认实现