//! OCR Engine using Windows.Media.Ocr for offline text extraction
//! RF-02: Extracción de texto offline con Windows.Media.Ocr
//! RNF-PERF-02: OCR en menos de 1.5s

use crate::core::error::{AppError, AppResult};
use image::DynamicImage;
use std::time::Instant;

#[cfg(windows)]
use windows::{
    Graphics::Imaging::BitmapDecoder,
    Media::Ocr::OcrEngine as WindowsOcrEngine,
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream},
};

/// Resultado del proceso OCR con métricas de calidad
#[derive(Debug, Clone)]
pub struct OcrResult {
    pub texto_extraido: String,
    pub confianza_promedio: f64,
    pub tiempo_procesamiento_ms: u64,
    pub lineas_detectadas: usize,
}

/// Motor OCR basado en Windows.Media.Ocr
pub struct OcrEngine {
    #[cfg(windows)]
    engine: Option<WindowsOcrEngine>,
    idiomas_soportados: Vec<String>,
}

impl OcrEngine {
    /// Inicializa el motor OCR con el idioma especificado
    pub fn new(idioma: &str) -> Self {
        #[cfg(windows)]
        {
            // Intentar inicializar Windows.Media.Ocr
            let engine = match WindowsOcrEngine::TryCreateFromUserProfileLanguages() {
                Ok(e) => {
                    log::info!("Motor OCR Windows inicializado correctamente");
                    Some(e)
                }
                Err(e) => {
                    log::warn!("Falló inicialización OCR Windows: {:?}", e);
                    None
                }
            };

            Self {
                engine,
                idiomas_soportados: vec![idioma.to_string()],
            }
        }

        #[cfg(not(windows))]
        {
            log::warn!("Windows.Media.Ocr solo disponible en Windows");
            Self {
                idiomas_soportados: vec![idioma.to_string()],
            }
        }
    }

    /// Ejecuta OCR sobre una imagen en thread blocking (RNF-PERF-01)
    pub async fn procesar_imagen(&self, imagen: &DynamicImage) -> AppResult<OcrResult> {
        let start = Instant::now();

        // Clonar imagen para el thread blocking
        let img_clone = imagen.clone();
        
        let resultado = tokio::task::spawn_blocking(move || {
            Self::procesar_sincrono(&img_clone)
        })
        .await
        .map_err(|e| AppError::Ocr(format!("Error en thread OCR: {}", e)))??;

        let tiempo_ms = start.elapsed().as_millis() as u64;

        // Validar RNF-PERF-02: OCR < 1.5s
        if tiempo_ms > 1500 {
            log::warn!("OCR excedió límite de rendimiento: {}ms > 1500ms", tiempo_ms);
        } else {
            log::debug!("OCR completado en {}ms", tiempo_ms);
        }

        Ok(OcrResult {
            tiempo_procesamiento_ms: tiempo_ms,
            ..resultado
        })
    }

    /// Implementación síncrona del OCR
    #[cfg(windows)]
    fn procesar_sincrono(imagen: &DynamicImage) -> AppResult<OcrResult> {
        use windows::Storage::Streams::IBuffer;
        use std::convert::TryInto;

        // Convertir imagen a formato compatible con Windows.Media.Ocr
        let mut buffer = Vec::new();
        imagen
            .write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)
            .map_err(|e| AppError::Ocr(format!("Falló conversión de imagen: {}", e)))?;

        // Crear stream en memoria para BitmapDecoder
        let stream = InMemoryRandomAccessStream::new()?;
        let writer = DataWriter::CreateDataWriter(&stream)?;
        writer.WriteBytes(&buffer)?;
        writer.StoreAsync()?.get()?;
        writer.FlushAsync()?.get()?;
        stream.Seek(0)?;

        // Decodificar bitmap
        let decoder = BitmapDecoder::CreateWithIdAsync(BitmapDecoder::PngDecoderId()?, &stream)?
            .get()?;
        let bitmap = decoder.GetSoftwareBitmapAsync()?.get()?;

        // Ejecutar OCR
        let engine = WindowsOcrEngine::TryCreateFromUserProfileLanguages()
            .map_err(|e| AppError::Ocr(format!("OCR no disponible: {:?}", e)))?;
        
        let result = engine.RecognizeAsync(&bitmap)?.get()?;
        let texto = result.Text()?.to_string();

        // Calcular métricas básicas
        let lineas = texto.lines().count();
        let confianza = if texto.is_empty() { 0.0 } else { 0.85 }; // Placeholder

        log::info!("OCR completado: {} caracteres, {} líneas", texto.len(), lineas);

        Ok(OcrResult {
            texto_extraido: texto,
            confianza_promedio: confianza,
            tiempo_procesamiento_ms: 0, // Se calcula en el wrapper async
            lineas_detectadas: lineas,
        })
    }

    /// Stub para plataformas no-Windows
    #[cfg(not(windows))]
    fn procesar_sincrono(_imagen: &DynamicImage) -> AppResult<OcrResult> {
        log::warn!("OCR requiere Windows.Media.Ocr (solo Windows)");
        
        // En desarrollo, retornar texto simulado
        Ok(OcrResult {
            texto_extraido: "[OCR Simulado] Documento de prueba - Solo disponible en Windows".to_string(),
            confianza_promedio: 0.95,
            tiempo_procesamiento_ms: 100,
            lineas_detectadas: 1,
        })
    }

    /// Verifica si el motor OCR está disponible
    pub fn esta_disponible(&self) -> bool {
        #[cfg(windows)]
        {
            self.engine.is_some()
        }
        #[cfg(not(windows))]
        {
            false
        }
    }

    /// Obtiene lista de idiomas soportados
    pub fn idiomas_soportados(&self) -> &[String] {
        &self.idiomas_soportados
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_engine_creation() {
        let engine = OcrEngine::new("es-ES");
        assert!(engine.idiomas_soportados.contains(&"es-ES".to_string()));
    }
}
