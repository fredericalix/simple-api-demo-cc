# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Development Commands
- `cargo run` - Start the application server on port 8080
- `cargo test` - Run all unit tests
- `cargo test --test integration_tests` - Run integration tests only
- `cargo clippy` - Run linting
- `cargo fmt` - Format code
- `cargo build --release` - Build release binary
- `cargo watch -x run` - Development with hot reload

### Quality Gates
Before committing, ensure all of these pass:
- `cargo test` - All tests pass
- `cargo clippy` - No warnings
- `cargo fmt --check` - Code is formatted
- `cargo build --release` - Release build succeeds

## Architecture

This is a Rust-based REST API using actix-web framework, designed for deployment on Clever Cloud with Otoroshi reverse proxy integration.

### Project Structure
- `src/main.rs` - Entry point, starts dual server setup
- `src/lib.rs` - Library exports for all modules
- `src/server.rs` - ServerManager handles dual server architecture
- `src/handlers.rs` - HTTP request handlers for API endpoints
- `src/config.rs` - Environment-based configuration management
- `src/error.rs` - Custom error types and handling
- `tests/integration_tests.rs` - Integration test suite

### Key Design Patterns
- **Dual Server Architecture**: Main server (simple endpoints) + Application server (JSON API)
- **Environment Configuration**: All settings loaded from environment variables via Config::from_env()
- **Structured Error Handling**: Custom AppError types using thiserror crate
- **Modular Design**: Clear separation between handlers, config, server management, and errors

### API Endpoints
- `GET /` - Welcome message
- `GET /hello` - Hello world response
- `POST /echo` - JSON echo service

### Development Standards
- All code, comments, and documentation must be in English only
- Follow Rust best practices and conventions from .cursorrules
- Use Result<T, E> for error handling, never unwrap() in production
- Apply Single Responsibility Principle to all functions and modules
- Write comprehensive doc comments (///) for public functions