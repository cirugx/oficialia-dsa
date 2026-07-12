//! Application Queries - Read operations (CQRS)

pub mod get_document;
pub mod list_documents;
pub mod search_documents;

pub use get_document::GetDocumentQuery;
pub use list_documents::ListDocumentsQuery;
pub use search_documents::SearchDocumentsQuery;
