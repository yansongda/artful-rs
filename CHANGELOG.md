# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0] - 2026-04-14

### Changed

- 重命名包名 `artful` → `artisan`，避免 crates.io 冲突
- 简化 `DirectionKind` 枚举命名
- 改进 API 设计和错误处理

### Fixed

- 修复 `JsonDirection` 错误类型映射
- 为零大小插件添加 `Clone + Copy` trait

### Added

- 添加完整测试覆盖 (59 tests)
- 添加 `AGENTS.md` 指导文件

### Documentation

- 更新 `AGENTS.md` 强调提交前验证流程

### Style

- `cargo fmt` 格式化代码

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