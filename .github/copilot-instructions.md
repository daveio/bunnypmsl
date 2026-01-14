# Agent Guide for Bunnypmsl

This document helps autonomous agents work effectively in the `bunnypmsl` codebase.

## Project Overview

`bunnypmsl` is a smart browser bookmark/shortcut tool written in Rust. It has two modes:

1. **CLI**: Runs commands directly in terminal (opens browser or prints URL).
2. **Server**: Runs a web server to redirect HTTP requests (like `http://localhost:8000/?cmd=gh`).

## Essential Commands

### Build & Run

- **Build**: `cargo build`
- **Run Server**: `cargo run -- serve`
- **Run CLI**: `cargo run -- [command]` (e.g., `cargo run -- gh`)
- **Docker**: `docker compose up -d`

### Testing & Quality

- **Test All**: `cargo test --all-features`
- **Lint**: `cargo clippy --all-features -- -D warnings`
- **Format**: `cargo fmt --all -- --check`
- **Trunk**: Uses `.trunk/trunk.yaml` for meta-linting configuration.

## Project Structure

- `src/main.rs`: Entry point. Dispatches to CLI or Server mode.
- `src/lib.rs`: Shared library code.
- `src/commands/`: **Core logic**. Contains individual command implementations (e.g., `github.rs`).
- `src/server/`: Web server implementation (Rocket + Leptos).
- `src/bunnypmsl_command_registry.rs`: Registry mapping strings to command handlers.
- `tests/`: Integration tests using `assert_cmd`.
- `deploy/`: Deployment scripts and docs.

## Development Patterns

### 1. Adding a New Command

To add a new shortcut (e.g., `mycmd`):

1. **Create Implementation**: Add `src/commands/mycmd.rs`.
   - Implement struct `MyCmdCommand`.
   - Define `BINDINGS` constant (aliases).
   - Implement `process_args` function.
   - Implement `get_info` function.
   - Look at `src/commands/github.rs` or others for a template.

2. **Export Module**: Add to `src/commands/mod.rs`:

   ```rust
   pub mod mycmd;
   pub use mycmd::MyCmdCommand;
   ```

3. **Register Command**: Add to `src/bunnypmsl_command_registry.rs` inside `register_commands!`:
   ```rust
   crate::commands::MyCmdCommand,
   ```

### 2. Testing

- **Unit Tests**: Place in the same file `src/commands/mycmd.rs` under `#[cfg(test)] mod tests`.
- **Integration Tests**: Add cases to `tests/cli_integration.rs` if needed, verifying CLI output.

### 3. Feature Flags

- The project uses `server` and `cli` features.
- Default includes both.
- When running `cargo run`, you usually want default features unless testing specific compilation targets.

## Configuration

- Config loading uses `xdg` crate.
- Default paths: `~/.config/bunnypmsl/config.toml` (Linux/macOS).
- See `README.md` for config schema.

## Deployment

- Deployment is handled via Docker or systemd.
- See `deploy/DEPLOYMENT.md` for details.
- `deploy/auto-deploy.sh` handles automated updates.
