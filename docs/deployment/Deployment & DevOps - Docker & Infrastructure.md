---
title: Deployment & DevOps - Docker & Infrastructure
type: note
permalink: deployment/deployment-dev-ops-docker-infrastructure
---

# Deployment & DevOps - Docker & Infrastructure

## 🐳 Docker Configuration

### Multi-Stage Dockerfile
Optimized for security, performance, and minimal image size.

#### Build Stage
```dockerfile
FROM rust:1.78-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build with release optimizations
RUN cargo build --release
```

#### Runtime Stage
```dockerfile
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false appuser

WORKDIR /app
COPY --from=builder /app/target/release/simple-api-demo-cc /usr/local/bin/

# Security: Run as non-root user
USER appuser

EXPOSE 8080 4242
CMD ["simple-api-demo-cc"]
```

### Security Features
- ✅ **Non-root user**: Runs as `appuser` for minimal privileges
- ✅ **Minimal base image**: debian:bookworm-slim for reduced attack surface
- ✅ **Dependency cleanup**: Removes package lists and build tools
- ✅ **CA certificates**: Proper TLS certificate handling
- ✅ **Release build**: Optimized binary for production

---

## 🐙 Docker Compose Stack

### Development Configuration
```yaml
version: '3.8'

services:
  api:
    build: .
    ports:
      - "8080:8080"
      - "4242:4242"
    environment:
      - RUST_LOG=info
      - PORT=8080
      - PORT_APP=4242
      - BIND_ADDRESS=0.0.0.0
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

### Production with Nginx Reverse Proxy
```yaml
services:
  api:
    # ... same as above ...
    expose:
      - "8080"
      - "4242"
    # Remove direct port mapping for security

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro  # SSL certificates
    depends_on:
      - api
    restart: unless-stopped
```

---

## 🌐 Nginx Reverse Proxy

### Configuration Overview
```nginx
upstream api_main {
    server api:8080;
}

upstream api_app {
    server api:4242;
}

server {
    listen 80;
    server_name your-domain.com;
    
    # Main API (Port 8080)
    location /main/ {
        proxy_pass http://api_main/;
        include /etc/nginx/proxy_params;
    }
    
    # Application API (Port 4242)
    location /api/ {
        proxy_pass http://api_app/;
        include /etc/nginx/proxy_params;
    }
}
```

### Security Headers
```nginx
# Security headers
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header Referrer-Policy "no-referrer-when-downgrade" always;
add_header Content-Security-Policy "default-src 'self'" always;
```

### Performance Optimizations
```nginx
# Gzip compression
gzip on;
gzip_vary on;
gzip_min_length 1024;
gzip_types text/plain application/json application/javascript text/css;

# Connection optimization
keepalive_timeout 65;
keepalive_requests 100;

# Buffer optimization
client_body_buffer_size 128k;
client_max_body_size 10m;
```

---

## ☁️ Cloud Deployment Options

### CleverCloud PaaS

#### Configuration
```json
{
  "type": "rust",
  "build": {
    "type": "cargo"
  },
  "deploy": {
    "container": "rust",
    "memory": 512,
    "instances": {
      "min": 1,
      "max": 3
    }
  }
}
```

#### Environment Variables
```bash
RUST_LOG=info
PORT=8080
PORT_APP=4242
BIND_ADDRESS=0.0.0.0
```

### Docker Hub Deployment
```bash
# Build and tag
docker build -t simple-api-demo:latest .

# Tag for registry
docker tag simple-api-demo:latest your-registry/simple-api-demo:v1.0.0

# Push to registry
docker push your-registry/simple-api-demo:v1.0.0
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: simple-api-demo
spec:
  replicas: 3
  selector:
    matchLabels:
      app: simple-api-demo
  template:
    metadata:
      labels:
        app: simple-api-demo
    spec:
      containers:
      - name: api
        image: simple-api-demo:latest
        ports:
        - containerPort: 8080
        - containerPort: 4242
        env:
        - name: RUST_LOG
          value: "info"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
```

---

## 🔍 Monitoring & Observability

### Health Checks
```bash
# Docker health check
HEALTHCHECK --interval=30s --timeout=10s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# Kubernetes liveness probe
livenessProbe:
  httpGet:
    path: /health
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10
```

### Logging Configuration
```bash
# Environment variables for logging
RUST_LOG=simple_api_demo_cc=info,actix_web=info

# Docker logging driver
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```

### Metrics Collection
```nginx
# Nginx access logs for metrics
log_format json_combined escape=json
  '{'
    '"time_local":"$time_local",'
    '"remote_addr":"$remote_addr",'
    '"request":"$request",'
    '"status":$status,'
    '"body_bytes_sent":$body_bytes_sent,'
    '"request_time":$request_time,'
    '"upstream_response_time":"$upstream_response_time"'
  '}';

access_log /var/log/nginx/access.log json_combined;
```

---

## 🚀 Deployment Commands

### Local Development
```bash
# Development with hot reload
cargo watch -x run

# Build and test
cargo build --release
cargo test

# Docker development
docker-compose up --build

# Docker production
docker-compose -f docker-compose.prod.yml up -d
```

### Production Deployment
```bash
# Build production image
docker build -t simple-api-demo:$(git rev-parse --short HEAD) .

# Deploy with zero downtime
docker-compose -f docker-compose.prod.yml pull
docker-compose -f docker-compose.prod.yml up -d --no-deps

# Verify deployment
curl -f http://your-domain.com/api/health
```

### Rollback Strategy
```bash
# Tag current deployment
docker tag simple-api-demo:latest simple-api-demo:rollback

# Deploy previous version
docker-compose -f docker-compose.prod.yml down
docker-compose -f docker-compose.prod.yml up -d simple-api-demo:previous-tag
```

---

## 📊 Infrastructure as Code

### Docker Ignore Optimization
```gitignore
# .dockerignore
target/
Dockerfile*
docker-compose*
.git
.gitignore
README.md
.env
*.log
```

### CI/CD Pipeline Example
```yaml
# GitHub Actions / GitLab CI
name: Deploy
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    
    - name: Build Docker image
      run: docker build -t simple-api-demo:${{ github.sha }} .
    
    - name: Run tests
      run: docker run --rm simple-api-demo:${{ github.sha }} cargo test
    
    - name: Deploy to production
      run: |
        docker tag simple-api-demo:${{ github.sha }} simple-api-demo:latest
        # Deploy commands here
```

---

## 🔧 Environment Configuration

### Development (.env)
```bash
RUST_LOG=debug
PORT=8080
PORT_APP=4242
BIND_ADDRESS=127.0.0.1
```

### Production
```bash
RUST_LOG=info
PORT=8080
PORT_APP=4242
BIND_ADDRESS=0.0.0.0
```

### Container Limits
```yaml
# Resource constraints
deploy:
  resources:
    limits:
      cpus: '0.5'
      memory: 256M
    reservations:
      cpus: '0.25'
      memory: 128M
```