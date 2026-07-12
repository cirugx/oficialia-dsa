//! Get Document Query

use serde::{Deserialize, Serialize};

/// Query to get a single document by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentQuery {
    pub id: String,
}

impl GetDocumentQuery {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
