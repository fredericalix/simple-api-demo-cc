use actix_web::{HttpResponse, ResponseError};
use std::fmt::Display;
use thiserror::Error;

/// Application-specific error types
/// 
/// This enum defines all possible errors that can occur in the application,
/// providing structured error handling with appropriate HTTP status codes.
#[derive(Error, Debug)]
pub enum AppError {
    /// Configuration-related errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Server startup or runtime errors
    #[error("Server error: {message}")]
    Server { message: String },

    /// Environment variable parsing errors
    #[error("Environment variable error: {var_name} - {message}")]
    Environment { var_name: String, message: String },

    /// Generic internal server errors
    #[error("Internal server error: {message}")]
    Internal { message: String },

    /// Validation errors for request data
    #[error("Validation error: {message}")]
    Validation { message: String },
}

impl AppError {
    /// Creates a new configuration error
    pub fn config<T: Display>(message: T) -> Self {
        Self::Config {
            message: message.to_string(),
        }
    }

    /// Creates a new server error
    pub fn server<T: Display>(message: T) -> Self {
        Self::Server {
            message: message.to_string(),
        }
    }

    /// Creates a new environment variable error
    pub fn environment<T: Display, U: Display>(var_name: T, message: U) -> Self {
        Self::Environment {
            var_name: var_name.to_string(),
            message: message.to_string(),
        }
    }

    /// Creates a new internal error
    pub fn internal<T: Display>(message: T) -> Self {
        Self::Internal {
            message: message.to_string(),
        }
    }

    /// Creates a new validation error
    pub fn validation<T: Display>(message: T) -> Self {
        Self::Validation {
            message: message.to_string(),
        }
    }
}

impl ResponseError for AppError {
    /// Returns the appropriate HTTP status code for each error type
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Config { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Server { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Environment { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation { .. } => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }

    /// Returns a JSON error response for API consumers
    fn error_response(&self) -> HttpResponse {
        let error_json = serde_json::json!({
            "error": {
                "type": self.error_type(),
                "message": self.to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        HttpResponse::build(self.status_code()).json(error_json)
    }
}

impl AppError {
    /// Returns a string identifier for the error type
    fn error_type(&self) -> &'static str {
        match self {
            AppError::Config { .. } => "configuration_error",
            AppError::Server { .. } => "server_error",
            AppError::Environment { .. } => "environment_error",
            AppError::Internal { .. } => "internal_error",
            AppError::Validation { .. } => "validation_error",
        }
    }
}

/// Convenient Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_creation() {
        let config_error = AppError::config("Invalid port");
        assert!(matches!(config_error, AppError::Config { .. }));
        
        let server_error = AppError::server("Failed to bind");
        assert!(matches!(server_error, AppError::Server { .. }));
        
        let env_error = AppError::environment("PORT", "Not a number");
        assert!(matches!(env_error, AppError::Environment { .. }));
    }

    #[test]
    fn test_error_status_codes() {
        let config_error = AppError::config("test");
        assert_eq!(config_error.status_code(), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        
        let validation_error = AppError::validation("test");
        assert_eq!(validation_error.status_code(), actix_web::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_error_types() {
        let config_error = AppError::config("test");
        assert_eq!(config_error.error_type(), "configuration_error");
        
        let validation_error = AppError::validation("test");
        assert_eq!(validation_error.error_type(), "validation_error");
    }
} 