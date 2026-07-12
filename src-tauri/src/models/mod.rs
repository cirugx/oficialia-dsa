//! Core domain models for the Document Management System
//! Implements strict typing for official documents (Oficios) with metadata extraction

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Clasificación documental según tipología oficial
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Clasificacion {
    Entrante,
    Saliente,
    Interno,
    Confidencial,
    Urgente,
}

impl std::fmt::Display for Clasificacion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Clasificacion::Entrante => write!(f, "ENTRANTE"),
            Clasificacion::Saliente => write!(f, "SALIENTE"),
            Clasificacion::Interno => write!(f, "INTERNO"),
            Clasificacion::Confidencial => write!(f, "CONFIDENCIAL"),
            Clasificacion::Urgente => write!(f, "URGENTE"),
        }
    }
}

/// Representación del modelo de oficio para persistencia en SQLite
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Oficio {
    pub id: String,
    pub numero_oficio: String,
    pub remitente: String,
    pub asunto: String,
    pub clasificacion: String,
    pub fecha_oficio: String,
    pub fecha_registro: DateTime<Utc>,
    pub ruta_pdf_local: String,
}

/// DTO para la creación de nuevos oficios (fase 2 del pipeline)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrearOficioDto {
    pub numero_oficio: String,
    pub remitente: String,
    pub asunto: String,
    pub clasificacion: Clasificacion,
    pub fecha_oficio: String,
    pub ruta_pdf_temporal: String,
    pub palabras_clave: Vec<String>,
}

/// Resultado del análisis OCR + IA (Fase 1 del pipeline)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalisisOficioResultado {
    pub texto_completo: String,
    pub metadatos_extraidos: MetadatosExtraidos,
    pub ruta_pdf_temporal: String,
    pub confianza_ocr: f64,
    pub tiempo_procesamiento_ms: u64,
}

/// Metadatos extraídos por Gemini con Structured Outputs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadatosExtraidos {
    pub numero_oficio: Option<String>,
    pub remitente: Option<String>,
    pub asunto: Option<String>,
    pub clasificacion_sugerida: Option<Clasificacion>,
    pub fecha_oficio: Option<String>,
    pub palabras_clave: Vec<String>,
    pub resumen_ejecutivo: String,
}

/// Palabra clave indexable para búsqueda full-text
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PalabraClave {
    pub id: i64,
    pub oficio_id: String,
    pub palabra: String,
}

/// Estado del pipeline de procesamiento
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EstadoPipeline {
    PendienteAnalisis,
    EnAnalisis,
    PendienteAuditoria,
    EnAuditoria,
    ListoParaPersistencia,
    Completado,
    Error(String),
}

/// Configuración del sistema cargada desde entorno
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuracion {
    pub gemini_api_key: String,
    pub directorio_base: String,
    pub dpi_escaneo: u32,
    pub timeout_http_ms: u64,
    pub ruta_db_sqlite: String,
}

impl Default for Configuracion {
    fn default() -> Self {
        Self {
            gemini_api_key: std::env::var("GEMINI_API_KEY").unwrap_or_default(),
            directorio_base: std::env::var("APP_DATA_DIR").unwrap_or_else(|_| "./data".to_string()),
            dpi_escaneo: 300,
            timeout_http_ms: 30000,
            ruta_db_sqlite: "./data/oficios.db".to_string(),
        }
    }
}
