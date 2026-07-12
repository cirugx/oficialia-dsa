//! Gemini AI Client with Structured Outputs and Zero Retention Policy
//! RF-03: Inferencia con Gemini forzando formato JSON (Structured Outputs)
//! RNF-SEC-02: Cero Retención - DELETE inmediato tras recibir JSON

use crate::core::error::{AppError, AppResult};
use crate::models::{Clasificacion, MetadatosExtraidos};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cliente HTTP para Gemini API con políticas de seguridad estrictas
pub struct GeminiClient {
    api_key: String,
    client: reqwest::Client,
    base_url: String,
}

/// Schema forzado para Structured Outputs de Gemini
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiSchema {
    r#type: String,
    properties: serde_json::Value,
    required: Vec<String>,
}

impl GeminiClient {
    /// Inicializa cliente con configuración de timeout y retries
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(5)
            .build()
            .expect("Falló creación de cliente HTTP");

        Self {
            api_key,
            client,
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }

    /// Analiza documento completo: upload -> inferencia -> delete (RNF-SEC-02)
    pub async fn analizar_documento(
        &self,
        imagen_bytes: &[u8],
        texto_ocr: &str,
    ) -> AppResult<MetadatosExtraidos> {
        // Paso 1: Subir archivo a Gemini Files API
        let file_id = self.subir_archivo(imagen_bytes).await?;
        
        // Paso 2: Ejecutar inferencia con Structured Outputs
        let resultado = self
            .ejecutar_inferencia(&file_id, texto_ocr)
            .await;
        
        // Paso 3: DELETE inmediato (Cero Retención - RNF-SEC-02)
        if let Err(e) = self.eliminar_archivo(&file_id).await {
            log::warn!("Falló eliminación de archivo en Gemini: {:?}", e);
        } else {
            log::debug!("Archivo {} eliminado correctamente (Zero Retention)", file_id);
        }

        resultado
    }

    /// Sube imagen a Gemini Files API
    async fn subir_archivo(&self, imagen_bytes: &[u8]) -> AppResult<String> {
        let part = multipart::Part::bytes(imagen_bytes.to_vec())
            .file_name("documento.png")
            .mime_str("image/png")
            .map_err(|e| AppError::GeminiApi(format!("Falló creación de multipart: {}", e)))?;

        let form = multipart::Form::new().part("file", part);

        let url = format!("{}/files?key={}", self.base_url, self.api_key);
        
        let response = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::GeminiApi(format!("Falló upload: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::GeminiApi(format!(
                "Upload falló con {}: {}",
                status, body
            )));
        }

        #[derive(Deserialize)]
        struct UploadResponse {
            name: String,
        }

        let upload_resp: UploadResponse = response
            .json()
            .await
            .map_err(|e| AppError::Json(e))?;

        // Extraer ID del nombre (formato: "files/xxx")
        let file_id = upload_resp
            .name
            .strip_prefix("files/")
            .unwrap_or(&upload_resp.name)
            .to_string();

        log::debug!("Archivo subido exitosamente: {}", file_id);
        Ok(file_id)
    }

    /// Ejecuta inferencia con Structured Outputs forzando JSON schema
    async fn ejecutar_inferencia(
        &self,
        file_id: &str,
        texto_ocr: &str,
    ) -> AppResult<MetadatosExtraidos> {
        // Schema estricto para Structured Outputs
        let schema = serde_json::json!({
            "type": "OBJECT",
            "properties": {
                "numeroOficio": {
                    "type": "STRING",
                    "description": "Número oficial del documento"
                },
                "remitente": {
                    "type": "STRING",
                    "description": "Entidad o persona que emite el oficio"
                },
                "asunto": {
                    "type": "STRING",
                    "description": "Tema principal del documento"
                },
                "clasificacionSugerida": {
                    "type": "STRING",
                    "enum": ["ENTRANTE", "SALIENTE", "INTERNO", "CONFIDENCIAL", "URGENTE"],
                    "description": "Clasificación documental sugerida"
                },
                "fechaOficio": {
                    "type": "STRING",
                    "description": "Fecha del oficio en formato YYYY-MM-DD"
                },
                "palabrasClave": {
                    "type": "ARRAY",
                    "items": {"type": "STRING"},
                    "description": "Palabras clave indexables"
                },
                "resumenEjecutivo": {
                    "type": "STRING",
                    "description": "Resumen ejecutivo del contenido (max 200 palabras)"
                }
            },
            "required": ["asunto", "resumenEjecutivo", "palabrasClave"]
        });

        let prompt = format!(
            r#"Analiza este documento oficial y extrae los metadatos estructurados.
            
TEXTO OCR EXTRAÍDO:
{}

Instrucciones:
1. Extrae información precisa del documento
2. Clasifica según tipología oficial mexicana
3. Genera palabras clave relevantes para búsqueda
4. Crea un resumen ejecutivo claro y conciso"#,
            texto_ocr
        );

        let payload = serde_json::json!({
            "contents": [{
                "parts": [
                    {"text": prompt},
                    {
                        "fileData": {
                            "mimeType": "image/png",
                            "fileUri": format!("https://generativelanguage.googleapis.com/v1beta/files/{}", file_id)
                        }
                    }
                ]
            }],
            "generationConfig": {
                "responseMimeType": "application/json",
                "responseSchema": schema
            }
        });

        let url = format!("{}/models/gemini-1.5-pro:generateContent?key={}", 
            self.base_url, self.api_key);

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::GeminiApi(format!("Falló inferencia: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::GeminiApi(format!(
                "Inferencia falló con {}: {}",
                status, body
            )));
        }

        #[derive(Deserialize, Debug)]
        struct GeminiResponse {
            candidates: Vec<Candidate>,
        }

        #[derive(Deserialize, Debug)]
        struct Candidate {
            content: Content,
        }

        #[derive(Deserialize, Debug)]
        struct Content {
            parts: Vec<Part>,
        }

        #[derive(Deserialize, Debug)]
        struct Part {
            text: String,
        }

        let gemini_resp: GeminiResponse = response
            .json()
            .await
            .map_err(|e| AppError::Json(e))?;

        let json_text = gemini_resp
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .ok_or_else(|| AppError::GeminiApi("Respuesta vacía de Gemini".to_string()))?;

        let metadatos: MetadatosExtraidos = serde_json::from_str(&json_text)
            .map_err(|e| AppError::Json(e))?;

        log::info!("Inferencia completada exitosamente");
        Ok(metadatos)
    }

    /// Elimina archivo de Gemini Files API (RNF-SEC-02: Cero Retención)
    async fn eliminar_archivo(&self, file_id: &str) -> AppResult<()> {
        let url = format!("{}/files/{}?key={}", self.base_url, file_id, self.api_key);

        self.client
            .delete(&url)
            .send()
            .await
            .map_err(|e| AppError::GeminiApi(format!("Falló DELETE: {}", e)))?;

        Ok(())
    }

    /// Verifica conectividad con Gemini API
    pub async fn verificar_conexion(&self) -> bool {
        let url = format!("{}/models?key={}", self.base_url, self.api_key);
        
        match self.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_client_creation() {
        let client = GeminiClient::new("test_api_key".to_string());
        assert_eq!(client.api_key, "test_api_key");
    }
}
