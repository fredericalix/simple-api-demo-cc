---
title: Getting Started Guide
type: note
permalink: getting-started/getting-started-guide
---

# Getting Started Guide

## 🚀 Quick Start

### Prerequisites
- **Rust**: 1.78+ (Install via [rustup.rs](https://rustup.rs/))
- **Docker**: For containerized deployment (optional)
- **Git**: For version control

### Installation & Setup
```bash
# Clone the repository
git clone <repository-url>
cd simple-api-demo-cc

# Install dependencies and build
cargo build

# Run tests to verify setup
cargo test

# Start the application
cargo run
```

**Expected Output:**
```
[INFO] Starting Simple API Demo
[INFO] Configuration: main_port=8080, app_port=4242, bind_address=0.0.0.0
[INFO] Main server starting on http://0.0.0.0:8080
[INFO] Application server starting on http://0.0.0.0:4242
[INFO] Both servers started successfully
```

---

## 🎯 First Steps

### 1. Verify Installation
```bash
# Test both servers are running
curl http://localhost:8080/
# Expected: "Hello world!"

curl http://localhost:4242/
# Expected: {"status":"ok","service":"simple-api-demo","version":"0.1.0"}
```

### 2. Explore the API
```bash
# Health checks
curl http://localhost:8080/health
curl http://localhost:4242/health

# JSON endpoints
curl http://localhost:4242/public
curl http://localhost:4242/private
```

### 3. Check Logs
The application uses structured logging. You'll see output like:
```
[INFO] simple_api_demo_cc::server: Main server configured with CORS
[INFO] actix_web::middleware::logger: 127.0.0.1 "GET / HTTP/1.1" 200 13
```

---

## 🔧 Configuration

### Environment Variables
| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8080 | Main server port |
| `PORT_APP` | 4242 | Application server port |
| `BIND_ADDRESS` | 0.0.0.0 | Server bind address |
| `RUST_LOG` | info | Logging level |

### Development Configuration
```bash
# Create .env file for development
cat > .env << EOF
RUST_LOG=debug
PORT=8080
PORT_APP=4242
BIND_ADDRESS=127.0.0.1
EOF

# Load environment and run
source .env
cargo run
```

### Production Configuration
```bash
export RUST_LOG=info
export PORT=8080
export PORT_APP=4242
export BIND_ADDRESS=0.0.0.0
```

---

## 🧪 Development Workflow

### 1. Set Up Development Environment
```bash
# Install development tools
cargo install cargo-watch
cargo install cargo-tarpaulin

# Install VS Code extensions (if using VS Code)
# - rust-analyzer
# - Better TOML
# - Error Lens
```

### 2. Development Commands
```bash
# Hot reload during development
cargo watch -x run

# Run tests with coverage
cargo tarpaulin --out html

# Lint and format
cargo clippy
cargo fmt

# Build release version
cargo build --release
```

### 3. Testing Workflow
```bash
# Run all tests
cargo test

# Run specific test module
cargo test config::tests

# Run integration tests only
cargo test --test integration_tests

# Test with output
cargo test -- --nocapture
```

---

## 🐳 Docker Development

### Quick Docker Setup
```bash
# Build Docker image
docker build -t simple-api-demo .

# Run container
docker run -p 8080:8080 -p 4242:4242 simple-api-demo

# Or use Docker Compose
docker-compose up --build
```

### Development with Docker
```bash
# Development with volume mounting
docker run -it --rm \
  -v $(pwd):/app \
  -w /app \
  -p 8080:8080 -p 4242:4242 \
  rust:1.78-slim \
  bash

# Inside container:
cargo run
```

---

## 📁 Project Structure Guide

```
simple-api-demo-cc/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── lib.rs             # Library exports
│   ├── config.rs          # Configuration management
│   ├── error.rs           # Custom error types
│   ├── handlers.rs        # HTTP handlers
│   └── server.rs          # Server setup
├── tests/                 # Integration tests
│   └── integration_tests.rs
├── docs/                  # Documentation (Basic Memory)
├── Cargo.toml            # Rust dependencies
├── Dockerfile            # Container build
├── docker-compose.yml    # Multi-container setup
└── README.md            # Project documentation
```

### Key Files to Know
- **`src/main.rs`**: Application entry point - start here
- **`src/handlers.rs`**: Add new API endpoints here
- **`src/config.rs`**: Modify configuration options here
- **`tests/integration_tests.rs`**: Add new endpoint tests here

---

## 🔍 Common Tasks

### Adding a New Endpoint
1. **Add handler function** in `src/handlers.rs`:
```rust
pub async fn my_new_endpoint() -> ActixResult<HttpResponse> {
    let response = json!({
        "message": "My new endpoint",
        "timestamp": Utc::now()
    });
    Ok(HttpResponse::Ok().json(response))
}
```

2. **Register route** in `src/server.rs`:
```rust
.route("/my-endpoint", web::get().to(handlers::app_server::my_new_endpoint))
```

3. **Add test** in `tests/integration_tests.rs`:
```rust
#[actix_web::test]
async fn test_my_new_endpoint() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/my-endpoint").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```

### Adding Configuration Options
1. **Update Config struct** in `src/config.rs`:
```rust
pub struct Config {
    // ... existing fields ...
    pub new_option: String,
}
```

2. **Add parsing logic**:
```rust
impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        // ... existing code ...
        let new_option = std::env::var("NEW_OPTION")
            .unwrap_or_else(|_| "default_value".to_string());
        // ...
    }
}
```

3. **Add test for new configuration**

### Debugging Issues
```bash
# Increase log verbosity
RUST_LOG=debug cargo run

# Check configuration parsing
RUST_LOG=simple_api_demo_cc::config=debug cargo run

# Run specific failing test
cargo test test_name -- --nocapture

# Check dependencies
cargo tree
```

---

## 🚨 Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Error: Address already in use (os error 98)

# Solution: Find and kill process
lsof -ti:8080 | xargs kill -9
lsof -ti:4242 | xargs kill -9

# Or use different ports
PORT=8081 PORT_APP=4243 cargo run
```

#### Environment Variables Not Loading
```bash
# Check current environment
env | grep -E "PORT|RUST_LOG"

# Load from .env file
source .env
cargo run

# Or use dotenv in code (already included)
```

#### Tests Failing Due to Environment Conflicts
```bash
# Run tests sequentially to avoid conflicts
cargo test -- --test-threads=1

# Or use the built-in mutex synchronization
cargo test  # Should work fine with current implementation
```

#### Docker Build Issues
```bash
# Clear Docker cache
docker system prune -a

# Build with verbose output
docker build --no-cache -t simple-api-demo .

# Check for dependency issues
docker run --rm -it rust:1.78-slim cargo --version
```

### Getting Help
1. **Check logs**: Look for error messages in console output
2. **Review tests**: Tests often show expected behavior
3. **Documentation**: Check the `/docs` folder (Basic Memory project)
4. **Rust community**: [users.rust-lang.org](https://users.rust-lang.org)

---

## 🎯 Next Steps

### Learn the Codebase
1. **Start with `/api` endpoints** - understand the API structure
2. **Review `/testing` docs** - see how testing works
3. **Explore `/architecture`** - understand module organization
4. **Check `/deployment`** - learn deployment options

### Extend the Application
1. **Add database integration** (PostgreSQL/SQLite)
2. **Implement authentication** (JWT tokens)
3. **Add input validation** (serde validators)
4. **Create more complex endpoints** (CRUD operations)

### Production Deployment
1. **Review `/deployment` documentation**
2. **Configure monitoring and logging**
3. **Set up CI/CD pipeline**
4. **Implement security best practices**

---

## 📚 Learning Resources

### Rust-Specific
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Actix-web Documentation](https://actix.rs/)

### API Development
- [REST API Design Guide](https://restfulapi.net/)
- [HTTP Status Codes](https://httpstatuses.com/)
- [API Security Best Practices](https://owasp.org/www-project-api-security/)

### Testing
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Integration Testing Patterns](https://doc.rust-lang.org/book/ch11-03-test-organization.html)