//! Update Document Command

use serde::{Deserialize, Serialize};

/// Command to update an existing document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentCommand {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl UpdateDocumentCommand {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: None,
            content: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
}
