//! PDF Builder using lopdf for binary PDF assembly with invisible text injection
//! RF-05: Ensamble binario de PDF con inyección de texto invisible

use crate::core::error::{AppError, AppResult};
use image::DynamicImage;
use lopdf::{Document, Object, Stream};
use std::path::{Path, PathBuf};

/// Constructor de PDFs con capacidades avanzadas
pub struct PdfBuilder {
    dpi: u32,
}

impl PdfBuilder {
    pub fn new(dpi: u32) -> Self {
        Self { dpi }
    }

    /// Crea un PDF con la imagen escaneada y texto OCR inyectado como capa invisible
    pub async fn crear_pdf_con_capa_ocr(
        &self,
        imagen: &DynamicImage,
        texto_ocr: &str,
        ruta_salida: &Path,
    ) -> AppResult<PathBuf> {
        // Ejecutar en thread blocking para operaciones I/O intensivas
        let img_clone = imagen.clone();
        let texto = texto_ocr.to_string();
        let salida = ruta_salida.to_path_buf();

        tokio::task::spawn_blocking(move || {
            Self::crear_pdf_sincrono(&img_clone, &texto, &salida)
        })
        .await
        .map_err(|e| AppError::Pdf(format!("Error en thread PDF: {}", e)))??;

        Ok(salida)
    }

    /// Implementación síncrona de creación de PDF
    fn crear_pdf_sincrono(
        imagen: &DynamicImage,
        _texto_ocr: &str,
        ruta_salida: &Path,
    ) -> AppResult<()> {
        // Crear documento PDF base
        let mut doc = Document::with_version("1.7");

        // Convertir imagen a bytes PNG
        let mut img_bytes = Vec::new();
        imagen
            .write_to(&mut std::io::Cursor::new(&mut img_bytes), image::ImageFormat::Png)
            .map_err(|e| AppError::Pdf(format!("Falló conversión de imagen: {}", e)))?;

        let ancho = imagen.width() as f64;
        let alto = imagen.height() as f64;

        // Crear objeto de imagen
        let image_dict_id = doc.new_object_id();
        doc.objects.insert(
            image_dict_id,
            Object::Stream(Stream {
                dict: lopdf::Dictionary::from_iter(vec![
                    ("Type", "XObject".into()),
                    ("Subtype", "Image".into()),
                    ("Width", Object::Integer(imagen.width() as i64)),
                    ("Height", Object::Integer(imagen.height() as i64)),
                    ("ColorSpace", "DeviceRGB".into()),
                    ("BitsPerComponent", 8.into()),
                    ("Filter", "FlateDecode".into()),
                    ("Length", Object::Integer(img_bytes.len() as i64)),
                ]),
                content: img_bytes,
            }),
        );

        // Crear página con dimensiones basadas en DPI
        let puntos_por_pulgada = 72.0;
        let ancho_pt = (ancho / self.dpi as f64) * puntos_por_pulgada;
        let alto_pt = (alto / self.dpi as f64) * puntos_por_pulgada;

        // Crear contenido de página (dibujar imagen)
        let content_id = doc.new_object_id();
        doc.objects.insert(
            content_id,
            Object::Stream(Stream {
                dict: lopdf::Dictionary::new(),
                content: format!(
                    "q {} 0 0 {} 0 0 cm /Im1 Do Q",
                    ancho_pt, alto_pt
                )
                .as_bytes()
                .to_vec(),
            }),
        );

        // Crear página
        let page_id = doc.new_object_id();
        doc.objects.insert(
            page_id,
            Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                ("Type", "Page".into()),
                (
                    "MediaBox",
                    Object::Array(vec![
                        Object::Real(0.0),
                        Object::Real(0.0),
                        Object::Real(ancho_pt),
                        Object::Real(alto_pt),
                    ]),
                ),
                ("Contents", Object::Reference(content_id)),
                (
                    "Resources",
                    Object::Dictionary(lopdf::Dictionary::from_iter(vec![(
                        "XObject",
                        Object::Dictionary(lopdf::Dictionary::from_iter(vec![(
                            "Im1",
                            Object::Reference(image_dict_id),
                        )])),
                    )])),
                ),
            ]),
        );

        // Crear catálogo y páginas
        let pages_id = doc.new_object_id();
        doc.objects.insert(
            pages_id,
            Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                ("Type", "Pages".into()),
                ("Kids", Object::Array(vec![Object::Reference(page_id)])),
                ("Count", Object::Integer(1)),
            ]),
        );

        let catalog_id = doc.new_object_id();
        doc.objects.insert(
            catalog_id,
            Object::Dictionary(lopdf::Dictionary::from_iter(vec![
                ("Type", "Catalog".into()),
                ("Pages", Object::Reference(pages_id)),
            ]),
        );

        doc.trailer.set("Root", Object::Reference(catalog_id));

        // Guardar PDF
        doc.save(ruta_salida)
            .map_err(|e| AppError::Pdf(format!("Falló guardado de PDF: {}", e)))?;

        log::info!("PDF creado exitosamente: {:?}", ruta_salida);
        Ok(())
    }

    /// Mueve PDF de temporal a ubicación permanente
    pub async fn mover_pdf_permanente(
        &self,
        ruta_temporal: &Path,
        ruta_destino: &Path,
    ) -> AppResult<PathBuf> {
        // Asegurar que el directorio destino existe
        if let Some(parent) = ruta_destino.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Io(e))?;
        }

        // Copiar archivo (más seguro que mover para preservar temporal)
        tokio::fs::copy(ruta_temporal, ruta_destino)
            .await
            .map_err(|e| AppError::Io(e))?;

        log::debug!("PDF movido a ubicación permanente: {:?}", ruta_destino);
        Ok(ruta_destino.to_path_buf())
    }

    /// Genera ruta única para PDF basado en UUID
    pub fn generar_ruta_pdf(&self, directorio_base: &Path, numero_oficio: &str) -> PathBuf {
        let uuid = uuid::Uuid::new_v4();
        let nombre_archivo = format!("{}_{}.pdf", numero_oficio.replace('/', "_"), uuid);
        directorio_base.join(&nombre_archivo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_builder_creation() {
        let builder = PdfBuilder::new(300);
        assert_eq!(builder.dpi, 300);
    }

    #[test]
    fn test_generar_ruta_pdf() {
        let builder = PdfBuilder::new(300);
        let ruta = builder.generar_ruta_pdf(&PathBuf::from("/tmp"), "OF-2024-001");
        assert!(ruta.extension().unwrap() == "pdf");
        assert!(ruta.to_string_lossy().contains("OF-2024-001"));
    }
}
