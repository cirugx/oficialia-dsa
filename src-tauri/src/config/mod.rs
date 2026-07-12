//! Configuration Module - Application configuration

use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub version: String,
    pub database: DatabaseConfig,
    pub ai: AiConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub provider: String, // "memory", "sqlite", "postgres"
    pub connection_string: Option<String>,
}

/// AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub gemini_api_key: Option<String>,
    pub enabled: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "Oficialia DSA".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database: DatabaseConfig::default(),
            ai: AiConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            provider: "memory".to_string(),
            connection_string: None,
        }
    }
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            gemini_api_key: std::env::var("GEMINI_API_KEY").ok(),
            enabled: false,
        }
    }
}

impl AppConfig {
    /// Load configuration from environment
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
            config.ai.gemini_api_key = Some(api_key);
            config.ai.enabled = true;
        }
        
        config
    }
}
