---
title: Development Workflow & Best Practices
type: note
permalink: development/development-workflow-best-practices
---

# Development Workflow & Best Practices

## 🔄 Development Lifecycle

### Daily Development Workflow
```bash
# 1. Start development session
git pull origin main
cargo update

# 2. Run tests and linting
cargo test
cargo clippy
cargo fmt

# 3. Development with hot reload
cargo watch -x run

# 4. Verify changes
cargo build --release
cargo test --release
```

### Code Quality Gates
- ✅ **All tests pass**: `cargo test`
- ✅ **No clippy warnings**: `cargo clippy`
- ✅ **Formatted code**: `cargo fmt --check`
- ✅ **Release build works**: `cargo build --release`
- ✅ **Integration tests pass**: `cargo test --test integration_tests`

---

## 📋 Code Review Checklist

### Architecture & Design
- [ ] **Single Responsibility**: Each function/module has one clear purpose
- [ ] **Error Handling**: Proper use of `Result<T, E>` and custom error types
- [ ] **Documentation**: All public functions have doc comments (`///`)
- [ ] **Naming**: Clear, descriptive names in English only
- [ ] **Dependencies**: Justified use of external crates

### Rust-Specific
- [ ] **Ownership**: Appropriate use of borrowing vs. owned values
- [ ] **Async/Await**: Proper async function usage for I/O operations
- [ ] **Memory Safety**: No `unwrap()` in production code
- [ ] **Performance**: Use of efficient data structures and algorithms
- [ ] **Traits**: Proper implementation of standard traits (Debug, Clone, etc.)

### API Design
- [ ] **RESTful**: Proper HTTP methods and status codes
- [ ] **Consistent**: Uniform response formats across endpoints
- [ ] **CORS**: Appropriate cross-origin configuration
- [ ] **Security**: Input validation and sanitization
- [ ] **Logging**: Structured logging without sensitive data

### Testing
- [ ] **Coverage**: New code has corresponding tests
- [ ] **Integration**: HTTP endpoints tested end-to-end
- [ ] **Error Cases**: Both success and failure paths tested
- [ ] **Isolation**: Tests don't interfere with each other

---

## 🧪 Testing Workflow

### Test-Driven Development (TDD)
```bash
# 1. Write failing test
cargo test test_new_feature -- --nocapture

# 2. Implement minimal code to pass
# ... code implementation ...

# 3. Refactor while keeping tests green
cargo test && cargo clippy

# 4. Add more test cases
cargo test -- --nocapture
```

### Test Categories

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature_specific_behavior() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result.unwrap(), expected_value);
    }
}
```

#### Integration Tests
```rust
#[actix_web::test]
async fn test_endpoint_behavior() {
    let app = test::init_service(
        App::new().route("/endpoint", web::get().to(handler))
    ).await;
    
    let req = test::TestRequest::get().uri("/endpoint").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
}
```

---

## 🔧 Environment Setup

### Development Dependencies
```bash
# Core development tools
cargo install cargo-watch
cargo install cargo-tarpaulin  # Code coverage
cargo install cargo-audit      # Security audits
cargo install cargo-outdated   # Dependency updates
```

### IDE Configuration (VS Code)
```json
{
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### Git Hooks
```bash
# .git/hooks/pre-commit
#!/bin/sh
set -e

echo "Running pre-commit checks..."
cargo fmt --check
cargo clippy -- -D warnings
cargo test

echo "All checks passed!"
```

---

## 🚀 Deployment Pipeline

### Continuous Integration
```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Check formatting
      run: cargo fmt --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Run tests
      run: cargo test
    
    - name: Build release
      run: cargo build --release
```

### Deployment Stages
1. **Development**: Local testing with hot reload
2. **Testing**: Automated CI/CD pipeline
3. **Staging**: Docker deployment with production config
4. **Production**: Full infrastructure deployment

---

## 📝 Documentation Standards

### Code Documentation
```rust
/// Handles HTTP requests to the public route endpoint.
/// 
/// This endpoint is designed to be accessible without authentication
/// and returns basic information about the request context.
/// 
/// # Returns
/// 
/// Returns an HTTP 200 response with JSON containing:
/// - `message`: Static text "public route"
/// - `access`: Access level identifier ("public")
/// - `timestamp`: ISO 8601 formatted current timestamp
/// 
/// # Examples
/// 
/// ```
/// use actix_web::test;
/// 
/// let resp = public_route().await;
/// assert!(resp.status().is_success());
/// ```
pub async fn public_route() -> ActixResult<HttpResponse> {
    // Implementation...
}
```

### API Documentation
- **OpenAPI/Swagger**: Generate API specs from code
- **Examples**: Include request/response samples
- **Error Cases**: Document all possible error responses
- **Authentication**: Clear auth requirements

### Architecture Documentation
- **Module Purpose**: Clear explanation of each module's role
- **Data Flow**: How requests flow through the system
- **Dependencies**: External service integrations
- **Configuration**: Environment variable documentation

---

## 🔒 Security Practices

### Input Validation
```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    username: String,
    
    #[validate(email)]
    email: String,
}

