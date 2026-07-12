//! Search Documents Query

use serde::{Deserialize, Serialize};

/// Query to search documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocumentsQuery {
    pub query: String,
    pub limit: u32,
}

impl Default for SearchDocumentsQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 20,
        }
    }
}

impl SearchDocumentsQuery {
    pub fn new(query: String, limit: u32) -> Self {
        Self { query, limit }
    }
}
