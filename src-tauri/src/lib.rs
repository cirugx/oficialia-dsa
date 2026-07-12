//! Oficialia DSA - Document Processing Application
//! 
//! A Tauri-based desktop application for document scanning, OCR, and AI-powered analysis.
//! 
//! # Architecture
//! 
//! This application follows Clean Architecture principles:
//! 
//! - **Domain Layer**: Core business entities, value objects, and services
//! - **Application Layer**: Use cases, commands, queries (CQRS pattern)
//! - **Infrastructure Layer**: Database, file system, API clients, Tauri handlers
//! - **Interface Layer**: Tauri commands, frontend components

// Domain layer
pub mod domain;

// Application layer
pub mod application;

// Infrastructure layer
pub mod infrastructure;

// Error handling
pub mod error;

// Configuration
pub mod config;

// Legacy modules (to be removed in future refactoring)
pub mod commands;
pub mod database;
pub mod scanner;
pub mod ocr;
pub mod gemini;

use tauri::Manager;

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = infrastructure::tauri_handlers::AppState::new();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(infrastructure::tauri_handlers::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
