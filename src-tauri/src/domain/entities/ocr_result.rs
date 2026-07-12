//! OCR Result Entity - Represents the result of OCR processing

use serde::{Deserialize, Serialize};
use crate::domain::entities::DocumentId;

/// Text block detected by OCR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    pub text: String,
    pub confidence: f64,
    pub bounding_box: Option<BoundingBox>,
}

/// Bounding box coordinates for detected text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Result of an OCR operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    pub id: String,
    pub document_id: DocumentId,
    pub extracted_text: String,
    pub blocks: Vec<TextBlock>,
    pub language: String,
    pub processing_time_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

impl OcrResult {
    pub fn new(document_id: DocumentId, extracted_text: String, language: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            document_id,
            extracted_text,
            blocks: Vec::new(),
            language,
            processing_time_ms: 0,
            success: true,
            error_message: None,
        }
    }

    pub fn with_blocks(mut self, blocks: Vec<TextBlock>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn with_processing_time(mut self, time_ms: u64) -> Self {
        self.processing_time_ms = time_ms;
        self
    }

    pub fn error(document_id: DocumentId, error: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            document_id,
            extracted_text: String::new(),
            blocks: Vec::new(),
            language: String::new(),
            processing_time_ms: 0,
            success: false,
            error_message: Some(error),
        }
    }
}
