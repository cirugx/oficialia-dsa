//! Document Domain Service - Business logic for document operations

use crate::domain::entities::{Document, DocumentId};
use crate::domain::value_objects::FilePath;
use crate::domain::repositories::DocumentRepository;
use crate::error::DomainError;

/// Service for document-related business logic
pub struct DocumentService<R: DocumentRepository> {
    repository: R,
}

impl<R: DocumentRepository> DocumentService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Create a new document
    pub async fn create_document(
        &self,
        title: String,
        content: String,
    ) -> Result<Document, DomainError> {
        let document = Document::new(title, content);
        self.repository.save(&document).await?;
        Ok(document)
    }

    /// Get a document by ID
    pub async fn get_document(&self, id: &DocumentId) -> Result<Option<Document>, DomainError> {
        self.repository.find_by_id(id).await
    }

    /// Update document content
    pub async fn update_content(
        &self,
        id: &DocumentId,
        content: String,
    ) -> Result<Document, DomainError> {
        let mut document = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(DomainError::DocumentNotFound)?;

        document.update_content(content);
        self.repository.save(&document).await?;
        Ok(document)
    }

    /// Set document file path
    pub async fn set_file_path(
        &self,
        id: &DocumentId,
        path: FilePath,
    ) -> Result<Document, DomainError> {
        let mut document = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(DomainError::DocumentNotFound)?;

        document = document.with_path(path.as_str().unwrap_or("").to_string());
        self.repository.save(&document).await?;
        Ok(document)
    }

    /// Delete a document
    pub async fn delete_document(&self, id: &DocumentId) -> Result<(), DomainError> {
        self.repository.delete(id).await
    }

    /// List all documents with pagination
    pub async fn list_documents(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Document>, DomainError> {
        self.repository.find_all(limit, offset).await
    }
}
