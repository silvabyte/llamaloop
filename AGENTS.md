# Agent Guidelines for llamaloop

## Build/Test Commands
- **Build**: `cargo build` or `make build`
- **Test**: `cargo test -- --nocapture` or `make test`  
- **Lint**: `cargo clippy` or `make lint`
- **Format**: `cargo fmt` or `make fmt`
- **Run**: `cargo run` or `make run`
- **Pre-commit**: `make pre-commit` (runs format check, lint, test)

## Code Style & Conventions
- **Imports**: Use `anyhow::Result` for errors, group std/external/local imports with blank lines
- **Naming**: snake_case for functions/variables, PascalCase for structs/enums, SCREAMING_SNAKE_CASE for constants
- **Error Handling**: Use `Result<T>` return types, prefer `?` operator, use `anyhow` for error context
- **Async**: Mark functions `async` when using `.await`, use `tokio::spawn` for background tasks
- **Documentation**: Doc comments on public items using `///`, inline comments sparingly
- **Types**: Use explicit types for public APIs, prefer `&str` over `String` for parameters
- **Pattern Matching**: Use exhaustive matches, prefer `match` over long `if/else` chains

## Dependencies
- UI: ratatui + crossterm for TUI
- Async: tokio runtime with full features
- HTTP: reqwest with json/stream features  
- Serialization: serde with derive feature
- Error handling: anyhow for application errors