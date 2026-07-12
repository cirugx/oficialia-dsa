//! Database Infrastructure - Persistence implementations

use async_trait::async_trait;
use crate::domain::entities::{Document, DocumentId};
use crate::domain::repositories::DocumentRepository;
use crate::error::DomainError;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// In-memory document repository (for development/demo)
pub struct InMemoryDocumentRepository {
    documents: RwLock<HashMap<String, Document>>,
}

impl InMemoryDocumentRepository {
    pub fn new() -> Self {
        Self {
            documents: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryDocumentRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DocumentRepository for InMemoryDocumentRepository {
    async fn save(&self, document: &Document) -> Result<(), DomainError> {
        let mut docs = self.documents.write().await;
        docs.insert(document.id.0.clone(), document.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &DocumentId) -> Result<Option<Document>, DomainError> {
        let docs = self.documents.read().await;
        Ok(docs.get(&id.0).cloned())
    }

    async fn find_all(&self, limit: u32, offset: u32) -> Result<Vec<Document>, DomainError> {
        let docs = self.documents.read().await;
        let mut all_docs: Vec<Document> = docs.values().cloned().collect();
        
        // Sort by created_at descending
        all_docs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Apply pagination
        let start = offset as usize;
        let end = (offset + limit) as usize;
        
        if start >= all_docs.len() {
            return Ok(Vec::new());
        }
        
        Ok(all_docs[start..end.min(all_docs.len())].to_vec())
    }

    async fn delete(&self, id: &DocumentId) -> Result<(), DomainError> {
        let mut docs = self.documents.write().await;
        docs.remove(&id.0);
        Ok(())
    }

    async fn find_by_title(&self, title: &str) -> Result<Vec<Document>, DomainError> {
        let docs = self.documents.read().await;
        let results = docs
            .values()
            .filter(|d| d.title.to_lowercase().contains(&title.to_lowercase()))
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_by_tag(&self, tag: &str) -> Result<Vec<Document>, DomainError> {
        let docs = self.documents.read().await;
        let results = docs
            .values()
            .filter(|d| d.tags.iter().any(|t| t.to_lowercase() == tag.to_lowercase()))
            .cloned()
            .collect();
        Ok(results)
    }
}
