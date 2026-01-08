# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Development Commands

```bash
# Build
cargo build                     # Debug build (both CLI + server)
cargo build --release           # Release build
cargo build --features cli --no-default-features   # CLI only (smaller binary)
cargo build --features server --no-default-features # Server only

# Run
cargo run -- serve              # Start web server (port 8000)
cargo run -- gh facebook/react  # CLI: open GitHub URL
cargo run -- --dry-run gh user  # Preview URL without opening browser
cargo run -- list               # Show all commands in terminal table

# Test & Lint
cargo test --all-features       # Run all tests
cargo test commands::github     # Run specific module tests
cargo clippy --all-features -- -D warnings  # Lint
cargo fmt --all -- --check      # Check formatting
cargo fmt                       # Auto-format
```

## Architecture

### Dual Binary with Feature Flags

The crate produces a single binary that can operate as:
- **CLI tool**: Opens URLs in browser or prints them (`--dry-run`)
- **Web server**: Rocket-based HTTP server that redirects `/?cmd=<query>` to URLs

Features in `Cargo.toml`:
- `default = ["server", "cli"]` - Both modes
- `server` - Web server (Rocket + Leptos SSR)
- `cli` - Terminal UI (tabled, open, service-manager)

### Command System (Plugin Architecture)

Commands implement the `BunnypmslCommand` trait (`src/commands/bunnypmsl_command.rs`):

```rust
pub trait BunnypmslCommand {
    const BINDINGS: &'static [&'static str];  // e.g., &["gh", "github"]
    fn process_args(args: &str) -> String;     // Returns target URL
    fn get_info() -> BunnypmslCommandInfo;     // Metadata for list/help
}
```

**Adding a new command:**
1. Create `src/commands/myservice.rs` implementing `BunnypmslCommand`
2. Add module and re-export in `src/commands/mod.rs`
3. Register in `register_commands!` macro in `src/bunnypmsl_command_registry.rs`

The registry uses `OnceLock` for O(1) HashMap lookup of commands by alias.

### Key Files

| Path | Purpose |
|------|---------|
| `src/main.rs` | CLI argument parsing (clap), subcommands (serve, service, completion) |
| `src/lib.rs` | Library exports, feature-gated module declarations |
| `src/bunnypmsl_command_registry.rs` | Central command registry with `register_commands!` macro |
| `src/commands/bunnypmsl_command.rs` | `BunnypmslCommand` trait definition |
| `src/commands/*.rs` | Individual command implementations (46 commands) |
| `src/server/mod.rs` | Rocket routes: `/?cmd=` redirect, `/health`, 404 handler |
| `src/server/web.rs` | Leptos SSR landing page with command grid |
| `src/config.rs` | `BunnypmslConfig` (TOML), XDG paths, search engine fallback |
| `src/server/service.rs` | systemd service install/uninstall (Linux only) |

### Request Flow

1. **CLI**: `args` → `resolve_command()` (aliases) → `process_command_with_config()` → URL → `open::that()`
2. **Server**: `/?cmd=query` → extract command → `process_command_with_config()` → `Redirect::to(url)`

Unrecognized commands fall through to the configured default search engine (Google/DDG/Bing).

### Configuration

TOML config at `~/.config/bunnypmsl/config.toml` (or `/etc/bunnypmsl/config.toml` for system service):
- `browser`: Override default browser
- `aliases`: Custom command shortcuts
- `default_search`: Fallback search engine
- `[server]`: Port, address, display URL for landing page

## Testing Patterns

- Unit tests live in same file as implementation (Rust convention)
- Integration tests in `tests/cli_integration.rs` use `assert_cmd`
- Command tests verify URL generation for various input patterns
- Registry tests check for binding collisions and cache correctness
