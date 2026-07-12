//! Error handling module with professional error types
//! Uses thiserror for derive macros and anyhow for context

use thiserror::Error;

/// Error principal del sistema con tipado estricto
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de hardware: {0}")]
    Hardware(String),

    #[error("Error de OCR: {0}")]
    Ocr(String),

    #[error("Error de procesamiento PDF: {0}")]
    Pdf(String),

    #[error("Error de comunicación con Gemini API: {0}")]
    GeminiApi(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Error de E/S: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de serialización JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error HTTP: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Error de validación: {0}")]
    Validacion(String),

    #[error("Error interno: {0}")]
    Interno(String),
}

/// Result type alias para consistencia en toda la aplicación
pub type AppResult<T> = Result<T, AppError>;

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Interno(msg)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::Interno(msg.to_string())
    }
}
