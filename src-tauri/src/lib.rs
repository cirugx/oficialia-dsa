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
//! 
//! # Stack Tecnológico Unificado (STU)
//! 
//! | Capa | Tecnología | Función Principal |
//! | --- | --- | --- |
//! | Frontend UI | React + TypeScript + Vite | Interfaz reactiva y tipado estricto |
//! | Diseño y Estados | Shadcn UI + TanStack Query | Estilizado accesible y gestión asíncrona de caché |
//! | Contenedor Desktop | Tauri v2 + WebView2 | Puente IPC nativo ligero |
//! | Backend Core | Rust (Edición 2021) + Tokio | Concurrencia segura y gestión de memoria extrema |
//! | Hardware & OCR | windows-rs | Integración nativa con WIA/TWAIN y Windows Media OCR |
//! | Motor PDF | lopdf + image | Ensamble binario de PDF con inyección de texto invisible |
//! | Red y API | reqwest + serde | Cliente HTTP asíncrono para interactuar con Gemini |
//! | Persistencia | SQLite + sqlx | Base relacional con validación SQL en tiempo de compilación |

// Core modules (new architecture)
pub mod core;
pub mod models;
pub mod db;
pub mod commands;

// Legacy modules (to be refactored)
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod error;
pub mod config;
pub mod database;
pub mod gemini;

use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Estado compartido de la aplicación (AppState)
pub struct AppState {
    pub db: Arc<Mutex<Option<db::DatabaseService>>>,
    pub scanner: Arc<core::scanner::ScannerService>,
    pub ocr: Arc<core::ocr::OcrEngine>,
    pub gemini: Arc<core::gemini_client::GeminiClient>,
    pub pdf_builder: Arc<core::pdf_builder::PdfBuilder>,
    pub directorio_base: std::path::PathBuf,
}

impl AppState {
    /// Crea una nueva instancia de AppState
    pub fn new() -> Self {
        use std::path::PathBuf;
        
        let directorio_base = PathBuf::from("./data");
        
        Self {
            db: Arc::new(Mutex::new(None)),
            scanner: Arc::new(core::scanner::ScannerService::new(
                core::scanner::ScannerConfig::default()
            )),
            ocr: Arc::new(core::ocr::OcrEngine::new("es-ES")),
            gemini: Arc::new(core::gemini_client::GeminiClient::new(String::new())),
            pdf_builder: Arc::new(core::pdf_builder::PdfBuilder::new(300)),
            directorio_base,
        }
    }
    
    /// Inicializa la base de datos y servicios con configuración
    pub async fn inicializar(&self, api_key: String, directorio_base: String) -> Result<(), String> {
        use std::path::PathBuf;
        
        let db_path = PathBuf::from(&directorio_base).join("oficios.db");
        
        // Inicializar base de datos
        let db = db::DatabaseService::conectar(&db_path).await
            .map_err(|e| format!("Error inicializando DB: {}", e))?;
        
        *self.db.lock().await = Some(db);
        
        // Actualizar Gemini client con API key
        let gemini = core::gemini_client::GeminiClient::new(api_key);
        // Nota: En producción, usar interior mutability o reconstruir AppState
        
        log::info!("AppState inicializado correctamente");
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    let app_state = Arc::new(AppState::new());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::inicializar_app,
            commands::analizar_oficio_escaner,
            commands::guardar_oficio_procesado,
            commands::obtener_oficios,
            commands::buscar_oficios_por_palabra,
            commands::eliminar_oficio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
