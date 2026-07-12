//! File System Infrastructure - File operations

use std::path::PathBuf;
use crate::domain::value_objects::FilePath;
use crate::error::DomainError;

/// Service for file system operations
pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        Self
    }

    /// Read file contents as bytes
    pub fn read_file(&self, path: &FilePath) -> Result<Vec<u8>, DomainError> {
        std::fs::read(path.as_path())
            .map_err(|_| DomainError::FileReadError)
    }

    /// Read file contents as string
    pub fn read_file_string(&self, path: &FilePath) -> Result<String, DomainError> {
        std::fs::read_to_string(path.as_path())
            .map_err(|_| DomainError::FileReadError)
    }

    /// Write bytes to file
    pub fn write_file(&self, path: &FilePath, data: &[u8]) -> Result<(), DomainError> {
        std::fs::write(path.as_path(), data)
            .map_err(|_| DomainError::FileWriteError)
    }

    /// Check if file exists
    pub fn exists(&self, path: &FilePath) -> bool {
        path.exists()
    }

    /// Get file extension
    pub fn get_extension(&self, path: &FilePath) -> Option<String> {
        path.extension().map(|s| s.to_string())
    }

    /// Create directory if it doesn't exist
    pub fn create_dir_all(&self, path: &PathBuf) -> Result<(), DomainError> {
        std::fs::create_dir_all(path)
            .map_err(|_| DomainError::FileWriteError)
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}
