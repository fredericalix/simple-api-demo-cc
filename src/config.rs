use std::env;
use crate::error::{AppError, AppResult};

/// Application configuration structure
/// 
/// Holds all configuration values loaded from environment variables
/// with sensible defaults for development.
#[derive(Debug, Clone)]
pub struct Config {
    /// Main server port (default: 8080)
    pub main_port: u16,
    /// Application server port (default: 4242)  
    pub app_port: u16,
    /// Server bind address (default: "0.0.0.0")
    pub bind_address: String,
}

impl Config {
    /// Creates a new Config instance from environment variables
    /// 
    /// # Environment Variables
    /// - `PORT`: Main server port (default: 8080)
    /// - `PORT_APP`: Application server port (default: 4242)
    /// - `BIND_ADDRESS`: Server bind address (default: "0.0.0.0")
    /// 
    /// # Errors
    /// Returns an AppError if port values cannot be parsed as valid u16 integers
    pub fn from_env() -> AppResult<Self> {
        let main_port = Self::parse_port_env("PORT", 8080)?;
        let app_port = Self::parse_port_env("PORT_APP", 4242)?;
        let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

        Ok(Config {
            main_port,
            app_port,
            bind_address,
        })
    }

    /// Parses a port value from an environment variable
    /// 
    /// # Arguments
    /// * `env_var` - Environment variable name
    /// * `default` - Default port value if env var is not set
    /// 
    /// # Returns
    /// Parsed port number or an AppError if parsing fails
    fn parse_port_env(env_var: &str, default: u16) -> AppResult<u16> {
        let port_str = env::var(env_var).unwrap_or_else(|_| default.to_string());
        
        port_str.parse::<u16>().map_err(|_| {
            AppError::environment(
                env_var,
                format!("must be a valid port number (1-65535), got: {}", port_str),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Use a mutex to prevent tests from running concurrently and interfering with env vars
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_config_from_env_with_defaults() {
        let _lock = TEST_MUTEX.lock().unwrap();
        
        // Clear environment variables to ensure defaults
        env::remove_var("PORT");
        env::remove_var("PORT_APP");
        env::remove_var("BIND_ADDRESS");

        let config = Config::from_env().expect("Should create config with defaults");
        
        assert_eq!(config.main_port, 8080);
        assert_eq!(config.app_port, 4242);
        assert_eq!(config.bind_address, "0.0.0.0");
    }

    #[test]
    fn test_config_from_env_with_custom_values() {
        let _lock = TEST_MUTEX.lock().unwrap();
        
        env::set_var("PORT", "3000");
        env::set_var("PORT_APP", "5000");
        env::set_var("BIND_ADDRESS", "127.0.0.1");

        let config = Config::from_env().expect("Should create config with custom values");
        
        assert_eq!(config.main_port, 3000);
        assert_eq!(config.app_port, 5000);
        assert_eq!(config.bind_address, "127.0.0.1");

        // Clean up
        env::remove_var("PORT");
        env::remove_var("PORT_APP");
        env::remove_var("BIND_ADDRESS");
    }

    #[test]
    fn test_config_from_env_invalid_port() {
        let _lock = TEST_MUTEX.lock().unwrap();
        
        // Clear other env vars first
        env::remove_var("PORT_APP");
        env::set_var("PORT", "invalid");

        let result = Config::from_env();
        assert!(result.is_err());
        
        // Clean up
        env::remove_var("PORT");
    }

    #[test]
    fn test_parse_port_env_valid() {
        let result = Config::parse_port_env("NONEXISTENT_PORT", 9000);
        assert_eq!(result.unwrap(), 9000);
    }

    #[test]
    fn test_parse_port_env_invalid() {
        let _lock = TEST_MUTEX.lock().unwrap();
        
        env::set_var("TEST_INVALID_PORT", "not_a_number");
        let result = Config::parse_port_env("TEST_INVALID_PORT", 9000);
        assert!(result.is_err());
        
        env::remove_var("TEST_INVALID_PORT");
    }
} 