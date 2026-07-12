//! List Documents Query

use serde::{Deserialize, Serialize};

/// Query to list documents with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentsQuery {
    pub limit: u32,
    pub offset: u32,
}

impl Default for ListDocumentsQuery {
    fn default() -> Self {
        Self {
            limit: 10,
            offset: 0,
        }
    }
}

impl ListDocumentsQuery {
    pub fn new(limit: u32, offset: u32) -> Self {
        Self { limit, offset }
    }
}
