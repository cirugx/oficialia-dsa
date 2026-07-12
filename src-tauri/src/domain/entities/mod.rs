//! Domain Entities - Core business objects
//! 
//! These entities represent the core business concepts of the application.
//! They are independent of any framework or external concern.

pub mod document;
pub mod scan_result;
pub mod ocr_result;

pub use document::Document;
pub use scan_result::ScanResult;
pub use ocr_result::OcrResult;