async fn create_user(data: web::Json<CreateUserRequest>) -> ActixResult<HttpResponse> {
    data.validate().map_err(|e| AppError::validation(e.to_string()))?;
    // Process validated data...
}
```

### Error Handling Security
```rust
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: ErrorDetail {
                error_type: self.error_type(),
                message: self.sanitized_message(), // Never expose internal details
                timestamp: Utc::now(),
            },
        };
        
        HttpResponse::build(self.status_code()).json(error_response)
    }
}
```

### Logging Security
```rust
// ✅ Good: Log without sensitive data
log::info!("User login attempt for user_id: {}", user_id);

// ❌ Bad: Never log passwords or tokens
log::info!("Login with password: {}", password); // DON'T DO THIS
```

---

## 📊 Performance Optimization

### Database Connections
```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(20)
    .acquire_timeout(Duration::from_secs(3))
    .connect(&database_url)
    .await?;
```

### Async Best Practices
```rust
// ✅ Use async for I/O-bound operations
async fn fetch_user_data(id: u32) -> Result<UserData, AppError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::database(e.to_string()))?;
    
    Ok(UserData::from(user))
}

// ✅ Use spawn for CPU-bound work
async fn process_heavy_computation(data: Vec<u8>) -> Result<ProcessedData, AppError> {
    let result = tokio::task::spawn_blocking(move || {
        // CPU-intensive processing here
        heavy_computation(data)
    })
    .await
    .map_err(|e| AppError::internal(e.to_string()))?;
    
    Ok(result)
}
```

### Memory Management
```rust
// ✅ Use references when possible
fn process_data(data: &[u8]) -> ProcessedData {
    // Process without cloning data
}

// ✅ Use Cow for flexible ownership
use std::borrow::Cow;

fn format_message(template: &str, value: Option<&str>) -> Cow<str> {
    match value {
        Some(v) => Cow::Owned(template.replace("{}", v)),
        None => Cow::Borrowed(template),
    }
}
```

---

## 🎯 Monitoring & Observability

### Structured Logging
```rust
use serde_json::json;

log::info!(
    "{}",
    json!({
        "event": "request_completed",
        "method": req.method().as_str(),
        "path": req.path(),
        "status": resp.status().as_u16(),
        "duration_ms": duration.as_millis(),
        "user_id": user_id,
    })
);
```

### Health Check Implementation
```rust
async fn health_check() -> ActixResult<HttpResponse> {
    let health_status = json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": get_uptime(),
    });
    
    Ok(HttpResponse::Ok().json(health_status))
}
```

### Metrics Collection
- **Request Duration**: Track API response times
- **Error Rates**: Monitor error frequency by type
- **Resource Usage**: CPU, memory, and connection pool metrics
- **Business Metrics**: Domain-specific KPIs