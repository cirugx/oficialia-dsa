//! Delete Document Command

use serde::{Deserialize, Serialize};

/// Command to delete a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentCommand {
    pub id: String,
}

impl DeleteDocumentCommand {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
