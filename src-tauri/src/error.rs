//! Error Handling - Unified error types for the application

use thiserror::Error;

/// Domain-level errors
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Document not found")]
    DocumentNotFound,
    
    #[error("Document is not editable in its current state")]
    DocumentNotEditable,
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Failed to read file")]
    FileReadError,
    
    #[error("Failed to write file")]
    FileWriteError,
    
    #[error("OCR processing failed: {0}")]
    OcrError(String),
    
    #[error("AI service not configured")]
    AiServiceNotConfigured,
    
    #[error("AI API error: {0}")]
    AiApiError(String),
    
    #[error("Repository error: {0}")]
    RepositoryError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl From<crate::infrastructure::error::InfrastructureError> for DomainError {
    fn from(err: crate::infrastructure::error::InfrastructureError) -> Self {
        match err {
            crate::infrastructure::error::InfrastructureError::Database(msg) => {
                DomainError::RepositoryError(msg)
            }
            crate::infrastructure::error::InfrastructureError::FileSystem(msg) => {
                DomainError::FileReadError // Simplified mapping
            }
            _ => DomainError::RepositoryError(err.to_string()),
        }
    }
}
