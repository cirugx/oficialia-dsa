//! Application Commands - Write operations (CQRS)

pub mod create_document;
pub mod update_document;
pub mod delete_document;
pub mod scan_document;
pub mod process_ocr;

pub use create_document::CreateDocumentCommand;
pub use update_document::UpdateDocumentCommand;
pub use delete_document::DeleteDocumentCommand;
pub use scan_document::ScanDocumentCommand;
pub use process_ocr::ProcessOcrCommand;
