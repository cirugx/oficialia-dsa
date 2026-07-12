//! Create Document Command

use serde::{Deserialize, Serialize};

/// Command to create a new document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentCommand {
    pub title: String,
    pub content: String,
}

impl CreateDocumentCommand {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}

/// Result of creating a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentResult {
    pub id: String,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
