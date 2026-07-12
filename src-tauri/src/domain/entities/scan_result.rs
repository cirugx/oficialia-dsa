//! Scan Result Entity - Represents the result of a document scan

use serde::{Deserialize, Serialize};
use crate::domain::entities::DocumentId;

/// Metadata about a scanned document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanMetadata {
    pub file_name: String,
    pub file_size: u64,
    pub mime_type: String,
    pub scan_date: chrono::DateTime<chrono::Utc>,
}

/// Result of a document scanning operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub id: String,
    pub document_id: DocumentId,
    pub metadata: ScanMetadata,
    pub success: bool,
    pub error_message: Option<String>,
}

impl ScanResult {
    pub fn new(document_id: DocumentId, metadata: ScanMetadata) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            document_id,
            metadata,
            success: true,
            error_message: None,
        }
    }

    pub fn error(document_id: DocumentId, error: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            document_id,
            metadata: ScanMetadata {
                file_name: String::new(),
                file_size: 0,
                mime_type: String::new(),
                scan_date: now,
            },
            success: false,
            error_message: Some(error),
        }
    }
}
