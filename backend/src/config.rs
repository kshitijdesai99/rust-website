use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub server_host: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "backend.db".to_string()),
            server_port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            server_host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        }
    }

    /// Get server address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}
