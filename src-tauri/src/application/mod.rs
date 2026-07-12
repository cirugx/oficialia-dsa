//! Application Layer - Use Cases (CQRS Pattern)
//! 
//! This layer contains application business logic, commands, queries, and DTOs.
//! It orchestrates the domain objects to execute use cases.

pub mod commands;
pub mod queries;
pub mod dto;
pub mod ports;

// Re-export for convenience
pub use commands::*;
pub use queries::*;
pub use dto::*;
