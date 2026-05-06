# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> 本文件仅记录 artisan facade 相关变更，各子 crate 变更详见各自目录下的 CHANGELOG.md

## [0.13.0] - 2026-05-06

### Changed

- **BREAKING**: 简化 facade 为模块级 re-export，import 路径从 `artisan::Type` 改为 `artisan::http::Type`
  ([65085f4](https://github.com/yansongda/artisan/commit/65085f4))
- 添加 `docsrs` 属性，支持 docs.rs feature badge 显示
  ([65085f4](https://github.com/yansongda/artisan/commit/65085f4))

## [0.12.0] - 2026-05-06

### Changed

- 重构为 workspace 结构
  ([9261167](https://github.com/yansongda/artisan/commit/9261167))

### Added

- Feature 控制的 re-export，支持 `default-features = false` 禁用 HTTP 功能
- 可选依赖 `artisan-http`（通过 "http" feature 控制）

## [0.11.0] - 2026-04-15

### Changed

- 移除 `Shortcut` trait 的 `Default` bound，使其 dyn compatible
  ([c4c1b9f](https://github.com/yansongda/artful-rs/commit/c4c1b9f))
- 代码优化 - 错误处理、架构、性能
  ([b5f3e5d](https://github.com/yansongda/artful-rs/commit/b5f3e5d))

## [0.10.0] - 2026-04-14

### Changed

- 重命名包名 `artful` → `artisan`，避免 crates.io 冲突
  ([0e7f91a](https://github.com/yansongda/artful-rs/commit/0e7f91a))
- 简化 `DirectionKind` 枚举命名
  ([42a37d9](https://github.com/yansongda/artful-rs/commit/42a37d9))
- 改进 API 设计和错误处理
  ([9eb78e6](https://github.com/yansongda/artful-rs/commit/9eb78e6))

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
