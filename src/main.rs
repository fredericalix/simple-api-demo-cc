mod config;
mod error;
mod handlers;
mod server;

use config::Config;
use error::AppError;
use server::ServerManager;

/// Entry point for the simple API demo application.
/// 
/// This application starts two HTTP servers:
/// - Main server: Simple hello world endpoint  
/// - Application server: Multiple endpoints with JSON responses
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    let config = Config::from_env()
        .map_err(|e| AppError::config(format!("Failed to load configuration: {}", e)))?;
    
    // Create and start server manager
    let server_manager = ServerManager::new(config);
    server_manager.start().await
        .map_err(|e| AppError::server(format!("Failed to start servers: {}", e)))?;

    Ok(())
}
