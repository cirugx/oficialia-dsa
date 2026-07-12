//! Infrastructure Layer - External concerns implementation
//! 
//! This layer contains implementations for:
//! - Database access
//! - File system operations
//! - External API clients
//! - Tauri command handlers

pub mod database;
pub mod file_system;
pub mod api_clients;
pub mod tauri_handlers;
pub mod error;

// Re-export for convenience
pub use error::InfrastructureError;
