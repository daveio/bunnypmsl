# bunnypmsl Agent Guide

This repository contains `bunnypmsl`, a Rust-based smart browser bookmarking tool (bunny1 clone). It operates as both a CLI tool and a web server.

## ⚡️ Quick Start

- **Build**: `cargo build`
- **Run CLI**: `cargo run -- <command> <args>` (e.g., `cargo run -- gh bunnypmsl`)
- **Run Server**: `cargo run -- serve`
- **Test**: `cargo test`

## 📂 Project Structure

- `src/main.rs`: Entry point. Dispatches to CLI or Server modes.
- `src/lib.rs`: Shared library logic.
- `src/commands/`: **Core logic**. Contains individual command implementations (e.g., `github.rs`, `google.rs`).
- `src/bunnypmsl_command_registry.rs`: **Registry**. Maps command strings to their implementations.
- `src/server/`: Web server implementation (Rocket framework).
- `src/server/templates/`: HTML templates (Askama).
- `deploy/`: Deployment scripts and documentation.

## 🛠 Common Tasks

### Adding a New Command

To add a new command (e.g., `mynewservice`), you must modify three files:

1.  **Create Implementation**: Add `src/commands/mynewservice.rs`.
    - Implement `BunnypmslCommand` trait.
    - Define `BINDINGS` (aliases like `["mns", "myservice"]`).
    - Implement `process_args` (URL generation).
    - Implement `get_info` (metadata for help/docs).

2.  **Export Module**: Edit `src/commands/mod.rs`.
    - Add `pub mod mynewservice;`
    - Add `pub use mynewservice::MyNewServiceCommand;`

3.  **Register Command**: Edit `src/bunnypmsl_command_registry.rs`.
    - Add `crate::commands::MyNewServiceCommand` to the `register_commands!` macro list.

### Web Server & Templates

- The web server uses **Rocket**.
- HTML is rendered using **Askama** (compile-time templates).
- Templates are in `src/server/templates/`.
- **Note**: If you modify a `.html` template, you must recompile the Rust code to see changes.

## 🧪 Testing

- **Unit Tests**: Place in the command file itself inside a `mod tests` block.
- **Integration Tests**: `tests/cli_integration.rs`.
- Run all tests: `cargo test`

## 🚀 Deployment

- **Docker**: `docker-compose up -d` (uses `Dockerfile`).
- **Systemd (Linux)**: See `deploy/DEPLOYMENT.md`.
- **Config**:
  - Local dev: `~/.config/bunnypmsl/config.toml` (Mac/Linux) or `%APPDATA%\bunnypmsl\config.toml` (Windows).
  - Docker: Configured via env vars or mounted volumes.

## ⚠️ Gotchas & Patterns

- **Symlinks**: `CLAUDE.md`, `GEMINI.md`, `JULES.md`, `WARP.md` are symlinks to this file (`AGENTS.md`).
- **Command Registry**: Forgot to register your command? It won't work even if the code compiles. Check `src/bunnypmsl_command_registry.rs`.
- **URL Encoding**: Use `crate::utils::url_encoding::build_path_url` helper for safe URL construction.
- **Optional Dependencies**: The project uses features `cli` and `server`. By default both are enabled.
