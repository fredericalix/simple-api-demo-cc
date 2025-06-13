---
title: API Documentation - Endpoints & Responses
type: note
permalink: api/api-documentation-endpoints-responses
---

# API Documentation - Endpoints & Responses

## 🌐 Server Overview

The application runs two concurrent HTTP servers with different purposes:

### 📡 Main Server (Port 8080)
Simple text-based endpoints for basic health checks and testing.

### 🔧 Application Server (Port 4242)
JSON API with structured responses and metadata.

---

## 🔗 Main Server Endpoints

### `GET /`
**Description:** Hello world endpoint for basic connectivity testing

**Response:**
```
Content-Type: text/plain; charset=utf-8
Status: 200 OK

Hello world!
```

**Usage:**
```bash
curl http://localhost:8080/
# Response: Hello world!
```

### `GET /health`
**Description:** Health check endpoint for monitoring and load balancers

**Response:**
```
Content-Type: text/plain; charset=utf-8
Status: 200 OK

Hello world!
```

**Usage:**
```bash
curl http://localhost:8080/health
# Response: Hello world!
```

---

## 🎯 Application Server Endpoints

### `GET /`
**Description:** Service status endpoint with version information

**Response:**
```json
{
  "status": "ok",
  "service": "simple-api-demo",
  "version": "0.1.0"
}
```

**Headers:**
```
Content-Type: application/json
Status: 200 OK
Access-Control-Allow-Origin: *
```

**Usage:**
```bash
curl http://localhost:4242/
```

### `GET /health`
**Description:** Health check endpoint returning service status

**Response:** Same as `GET /`

**Usage:**
```bash
curl http://localhost:4242/health
```

### `GET /public`
**Description:** Public route accessible without authentication

**Response:**
```json
{
  "message": "public route",
  "access": "public",
  "timestamp": "2024-01-15T10:30:00.123Z"
}
```

**Usage:**
```bash
curl http://localhost:4242/public
```

### `GET /private`
**Description:** Protected route (placeholder for authentication)

**Response:**
```json
{
  "message": "private and protected route",
  "access": "private",
  "timestamp": "2024-01-15T10:30:00.123Z",
  "warning": "This route should require authentication in production"
}
```

**Usage:**
```bash
curl http://localhost:4242/private
```

---

## 🚨 Error Responses

All API errors return structured JSON responses:

### Error Format
```json
{
  "error": {
    "type": "error_type_identifier",
    "message": "Human-readable error description",
    "timestamp": "2024-01-15T10:30:00.123Z"
  }
}
```

### Error Types

| Type | HTTP Status | Description |
|------|-------------|-------------|
| `configuration_error` | 500 | Configuration loading failed |
| `server_error` | 500 | Server startup or runtime error |
| `environment_error` | 500 | Environment variable parsing error |
| `internal_error` | 500 | Unexpected internal error |
| `validation_error` | 400 | Request validation failed |

### Example Error Response
```bash
# Invalid port configuration
curl http://localhost:4242/
```

```json
{
  "error": {
    "type": "environment_error",
    "message": "Environment variable error: PORT - must be a valid port number (1-65535), got: invalid",
    "timestamp": "2024-01-15T10:30:00.123Z"
  }
}
```

---

## 🌍 CORS Configuration

### Allowed Origins
- `*` (all origins for development)

### Allowed Methods
- `GET`
- `POST`
- `PUT`
- `DELETE`
- `OPTIONS`

### Allowed Headers
- `Authorization`
- `Accept`
- `Content-Type`

### Cache Control
- `max-age: 3600` (1 hour)

---

## 📊 Response Headers

### Standard Headers
All responses include:
```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: Accept, Authorization, Content-Type
```

### Content Types
- **Main Server**: `text/plain; charset=utf-8`
- **Application Server**: `application/json`

---

## 🧪 Testing Examples

### Health Check Monitoring
```bash
# Check both servers
curl -f http://localhost:8080/health && echo "Main server OK"
curl -f http://localhost:4242/health && echo "App server OK"
```

### JSON API Testing
```bash
# Get service status with version
curl -H "Accept: application/json" http://localhost:4242/ | jq .

# Test public endpoint
curl http://localhost:4242/public | jq '.timestamp'

# Test private endpoint warning
curl http://localhost:4242/private | jq '.warning'
```

### CORS Testing
```bash
# Test preflight request
curl -X OPTIONS http://localhost:4242/ \
  -H "Origin: https://example.com" \
  -H "Access-Control-Request-Method: GET" \
  -v
```