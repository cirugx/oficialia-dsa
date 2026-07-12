//! Scan Document Command

use serde::{Deserialize, Serialize};

/// Command to scan a document file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDocumentCommand {
    pub file_path: String,
}

impl ScanDocumentCommand {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}
