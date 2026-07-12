//! Tauri Commands - IPC Bridge between React UI and Rust Core
//! Implements the desacoplado pipeline: Fase 1 (Análisis) -> Fase 2 (Persistencia)

use crate::core::{
    error::AppResult,
    gemini_client::GeminiClient,
    ocr::OcrEngine,
    pdf_builder::PdfBuilder,
    scanner::{ScannerConfig, ScannerService},
};
use crate::db::DatabaseService;
use crate::models::{AnalisisOficioResultado, CrearOficioDto, MetadatosExtraidos};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Estado compartido de la aplicación (AppState)
pub struct AppState {
    pub db: Arc<Mutex<Option<DatabaseService>>>,
    pub scanner: Arc<ScannerService>,
    pub ocr: Arc<OcrEngine>,
    pub gemini: Arc<GeminiClient>,
    pub pdf_builder: Arc<PdfBuilder>,
    pub directorio_base: PathBuf,
}

/// Comando: Inicializa la aplicación y verifica conectividad
#[tauri::command]
pub async fn inicializar_app(
    state: tauri::State<'_, Arc<AppState>>,
    api_key: String,
    directorio_base: String,
) -> Result<InicializacionResponse, String> {
    log::info!("Inicializando aplicación...");

    // Actualizar estado con nueva configuración
    let gemini = GeminiClient::new(api_key.clone());
    
    // Verificar conexión a Gemini
    let gemini_disponible = gemini.verificar_conexion().await;
    
    // Verificar escáneres disponibles
    let escaneres = state.scanner.verificar_disponibilidad().await
        .map_err(|e| e.to_string())?;

    log::info!("Inicialización completada. Gemini disponible: {}", gemini_disponible);

    Ok(InicializacionResponse {
        exitoso: true,
        gemini_disponible,
        ocr_disponible: state.ocr.esta_disponible(),
        escaneres_detectados: escaneres,
        mensaje: "Aplicación inicializada correctamente".to_string(),
    })
}

