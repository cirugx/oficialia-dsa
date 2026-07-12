//! Scanner Domain Service - Business logic for document scanning

use crate::domain::entities::{Document, DocumentId, ScanResult, ScanMetadata};
use crate::domain::value_objects::FilePath;
use crate::error::DomainError;

/// Service for scanner-related business logic
pub struct ScannerService;

impl ScannerService {
    pub fn new() -> Self {
        Self
    }

    /// Process a file scan and return metadata
    pub async fn scan_file(&self, path: &FilePath) -> Result<ScanResult, DomainError> {
        if !path.exists() {
            return Err(DomainError::FileNotFound);
        }

        let file_name = path
            .as_str()
            .and_then(|p| p.split('/').last().or_else(|| p.split('\\').last()))
            .unwrap_or("unknown")
            .to_string();

        // Get file metadata
        let metadata = std::fs::metadata(path.as_path())
            .map_err(|_| DomainError::FileReadError)?;

        let scan_metadata = ScanMetadata {
            file_name,
            file_size: metadata.len(),
            mime_type: self.detect_mime_type(path),
            scan_date: chrono::Utc::now(),
        };

        let document_id = DocumentId::new();
        Ok(ScanResult::new(document_id, scan_metadata))
    }

    /// Detect MIME type based on file extension
    fn detect_mime_type(&self, path: &FilePath) -> String {
        match path.extension() {
            Some("pdf") => "application/pdf".to_string(),
            Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
            Some("png") => "image/png".to_string(),
            Some("gif") => "image/gif".to_string(),
            Some("tiff") | Some("tif") => "image/tiff".to_string(),
            Some("bmp") => "image/bmp".to_string(),
            Some("txt") => "text/plain".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }
}

impl Default for ScannerService {
    fn default() -> Self {
        Self::new()
    }
}
