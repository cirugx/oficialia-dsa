//! Data Transfer Objects (DTOs) - Application Layer

use serde::{Deserialize, Serialize};

/// Document DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDto {
    pub id: String,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
}

impl From<crate::domain::entities::Document> for DocumentDto {
    fn from(doc: crate::domain::entities::Document) -> Self {
        Self {
            id: doc.id.0,
            title: doc.title,
            content: doc.content,
            file_path: doc.file_path,
            created_at: doc.created_at,
            updated_at: doc.updated_at,
            tags: doc.tags,
        }
    }
}

/// Scan Result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResultDto {
    pub id: String,
    pub document_id: String,
    pub file_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub success: bool,
    pub error_message: Option<String>,
}

/// OCR Result DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResultDto {
    pub id: String,
    pub document_id: String,
    pub extracted_text: String,
    pub language: String,
    pub processing_time_ms: u64,
    pub success: bool,
}