/// Comando FASE 1: Analiza documento completo (Escaneo + OCR + IA)
#[tauri::command]
pub async fn analizar_oficio_escaner(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<AnalisisOficioResultado, String> {
    log::info!("Iniciando pipeline de análisis (Fase 1)...");
    
    let start = std::time::Instant::now();

    // Paso 1: Escanear documento
    log::debug!("Paso 1: Escaneando documento...");
    let imagen = state.scanner.escanear_documento().await
        .map_err(|e| format!("Error en escaneo: {}", e))?;

    // Guardar imagen temporal para PDF
    let temp_path = crate::core::scanner::guardar_imagen_temporal(&imagen)
        .map_err(|e| e.to_string())?;

    // Paso 2: Ejecutar OCR local
    log::debug!("Paso 2: Ejecutando OCR...");
    let resultado_ocr = state.ocr.procesar_imagen(&imagen).await
        .map_err(|e| format!("Error en OCR: {}", e))?;

    // Paso 3: Convertir imagen a bytes para Gemini
    let mut img_bytes = Vec::new();
    use image::ImageOutputFormat;
    imagen.write_to(&mut std::io::Cursor::new(&mut img_bytes), ImageOutputFormat::Png)
        .map_err(|e| format!("Error convirtiendo imagen: {}", e))?;

    // Paso 4: Inferencia con Gemini (Structured Outputs + Zero Retention)
    log::debug!("Paso 3: Ejecutando inferencia con Gemini AI...");
    let metadatos = state.gemini.analizar_documento(&img_bytes, &resultado_ocr.texto_extraido).await
        .map_err(|e| format!("Error en Gemini API: {}", e))?;

    // Paso 5: Construir PDF con capa OCR
    log::debug!("Paso 4: Generando PDF...");
    let pdf_temporal = state.directorio_base.join("temp").join(format!("{}.pdf", uuid::Uuid::new_v4()));
    
    state.pdf_builder.crear_pdf_con_capa_ocr(&imagen, &resultado_ocr.texto_extraido, &pdf_temporal).await
        .map_err(|e| format!("Error creando PDF: {}", e))?;

    let tiempo_total = start.elapsed().as_millis() as u64;

    // Validar RNF-PERF-02: Fase 1 < 5s
    if tiempo_total > 5000 {
        log::warn!("Fase 1 excedió límite de rendimiento: {}ms > 5000ms", tiempo_total);
    }

    log::info!("Fase 1 completada en {}ms", tiempo_total);

    Ok(AnalisisOficioResultado {
        texto_completo: resultado_ocr.texto_extraido,
        metadatos_extraidos: metadatos,
        ruta_pdf_temporal: pdf_temporal.to_string_lossy().to_string(),
        confianza_ocr: resultado_ocr.confianza_promedio,
        tiempo_procesamiento_ms: tiempo_total,
    })
}

/// Comando FASE 2: Guarda oficio procesado en base de datos local
#[tauri::command]
pub async fn guardar_oficio_procesado(
    state: tauri::State<'_, Arc<AppState>>,
    dto: CrearOficioDto,
) -> Result<String, String> {
    log::info!("Iniciando persistencia (Fase 2) para oficio {}...", dto.numero_oficio);

    // Obtener lock de la base de datos
    let db_guard = state.db.lock().await;
    let db = db_guard.as_ref()
        .ok_or_else(|| "Base de datos no inicializada".to_string())?;

    // Mover PDF a ubicación permanente
    let ruta_permanente = state.directorio_base.join("oficios").join(format!("{}.pdf", dto.numero_oficio.replace('/', "_")));
    
    state.pdf_builder.mover_pdf_permanente(PathBuf::from(&dto.ruta_pdf_temporal).as_path(), &ruta_permanente).await
        .map_err(|e| format!("Error moviendo PDF: {}", e))?;

    // Actualizar DTO con ruta permanente
    let dto_actualizado = CrearOficioDto {
        ruta_pdf_temporal: ruta_permanente.to_string_lossy().to_string(),
        ..dto
    };

    // Insertar en base de datos (transaccional)
    let id = db.guardar_oficio(&dto_actualizado).await
        .map_err(|e| format!("Error en base de datos: {}", e))?;

    log::info!("Oficio {} persistido exitosamente con ID: {}", dto_actualizado.numero_oficio, id);
    
    Ok(id)
}

/// Comando: Obtiene todos los oficios registrados
#[tauri::command]
pub async fn obtener_oficios(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<crate::models::Oficio>, String> {
    let db_guard = state.db.lock().await;
    let db = db_guard.as_ref()
        .ok_or_else(|| "Base de datos no inicializada".to_string())?;

    db.obtener_todos_oficios().await
        .map_err(|e| format!("Error obteniendo oficios: {}", e))
}

/// Comando: Busca oficios por palabra clave
#[tauri::command]
pub async fn buscar_oficios_por_palabra(
    state: tauri::State<'_, Arc<AppState>>,
    palabra: String,
) -> Result<Vec<crate::models::Oficio>, String> {
    let db_guard = state.db.lock().await;
    let db = db_guard.as_ref()
        .ok_or_else(|| "Base de datos no inicializada".to_string())?;

    db.buscar_por_palabra_clave(&palabra).await
        .map_err(|e| format!("Error en búsqueda: {}", e))
}

/// Comando: Elimina un oficio
#[tauri::command]
pub async fn eliminar_oficio(
    state: tauri::State<'_, Arc<AppState>>,
    id: String,
) -> Result<(), String> {
    let db_guard = state.db.lock().await;
    let db = db_guard.as_ref()
        .ok_or_else(|| "Base de datos no inicializada".to_string())?;

    db.eliminar_oficio(&id).await
        .map_err(|e| format!("Error eliminando oficio: {}", e))
}

/// Respuesta de inicialización
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InicializacionResponse {
    pub exitoso: bool,
    pub gemini_disponible: bool,
    pub ocr_disponible: bool,
    pub escaneres_detectados: Vec<String>,
    pub mensaje: String,
}
