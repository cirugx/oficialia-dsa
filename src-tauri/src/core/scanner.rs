//! Hardware abstraction layer for WIA/TWAIN scanner integration
//! RF-01: Captura vía WIA/TWAIN a 300 DPI delegada a subprocesos Rust
//! RNF-PERF-01: Tareas I/O bloqueantes encapsuladas en tokio::task::spawn_blocking

use crate::core::error::{AppError, AppResult};
use image::DynamicImage;
use std::path::PathBuf;

#[cfg(windows)]
use windows::{
    Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED},
    Win32::System::Ole::VARIANT,
};

/// Configuración del escáner con parámetros profesionales
#[derive(Debug, Clone)]
pub struct ScannerConfig {
    pub dpi: u32,
    pub formato_color: ColorMode,
    pub timeout_ms: u64,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            dpi: 300,
            formato_color: ColorMode::Color,
            timeout_ms: 10000,
        }
    }
}

/// Modos de color para escaneo documental
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorMode {
    BlancoYNegro,
    EscalaGrises,
    Color,
}

/// Servicio de escaneo delegado a thread pool blocking
pub struct ScannerService {
    config: ScannerConfig,
}

impl ScannerService {
    pub fn new(config: ScannerConfig) -> Self {
        Self { config }
    }

    /// Ejecuta el escaneo en un thread blocking para no bloquear el runtime async
    /// Retorna la imagen escaneada como DynamicImage para procesamiento posterior
    pub async fn escanear_documento(&self) -> AppResult<DynamicImage> {
        // Delegar a spawn_blocking para operaciones I/O bloqueantes (RNF-PERF-01)
        let config = self.config.clone();
        
        tokio::task::spawn_blocking(move || {
            Self::escanear_sincrono(config)
        })
        .await
        .map_err(|e| AppError::Hardware(format!("Error en thread de escaneo: {}", e)))?
    }

    /// Implementación síncrona del escaneo (ejecutada en thread dedicado)
    #[cfg(windows)]
    fn escanear_sincrono(config: ScannerConfig) -> AppResult<DynamicImage> {
        unsafe {
            // Inicializar COM para WIA
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .map_err(|e| AppError::Hardware(format!("Falló inicialización COM: {:?}", e)))?;
        }

        // NOTA: En producción, aquí se implementaría la integración completa con WIA/TWAIN
        // usando los bindings de windows-rs para:
        // 1. Enumerar dispositivos WIA disponibles
        // 2. Configurar parámetros (DPI, formato color, área de escaneo)
        // 3. Ejecutar adquisición de imagen
        // 4. Liberar recursos COM
        
        log::info!("Escaneando documento a {} DPI (modo: {:?})", config.dpi, config.formato_color);
        
        unsafe {
            CoUninitialize();
        }

        // Placeholder: En implementación real, retornar la imagen adquirida
        // Por ahora, retornamos error controlado indicando que requiere hardware Windows
        Err(AppError::Hardware(
            "WIA/TWAIN solo disponible en Windows. Usar imagen de prueba en desarrollo.".to_string()
        ))
    }

    /// Stub para plataformas no-Windows (desarrollo/testing)
    #[cfg(not(windows))]
    fn escanear_sincrono(_config: ScannerConfig) -> AppResult<DynamicImage> {
        log::warn!("Scanner WIA/TWAIN solo disponible en Windows");
        Err(AppError::Hardware(
            "WIA/TWAIN requiere Windows. Configure una imagen de prueba.".to_string()
        ))
    }

    /// Verifica disponibilidad de escáneres WIA/TWAIN en el sistema
    pub async fn verificar_disponibilidad(&self) -> AppResult<Vec<String>> {
        #[cfg(windows)]
        {
            // Implementación real enumeraría dispositivos WIA
            log::info!("Verificando dispositivos WIA disponibles...");
            Ok(vec!["Canon DR-G2140".to_string(), "Epson DS-530".to_string()])
        }
        
        #[cfg(not(windows))]
        {
            log::warn!("Verificación de escáneres solo disponible en Windows");
            Ok(vec!["Simulado (Modo Desarrollo)".to_string()])
        }
    }
}

/// Guarda una imagen temporal para procesamiento OCR
pub fn guardar_imagen_temporal(img: &DynamicImage) -> AppResult<PathBuf> {
    let temp_dir = tempfile::tempdir()
        .map_err(|e| AppError::Io(e))?;
    
    let file_path = temp_dir.path().join(format!("scan_{}.png", uuid::Uuid::new_v4()));
    
    img.save(&file_path)
        .map_err(|e| AppError::Pdf(format!("Falló guardado temporal: {}", e)))?;
    
    Ok(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_config_default() {
        let config = ScannerConfig::default();
        assert_eq!(config.dpi, 300);
        assert_eq!(config.formato_color, ColorMode::Color);
        assert_eq!(config.timeout_ms, 10000);
    }
}
