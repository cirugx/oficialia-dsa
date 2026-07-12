//! Core module exposing all internal components
//! Follows clean architecture principles with clear separation of concerns

pub mod error;
pub mod scanner;
pub mod ocr;
pub mod gemini_client;
pub mod pdf_builder;

pub use error::{AppError, AppResult};
