# AGENTS.md

## Project Overview

**artisan** (crate name, internal struct `Artful`) - Rust HTTP client framework using onion model pattern.

- Edition: 2024
- MSRV: 1.85
- License: MIT

## Commands

```bash
# Build & check
cargo check --all-features

# Test (all 59 tests)
cargo test --all-features

# Lint (required before commit)
cargo fmt --all -- --check
cargo clippy -- -D warnings

# Run examples
cargo run --example basic
cargo run --example config
cargo run --example shortcut
cargo run --example custom_plugin
cargo run --example direction

# Publish (automated via GitHub tag)
# Manual: cargo publish --token $CARGO_TOKEN
```

## Architecture

### Core Concepts (Onion Model)

```
Request → Plugin1 → Plugin2 → ... → HTTP → ... → Plugin2 → Plugin1 → Response
```

**Key Types**:

| Type | Role | File |
|------|------|------|
| `Artful` | Main entry point | `src/artisan.rs` |
| `Rocket` | Request/response carrier | `src/rocket.rs` |
| `Plugin` | Middleware trait | `src/plugin.rs` |
| `FlowCtrl` | Execution controller | `src/flow_ctrl.rs` |
| `Next` | Chain continuation | `src/flow_ctrl.rs` |
| `Direction` | Response parser trait | `src/direction.rs` |
| `Packer` | Serializer trait | `src/packer.rs` |
| `Shortcut` | Plugin preset trait | `src/shortcut.rs` |

### Module Structure

```
src/
├── lib.rs           # Public API exports
├── artisan.rs       # Artful struct (config, artful, shortcut, raw methods)
├── rocket.rs        # Rocket + RocketConfig + HttpOptions
├── flow_ctrl.rs     # FlowCtrl + Next (onion control)
├── plugin.rs        # Plugin trait (async_trait)
├── plugins/         # Built-in plugins
│   ├── start.rs     # StartPlugin (init payload)
│   ├── add_radar.rs # AddRadarPlugin (build Request)
│   ├── parser.rs    # ParserPlugin (execute + parse)
│   └── add_payload_body.rs
├── direction.rs     # Direction trait + DirectionKind + Destination
├── directions/      # Built-in parsers (JsonDirection)
├── packer.rs        # Packer trait
├── packers/         # Built-in serializers (JsonPacker)
├── shortcut.rs      # Shortcut trait
├── config.rs        # Config + LoggerConfig (global)
├── error.rs         # ArtfulError enum (thiserror)
└── http.rs          # Global Client singleton (OnceLock)
```

## Patterns & Conventions

### Plugin Implementation

```rust
#[derive(Clone, Copy, Debug, Default)]  // Required for zero-size plugins
pub struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    async fn assembly(&self, rocket: &mut Rocket, next: Next<'_>) -> Result<()> {
        // Forward logic
        next.call(rocket).await?;  // Propagate to next layer
        // Backward logic
        Ok(())
    }
}
```

### HTTP Client Singleton

Global `reqwest::Client` via `OnceLock` in `src/http.rs`. Connection pool shared across all requests.

### Error Handling

- `ArtfulError` uses `thiserror` with `#[source]` for error chains
- `JsonDeserializeError` requires `source: Option<serde_json::Error>`
- `InvalidUrl` uses `source: reqwest::Error` (not String)

### Shortcut Trait

```rust
pub trait Shortcut: Default {  // Default bound required
    fn get_plugins(&self, params: &HashMap<String, Value>) -> Vec<Arc<dyn Plugin>>;
}
```

## Testing

- 59 tests across 7 files
- Use `wiremock` for HTTP mocking in integration tests
- `#[tokio::test]` for async tests
- Tests in `tests/` directory, not inline

### Test Files

| File | Coverage |
|------|----------|
| `artful_test.rs` | Artful methods, HTTP errors, plugin error propagation |
| `direction_test.rs` | DirectionKind, Destination, custom Direction |
| `flow_ctrl_test.rs` | FlowCtrl::cease, skip_rest, empty chain |
| `rocket_test.rs` | Rocket convenience methods, RocketConfig |
| `integration_test.rs` | Full pipeline tests |
| `packer_test.rs` | JsonPacker pack/unpack |
| `shortcut_test.rs` | Shortcut trait |

## Gotchas

1. **Crate name vs struct name**: Crate is `artisan`, main struct is `Artful`
   ```rust
   use artisan::Artful;  // Correct
   ```

2. **Plugin error propagation**: Use `?` after `next.call(rocket).await`
   ```rust
   next.call(rocket).await?;  // Required - not just .await
   ```

3. **DirectionKind enum**: `Json`, `Response`, `NoRequest`, `Custom`

4. **Rocket params vs payload**: `params` immutable, `payload` mutable by plugins

5. **No binary**: Library crate only, examples for demo

6. **CI triggers**: Push to `main` or PR; publish on `v*` tag

## References

- Architecture: `docs/ARCHITECTURE.md` (comprehensive)
- Examples: `examples/*.rs` (5 working demos)
- CI: `.github/workflows/coding-linter.yml`, `.github/workflows/publish.yml`