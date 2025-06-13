use actix_web::{HttpResponse, Result as ActixResult};
use serde_json::json;

/// Main server handlers
pub mod main_server {
    use super::*;

    /// Hello world endpoint for the main server
    /// 
    /// Returns a simple "Hello world!" text response.
    /// This endpoint is designed for basic health checks and testing.
    pub async fn hello() -> ActixResult<HttpResponse> {
        Ok(HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body("Hello world!"))
    }
}

/// Application server handlers  
pub mod app_server {
    use super::*;

    /// Root endpoint for the application server
    /// 
    /// Returns a JSON response indicating the service status.
    /// Used for health checks and service discovery.
    pub async fn root() -> ActixResult<HttpResponse> {
        Ok(HttpResponse::Ok().json(json!({
            "status": "ok",
            "service": "simple-api-demo",
            "version": env!("CARGO_PKG_VERSION")
        })))
    }

    /// Public route endpoint
    /// 
    /// Returns a JSON response for publicly accessible content.
    /// This route does not require authentication.
    pub async fn public_route() -> ActixResult<HttpResponse> {
        Ok(HttpResponse::Ok().json(json!({
            "message": "public route",
            "access": "public",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    }

    /// Private route endpoint
    /// 
    /// Returns a JSON response for protected content.
    /// In a real application, this would require authentication.
    pub async fn private_route() -> ActixResult<HttpResponse> {
        Ok(HttpResponse::Ok().json(json!({
            "message": "private and protected route",
            "access": "private",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "warning": "This route should require authentication in production"
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_main_server_hello() {
        let response = main_server::hello().await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[actix_web::test]
    async fn test_app_server_root() {
        let response = app_server::root().await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[actix_web::test]
    async fn test_app_server_public_route() {
        let response = app_server::public_route().await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[actix_web::test]
    async fn test_app_server_private_route() {
        let response = app_server::private_route().await.unwrap();
        assert_eq!(response.status(), 200);
    }
} 