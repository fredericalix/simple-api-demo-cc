use actix_web::{HttpResponse, Result as ActixResult};
use serde_json::json;
use serde::{Deserialize, Serialize};

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

    /// RSS news item structure
    #[derive(Serialize, Deserialize)]
    pub struct RssNewsItem {
        pub title: String,
        pub link: String,
        pub description: String,
        pub pub_date: Option<String>,
    }

    /// RSS endpoint for Hacker News
    ///
    /// Fetches the RSS feed from Hacker News and returns the news items as JSON.
    /// Returns a JSON array of news items with title, link, description, and publication date.
    pub async fn rss() -> ActixResult<HttpResponse> {
        let client = reqwest::Client::new();

        match client.get("https://news.ycombinator.com/rss").send().await {
            Ok(response) => {
                match response.text().await {
                    Ok(rss_content) => {
                        match rss_content.parse::<rss::Channel>() {
                            Ok(channel) => {
                                let news_items: Vec<RssNewsItem> = channel
                                    .items()
                                    .iter()
                                    .map(|item| RssNewsItem {
                                        title: item.title().unwrap_or("No title").to_string(),
                                        link: item.link().unwrap_or("").to_string(),
                                        description: item.description().unwrap_or("").to_string(),
                                        pub_date: item.pub_date().map(|d| d.to_string()),
                                    })
                                    .collect();

                                Ok(HttpResponse::Ok().json(json!({
                                    "status": "success",
                                    "source": "Hacker News RSS",
                                    "count": news_items.len(),
                                    "items": news_items
                                })))
                            }
                            Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                                "status": "error",
                                "message": "Failed to parse RSS feed",
                                "error": e.to_string()
                            })))
                        }
                    }
                    Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": "Failed to read RSS response",
                        "error": e.to_string()
                    })))
                }
            }
            Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch RSS feed",
                "error": e.to_string()
            })))
        }
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