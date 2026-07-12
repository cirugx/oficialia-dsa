//! Domain Module - Core business logic
//! 
//! This module contains the domain layer of the Clean Architecture.
//! It includes entities, value objects, services, and repository traits.

pub mod entities;
pub mod value_objects;
pub mod services;
pub mod repositories;

// Re-export main types for convenience
pub use entities::*;
pub use value_objects::*;
pub use services::*;
pub use repositories::*;
