//! Domain Repositories - Traits for data persistence (ports)

use crate::domain::entities::{Document, DocumentId};
use crate::error::DomainError;

/// Repository trait for document persistence
#[async_trait::async_trait]
pub trait DocumentRepository: Send + Sync {
    /// Save a document
    async fn save(&self, document: &Document) -> Result<(), DomainError>;

    /// Find a document by ID
    async fn find_by_id(&self, id: &DocumentId) -> Result<Option<Document>, DomainError>;

    /// Find all documents with pagination
    async fn find_all(&self, limit: u32, offset: u32) -> Result<Vec<Document>, DomainError>;

    /// Delete a document
    async fn delete(&self, id: &DocumentId) -> Result<(), DomainError>;

    /// Find documents by title (search)
    async fn find_by_title(&self, title: &str) -> Result<Vec<Document>, DomainError>;

    /// Find documents by tag
    async fn find_by_tag(&self, tag: &str) -> Result<Vec<Document>, DomainError>;
}
