# Build & Development Commands

## Build & Run
- Build: `cargo build`
- Run: `cargo run` or `just run`
- Format: `cargo +nightly fmt` or `just fmt`
- Lint: `cargo clippy`
- Test: `cargo test`
- Test specific: `cargo test <test_name>`

## Code Style Guidelines

### Imports
- Group imports using `StdExternalCrate` with `Module` granularity (see rustfmt.toml)
- Order: std → external crates → project modules

### Error Handling
- Use `anyhow::Result` for functions that can fail
- Prefer `?` operator for error propagation

### Naming & Structure
- Use snake_case for variables and functions
- Use PascalCase for types/structs
- Use descriptive variable names
- Add basic documentation for public functions

### Environment
- Configuration via environment variables (HTTP_HOST, HTTP_PORT, HTTP_LOG_ANSI)
- RUST_LOG determines log verbosity