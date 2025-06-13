use actix_web::{
    middleware::Logger,
    web, App, HttpServer,
};
use actix_cors::Cors;
use log::info;

use crate::config::Config;
use crate::handlers::{app_server, main_server};

/// Server manager responsible for creating and starting HTTP servers
/// 
/// Manages the lifecycle of both main and application servers,
/// including configuration, routing, and graceful startup.
pub struct ServerManager {
    config: Config,
}

impl ServerManager {
    /// Creates a new ServerManager with the given configuration
    /// 
    /// # Arguments
    /// * `config` - Application configuration containing server settings
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Starts both HTTP servers concurrently
    /// 
    /// Creates and binds the main server and application server,
    /// then starts them in parallel using tokio's join functionality.
    /// 
    /// # Returns
    /// Result indicating success or failure of server startup
    pub async fn start(self) -> std::io::Result<()> {
        info!("Starting servers with configuration: {:?}", self.config);

        // Create and configure both servers
        let main_server = self.create_main_server()?;
        let app_server = self.create_app_server()?;

        info!("Main server starting on {}:{}", self.config.bind_address, self.config.main_port);
        info!("Application server starting on {}:{}", self.config.bind_address, self.config.app_port);

        // Start both servers concurrently
        let result = futures::future::try_join(main_server, app_server).await;

        match result {
            Ok(_) => {
                info!("Both servers shutdown gracefully");
                Ok(())
            }
            Err(e) => {
                log::error!("Server error: {}", e);
                Err(e)
            }
        }
    }

    /// Creates and configures the main HTTP server
    /// 
    /// Sets up the main server with a simple hello world endpoint
    /// and logging middleware.
    fn create_main_server(&self) -> std::io::Result<actix_web::dev::Server> {
        let server = HttpServer::new(|| {
            App::new()
                .wrap(Self::create_cors())
                .wrap(Logger::new("%a - - [%t] \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
                .service(
                    web::scope("")
                        .route("/", web::get().to(main_server::hello))
                        .route("/health", web::get().to(main_server::hello)) // Health check endpoint
                )
        })
        .bind((self.config.bind_address.as_str(), self.config.main_port))?
        .run();
        
        Ok(server)
    }

    /// Creates and configures the application HTTP server
    /// 
    /// Sets up the application server with multiple JSON endpoints,
    /// CORS support, and logging middleware.
    fn create_app_server(&self) -> std::io::Result<actix_web::dev::Server> {
        let server = HttpServer::new(|| {
            App::new()
                .wrap(Self::create_cors())
                .wrap(Logger::new("%a - - [%t] \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
                .service(
                    web::scope("")
                        .route("/", web::get().to(app_server::root))
                        .route("/health", web::get().to(app_server::root)) // Health check endpoint
                        .route("/public", web::get().to(app_server::public_route))
                        .route("/private", web::get().to(app_server::private_route))
                )
        })
        .bind((self.config.bind_address.as_str(), self.config.app_port))?
        .run();
        
        Ok(server)
    }

    /// Creates a CORS configuration for the servers
    /// 
    /// Configures CORS to allow common methods and headers for API access.
    fn create_cors() -> Cors {
        Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_manager_creation() {
        let config = Config {
            main_port: 8080,
            app_port: 4242,
            bind_address: "127.0.0.1".to_string(),
        };

        let server_manager = ServerManager::new(config);
        assert_eq!(server_manager.config.main_port, 8080);
        assert_eq!(server_manager.config.app_port, 4242);
    }

    #[test]
    fn test_cors_creation() {
        let _cors = ServerManager::create_cors();
        // Basic test that CORS can be created without errors
        // In a real application, you might want more detailed CORS testing
    }
} 