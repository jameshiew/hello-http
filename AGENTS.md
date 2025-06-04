# Build & Development Commands

## Build & Run

- Build: `cargo build`
- Run: `cargo run` or `just run`
- Format: `cargo +nightly fmt` or `just fmt`
- Lint: `cargo clippy`
- Test: `cargo test`
- Test specific: `cargo test <test_name>`

Format code as a final step after finishing editing code.

## Code Style Guidelines

### Error Handling

- Use `anyhow::Result` for functions that can fail
- Prefer `?` operator for error propagation

### Documentation

- Add basic documentation for public functions

### Environment

- Configuration via environment variables (HTTP_HOST, HTTP_PORT, HTTP_LOG_ANSI)
- RUST_LOG determines log verbosity
