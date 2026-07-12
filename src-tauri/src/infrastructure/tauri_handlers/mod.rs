//! Tauri Command Handlers - Bridge between Tauri and Application Layer

use tauri::State;
use crate::application::{CreateDocumentCommand, GetDocumentQuery, ListDocumentsQuery};
use crate::domain::services::DocumentService;
use crate::infrastructure::database::InMemoryDocumentRepository;

/// Application state shared across Tauri commands
pub struct AppState {
    pub document_service: DocumentService<InMemoryDocumentRepository>,
}

impl AppState {
    pub fn new() -> Self {
        let repository = InMemoryDocumentRepository::new();
        let document_service = DocumentService::new(repository);
        
        Self { document_service }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Tauri command: Create a new document
#[tauri::command]
pub async fn create_document(
    title: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let command = CreateDocumentCommand::new(title, content);
    
    // In a full implementation, we'd have a command handler that processes this
    // For now, we'll use the service directly
    match state.document_service.create_document(command.title, command.content).await {
        Ok(doc) => Ok(doc.id.0),
        Err(e) => Err(e.to_string()),
    }
}

/// Tauri command: Get a document by ID
#[tauri::command]
pub async fn get_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<crate::application::dto::DocumentDto>, String> {
    use crate::domain::entities::DocumentId;
    
    let doc_id = DocumentId::from_str(&id);
    
    match state.document_service.get_document(&doc_id).await {
        Ok(Some(doc)) => Ok(Some(doc.into())),
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Tauri command: List documents
#[tauri::command]
pub async fn list_documents(
    limit: Option<u32>,
    offset: Option<u32>,
    state: State<'_, AppState>,
) -> Result<Vec<crate::application::dto::DocumentDto>, String> {
    let query = ListDocumentsQuery::new(limit.unwrap_or(10), offset.unwrap_or(0));
    
    match state.document_service.list_documents(query.limit, query.offset).await {
        Ok(docs) => Ok(docs.into_iter().map(|d| d.into()).collect()),
        Err(e) => Err(e.to_string()),
    }
}

/// Tauri command: Delete a document
#[tauri::command]
pub async fn delete_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    use crate::domain::entities::DocumentId;
    
    let doc_id = DocumentId::from_str(&id);
    
    state.document_service.delete_document(&doc_id).await.map_err(|e| e.to_string())
}

/// Tauri command: Greet (legacy compatibility)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Export all commands for use in main.rs
pub fn get_handlers() -> impl Fn(tauri::invoke::Invoke<tauri::Wry>) + Send + Sync + 'static {
    tauri::generate_handler![
        greet,
        create_document,
        get_document,
        list_documents,
        delete_document
    ]
}
