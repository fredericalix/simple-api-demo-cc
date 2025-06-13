# Simple API Demo

A well-structured Rust-based REST API demonstration project featuring modern development practices and deployment options. Designed for learning Rust deployment on CleverCloud PaaS and testing Otoroshi reverse proxy features.

## 🏗️ Architecture

This project follows Rust best practices with a clean, modular architecture:

```
src/
├── main.rs         # Application entry point
├── lib.rs          # Library exports for testing
├── config.rs       # Configuration management
├── error.rs        # Custom error types and handling
├── handlers.rs     # HTTP request handlers
└── server.rs       # Server setup and management
```

### Key Features

- **🦀 Modern Rust**: Built with Rust 2021 edition using Actix-web framework
- **🔧 Proper Error Handling**: Custom error types with structured API responses
- **🧪 Comprehensive Testing**: Unit tests, integration tests, and test coverage
- **🌐 CORS Support**: Configured for cross-origin requests
- **📝 Extensive Documentation**: Full API documentation with examples
- **🐳 Docker Ready**: Multi-stage Docker builds with security best practices
- **🔄 Health Checks**: Built-in health monitoring endpoints
- **📊 Structured Logging**: Comprehensive request/response logging

## 🚀 Project Overview

This application runs two concurrent HTTP servers:

### Main Server (PORT: 8080)
- `GET /`: Returns "Hello world!" text response
- `GET /health`: Health check endpoint

### Application Server (PORT: 4242)
- `GET /`: Returns service status JSON with version info
- `GET /health`: Health check endpoint
- `GET /public`: Public route with JSON response and timestamp
- `GET /private`: Protected route (placeholder for authentication)

## 🛠️ Development

### Prerequisites

- Rust 1.86+ with Cargo
- Docker (optional, for containerized deployment)

### Local Development

1. **Clone and build:**
```bash
git clone <repository-url>
cd simple-api-demo-cc
cargo build
```

2. **Run tests:**
```bash
cargo test                    # Run all tests
cargo test --test integration_tests  # Run integration tests only
```

3. **Run the application:**
```bash
RUST_LOG=info cargo run
```

4. **Code quality checks:**
```bash
cargo clippy                  # Linting
cargo fmt                     # Code formatting
cargo check                   # Quick compilation check
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Main server port | 8080 |
| `PORT_APP` | Application server port | 4242 |
| `BIND_ADDRESS` | Server bind address | 0.0.0.0 |
| `RUST_LOG` | Log level | info |

## 🐳 Docker Deployment

### Quick Start

```bash
# Build and run with Docker Compose
docker-compose up --build

# Run with reverse proxy
docker-compose --profile with-proxy up --build
```

### Manual Docker Build

```bash
# Build image
docker build -t simple-api-demo .

# Run container
docker run -p 8080:8080 -p 4242:4242 \
  -e RUST_LOG=info \
  simple-api-demo
```

### Production Deployment

The Docker setup includes:
- Multi-stage builds for minimal image size
- Non-root user for security
- Health checks for monitoring
- Optional Nginx reverse proxy configuration

## 🧪 Testing

The project includes comprehensive test coverage:

- **Unit Tests**: Test individual components in isolation
- **Integration Tests**: Test complete HTTP endpoint behavior
- **Error Handling Tests**: Verify proper error responses
- **Configuration Tests**: Validate environment variable parsing

```bash
# Run all tests with coverage
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test config::tests
```

## 📁 Project Structure

### Core Modules

- **`config`**: Environment-based configuration management with validation
- **`error`**: Custom error types implementing `ResponseError` for structured API responses
- **`handlers`**: HTTP endpoint handlers organized by server type
- **`server`**: Server creation, configuration, and lifecycle management

### Best Practices Implemented

- ✅ Single Responsibility Principle (SRP)
- ✅ Proper error handling with custom types
- ✅ Comprehensive documentation with examples
- ✅ Test-driven development with high coverage
- ✅ Security-first Docker configuration
- ✅ Structured logging and monitoring
- ✅ CORS configuration for API access

## 🌐 API Examples

### Main Server Endpoints

```bash
# Hello world endpoint
curl http://localhost:8080/

# Health check
curl http://localhost:8080/health
```

### Application Server Endpoints

```bash
# Service status
curl http://localhost:4242/
# Response: {"status":"ok","service":"simple-api-demo","version":"0.1.0"}

# Public route
curl http://localhost:4242/public
# Response: {"message":"public route","access":"public","timestamp":"2024-01-15T10:30:00Z"}

# Private route
curl http://localhost:4242/private
# Response: {"message":"private and protected route","access":"private","timestamp":"2024-01-15T10:30:00Z","warning":"This route should require authentication in production"}
```

## 🚀 Deployment Options

### CleverCloud PaaS
This project is optimized for CleverCloud deployment with proper configuration management.

### Otoroshi Reverse Proxy
The dual-server setup enables testing of:
- Route mapping and service discovery
- Load balancing strategies
- Access control mechanisms
- API gateway features

### Docker Swarm/Kubernetes
The containerized setup supports orchestration platforms with proper health checks and configuration.

## 🤝 Contributing

1. Follow the established code organization patterns
2. Write tests for new functionality
3. Use descriptive commit messages following conventional commits
4. Ensure `cargo clippy` and `cargo fmt` pass
5. Update documentation for API changes

## 📄 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🔧 Development Tools

- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy`
- **Testing**: `cargo test`
- **Documentation**: `cargo doc --open`
- **Security Audit**: `cargo audit` (requires cargo-audit)

---

Built with ❤️ using Rust and modern development practices.
