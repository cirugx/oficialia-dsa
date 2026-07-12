//! Document Entity - Represents a document in the system

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a document
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub String);

impl DocumentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_str(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        Self::new()
    }
}

/// Core document entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub content: String,
    pub file_path: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tags: Vec<String>,
}

impl Document {
    pub fn new(title: String, content: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: DocumentId::new(),
            title,
            content,
            file_path: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
        }
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.file_path = Some(path);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = chrono::Utc::now();
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = chrono::Utc::now();
    }
}
