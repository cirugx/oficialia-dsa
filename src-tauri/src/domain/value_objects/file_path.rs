//! File Path Value Object - Ensures valid file paths

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A validated file path value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FilePath(PathBuf);

impl FilePath {
    pub fn new(path: &str) -> Result<Self, FilePathError> {
        if path.is_empty() {
            return Err(FilePathError::EmptyPath);
        }
        
        let path_buf = PathBuf::from(path);
        
        // Validate path doesn't contain invalid characters
        if let Some(path_str) = path_buf.to_str() {
            if path_str.contains('\0') {
                return Err(FilePathError::InvalidCharacters);
            }
        }
        
        Ok(Self(path_buf))
    }

    pub fn from_path_buf(path: PathBuf) -> Result<Self, FilePathError> {
        Self::new(path.to_str().unwrap_or(""))
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }

    pub fn as_str(&self) -> Option<&str> {
        self.0.to_str()
    }

    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    pub fn extension(&self) -> Option<&str> {
        self.0.extension()?.to_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilePathError {
    EmptyPath,
    InvalidCharacters,
    InvalidPath,
}

impl std::fmt::Display for FilePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilePathError::EmptyPath => write!(f, "File path cannot be empty"),
            FilePathError::InvalidCharacters => write!(f, "File path contains invalid characters"),
            FilePathError::InvalidPath => write!(f, "Invalid file path"),
        }
    }
}

impl std::error::Error for FilePathError {}
