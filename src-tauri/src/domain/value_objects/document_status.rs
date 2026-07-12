//! Document Status Value Object - Represents the state of a document

use serde::{Deserialize, Serialize};

/// The status of a document in its lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    Draft,
    Scanned,
    Processing,
    Processed,
    Archived,
    Deleted,
}

impl DocumentStatus {
    /// Check if the document is in an active state
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            DocumentStatus::Draft
                | DocumentStatus::Scanned
                | DocumentStatus::Processing
                | DocumentStatus::Processed
        )
    }

    /// Check if the document can be edited
    pub fn is_editable(&self) -> bool {
        matches!(self, DocumentStatus::Draft | DocumentStatus::Scanned)
    }

    /// Get all possible statuses
    pub fn all() -> Vec<Self> {
        vec![
            DocumentStatus::Draft,
            DocumentStatus::Scanned,
            DocumentStatus::Processing,
            DocumentStatus::Processed,
            DocumentStatus::Archived,
            DocumentStatus::Deleted,
        ]
    }
}

impl Default for DocumentStatus {
    fn default() -> Self {
        DocumentStatus::Draft
    }
}

impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentStatus::Draft => write!(f, "Draft"),
            DocumentStatus::Scanned => write!(f, "Scanned"),
            DocumentStatus::Processing => write!(f, "Processing"),
            DocumentStatus::Processed => write!(f, "Processed"),
            DocumentStatus::Archived => write!(f, "Archived"),
            DocumentStatus::Deleted => write!(f, "Deleted"),
        }
    }
}
