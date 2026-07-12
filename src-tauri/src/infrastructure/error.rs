//! Infrastructure Error Types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("File system error: {0}")]
    FileSystem(String),
    
    #[error("API client error: {0}")]
    ApiClient(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
}

impl From<serde_json::Error> for InfrastructureError {
    fn from(err: serde_json::Error) -> Self {
        InfrastructureError::Serialization(err.to_string())
    }
}
