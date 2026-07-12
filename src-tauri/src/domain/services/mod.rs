//! Domain Services - Business logic that doesn't belong to a single entity

pub mod document_service;
pub mod scanner_service;
pub mod ocr_service;
pub mod ai_service;

pub use document_service::DocumentService;
pub use scanner_service::ScannerService;
pub use ocr_service::OcrService;
pub use ai_service::AiService;
