use actix_web::{test, web, App, http::StatusCode};
use simple_api_demo::handlers::{app_server, main_server};
use serde_json::Value;
use std::sync::Mutex;

/// Integration tests for the application endpoints
/// 
/// These tests verify the complete behavior of HTTP endpoints
/// including request/response handling and JSON serialization.

// Use a mutex to prevent tests from running concurrently and interfering with env vars
static TEST_MUTEX: Mutex<()> = Mutex::new(());

#[actix_web::test]
async fn test_main_server_hello_endpoint() {
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(main_server::hello))
            .route("/health", web::get().to(main_server::hello))
    ).await;

    // Test root endpoint
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello world!");

    // Test health endpoint
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_app_server_endpoints() {
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(app_server::root))
            .route("/health", web::get().to(app_server::root))
            .route("/public", web::get().to(app_server::public_route))
            .route("/private", web::get().to(app_server::private_route))
    ).await;

    // Test root endpoint
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "simple-api-demo");
    assert!(body["version"].is_string());

    // Test public endpoint
    let req = test::TestRequest::get().uri("/public").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["message"], "public route");
    assert_eq!(body["access"], "public");
    assert!(body["timestamp"].is_string());

    // Test private endpoint
    let req = test::TestRequest::get().uri("/private").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["message"], "private and protected route");
    assert_eq!(body["access"], "private");
    assert!(body["timestamp"].is_string());
    assert!(body["warning"].is_string());
}

#[actix_web::test]
async fn test_app_server_content_types() {
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(app_server::root))
            .route("/public", web::get().to(app_server::public_route))
    ).await;

    // Test that JSON endpoints return proper content-type
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    
    let content_type = resp.headers().get("content-type").unwrap();
    assert!(content_type.to_str().unwrap().contains("application/json"));

    let req = test::TestRequest::get().uri("/public").to_request();
    let resp = test::call_service(&app, req).await;
    
    let content_type = resp.headers().get("content-type").unwrap();
    assert!(content_type.to_str().unwrap().contains("application/json"));
}

#[actix_web::test]
async fn test_main_server_content_type() {
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(main_server::hello))
    ).await;

    // Test that text endpoint returns proper content-type
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    
    let content_type = resp.headers().get("content-type").unwrap();
    assert!(content_type.to_str().unwrap().contains("text/plain"));
}

#[tokio::test]
async fn test_config_creation() {
    let _lock = TEST_MUTEX.lock().unwrap();
    
    // Test that config can be created with defaults
    std::env::remove_var("PORT");
    std::env::remove_var("PORT_APP");
    std::env::remove_var("BIND_ADDRESS");
    
    let config = simple_api_demo::config::Config::from_env().expect("Should create config");
    assert_eq!(config.main_port, 8080);
    assert_eq!(config.app_port, 4242);
    assert_eq!(config.bind_address, "0.0.0.0");
}

#[tokio::test]
async fn test_config_with_custom_env() {
    let _lock = TEST_MUTEX.lock().unwrap();
    
    // Test config with custom environment variables
    std::env::set_var("PORT", "9000");
    std::env::set_var("PORT_APP", "9001");
    std::env::set_var("BIND_ADDRESS", "127.0.0.1");
    
    let config = simple_api_demo::config::Config::from_env().expect("Should create config with custom values");
    assert_eq!(config.main_port, 9000);
    assert_eq!(config.app_port, 9001);
    assert_eq!(config.bind_address, "127.0.0.1");
    
    // Cleanup
    std::env::remove_var("PORT");
    std::env::remove_var("PORT_APP");
    std::env::remove_var("BIND_ADDRESS");
} 