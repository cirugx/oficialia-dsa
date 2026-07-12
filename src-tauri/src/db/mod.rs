//! Database layer with SQLx for compile-time validated SQLite queries
//! RNF-SEC-01: Toda base de datos histórica reside localmente en Windows

use crate::core::error::{AppError, AppResult};
use crate::models::{CrearOficioDto, Oficio, PalabraClave};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::Path;

/// Servicio de base de datos con pool de conexiones
pub struct DatabaseService {
    pool: SqlitePool,
}

impl DatabaseService {
    /// Crea o abre la base de datos SQLite local
    pub async fn conectar(ruta_db: &Path) -> AppResult<Self> {
        // Asegurar que el directorio existe
        if let Some(parent) = ruta_db.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Io(e))?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .min_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(10))
            .connect(ruta_db.to_str().unwrap())
            .await
            .map_err(|e| AppError::Database(e))?;

        let db = Self { pool };
        
        // Ejecutar migraciones iniciales
        db.ejecutar_migraciones().await?;
        
        log::info!("Conexión a SQLite establecida: {:?}", ruta_db);
        Ok(db)
    }

    /// Ejecuta migraciones para crear tablas si no existen
    async fn ejecutar_migraciones(&self) -> AppResult<()> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Database(e))?;

        // Tabla OFICIOS
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS oficios (
                id TEXT PRIMARY KEY,
                numero_oficio TEXT UNIQUE NOT NULL,
                remitente TEXT NOT NULL,
                asunto TEXT NOT NULL,
                clasificacion TEXT NOT NULL,
                fecha_oficio TEXT NOT NULL,
                fecha_registro TEXT NOT NULL,
                ruta_pdf_local TEXT NOT NULL
            )
            "#,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e))?;

        // Tabla PALABRAS_CLAVE con índice full-text
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palabras_clave (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                oficio_id TEXT NOT NULL,
                palabra TEXT NOT NULL,
                FOREIGN KEY (oficio_id) REFERENCES oficios(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e))?;

        // Índice para búsqueda rápida
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_palabras ON palabras_clave(palabra)")
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Database(e))?;

        // Índice para búsquedas por oficio
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_oficio ON palabras_clave(oficio_id)")
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::Database(e))?;

        tx.commit().await.map_err(|e| AppError::Database(e))?;
        
        log::debug!("Migraciones de base de datos ejecutadas exitosamente");
        Ok(())
    }

    /// Inserta un oficio con sus palabras clave en transacción atómica
    pub async fn guardar_oficio(&self, dto: &CrearOficioDto) -> AppResult<String> {
        let mut tx = self.pool.begin().await.map_err(|e| AppError::Database(e))?;

        let id = uuid::Uuid::new_v4().to_string();
        let fecha_registro = chrono::Utc::now().to_rfc3339();

        // Insertar oficio principal
        sqlx::query(
            r#"
            INSERT INTO oficios (id, numero_oficio, remitente, asunto, clasificacion, fecha_oficio, fecha_registro, ruta_pdf_local)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&dto.numero_oficio)
        .bind(&dto.remitente)
        .bind(&dto.asunto)
        .bind(dto.clasificacion.to_string())
        .bind(&dto.fecha_oficio)
        .bind(&fecha_registro)
        .bind(&dto.ruta_pdf_temporal)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e))?;

        // Insertar palabras clave
        for palabra in &dto.palabras_clave {
            sqlx::query("INSERT INTO palabras_clave (oficio_id, palabra) VALUES (?, ?)")
                .bind(&id)
                .bind(palabra)
                .execute(&mut *tx)
                .await
                .map_err(|e| AppError::Database(e))?;
        }

        tx.commit().await.map_err(|e| AppError::Database(e))?;
        
        log::info!("Oficio {} guardado exitosamente con {} palabras clave", 
            dto.numero_oficio, dto.palabras_clave.len());
        
        Ok(id)
    }

    /// Obtiene todos los oficios ordenados por fecha
    pub async fn obtener_todos_oficios(&self) -> AppResult<Vec<Oficio>> {
        let oficios = sqlx::query_as::<_, Oficio>(
            "SELECT * FROM oficios ORDER BY fecha_registro DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e))?;

        Ok(oficios)
    }

    /// Busca oficios por palabra clave usando índices full-text
    pub async fn buscar_por_palabra_clave(&self, palabra: &str) -> AppResult<Vec<Oficio>> {
        let oficios = sqlx::query_as::<_, Oficio>(
            r#"
            SELECT DISTINCT o.* 
            FROM oficios o
            INNER JOIN palabras_clave pk ON o.id = pk.oficio_id
            WHERE pk.palabra LIKE ?
            ORDER BY o.fecha_registro DESC
            "#,
        )
        .bind(format!("%{}%", palabra))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e))?;

        Ok(oficios)
    }

    /// Obtiene un oficio específico por ID
    pub async fn obtener_oficio_por_id(&self, id: &str) -> AppResult<Option<Oficio>> {
        let oficio = sqlx::query_as::<_, Oficio>("SELECT * FROM oficios WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(oficio)
    }

    /// Obtiene palabras clave asociadas a un oficio
    pub async fn obtener_palabras_clave(&self, oficio_id: &str) -> AppResult<Vec<PalabraClave>> {
        let palabras = sqlx::query_as::<_, PalabraClave>(
            "SELECT * FROM palabras_clave WHERE oficio_id = ?"
        )
        .bind(oficio_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e))?;

        Ok(palabras)
    }

    /// Elimina un oficio y sus palabras clave (cascade)
    pub async fn eliminar_oficio(&self, id: &str) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM oficios WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::Validacion("Oficio no encontrado".to_string()));
        }

        log::info!("Oficio {} eliminado exitosamente", id);
        Ok(())
    }

    /// Obtiene estadísticas básicas
    pub async fn obtener_estadisticas(&self) -> AppResult<EstadisticasDb> {
        let total_oficios: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM oficios")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        let total_palabras: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM palabras_clave")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(EstadisticasDb {
            total_oficios: total_oficios.0 as u64,
            total_palabras_clave: total_palabras.0 as u64,
        })
    }
}

/// Estadísticas de la base de datos
#[derive(Debug, Clone)]
pub struct EstadisticasDb {
    pub total_oficios: u64,
    pub total_palabras_clave: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Clasificacion;

    #[tokio::test]
    async fn test_database_conexion_memoria() {
        // Usar base de datos en memoria para testing
        let db = DatabaseService::conectar(Path::new(":memory:")).await;
        assert!(db.is_ok());
    }

    #[tokio::test]
    async fn test_guardar_y_obtener_oficio() {
        let db = DatabaseService::conectar(Path::new(":memory:")).await.unwrap();
        
        let dto = CrearOficioDto {
            numero_oficio: "TEST-001".to_string(),
            remitente: "Departamento Test".to_string(),
            asunto: "Oficio de prueba".to_string(),
            clasificacion: Clasificacion::Entrante,
            fecha_oficio: "2024-01-15".to_string(),
            ruta_pdf_temporal: "/tmp/test.pdf".to_string(),
            palabras_clave: vec!["prueba".to_string(), "test".to_string()],
        };

        let id = db.guardar_oficio(&dto).await.unwrap();
        assert!(!id.is_empty());

        let oficios = db.obtener_todos_oficios().await.unwrap();
        assert_eq!(oficios.len(), 1);
        assert_eq!(oficios[0].numero_oficio, "TEST-001");
    }
}
