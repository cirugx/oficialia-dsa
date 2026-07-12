//! Process OCR Command

use serde::{Deserialize, Serialize};

/// Command to process OCR on a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOcrCommand {
    pub document_id: String,
    pub language: String,
}

impl ProcessOcrCommand {
    pub fn new(document_id: String, language: String) -> Self {
        Self {
            document_id,
            language: language.unwrap_or("eng".to_string()),
        }
    }
}
