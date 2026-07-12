# Oficialia DSA - Sistema de Gestión Documental con OCR e IA

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue.svg)](https://www.typescriptlang.org)

## 📋 Descripción

Sistema profesional de gestión documental que implementa:

- **Escaneo hardware** vía WIA/TWAIN a 300 DPI
- **OCR local offline** con Windows.Media.Ocr (< 1.5s)
- **IA estructurada** con Gemini API (Structured Outputs + Zero Retention)
- **Pipeline desacoplado** en dos fases (Análisis → Auditoría → Persistencia)
- **Base de datos local** SQLite con validación SQL en tiempo de compilación
- **Visor PDF nativo** usando `convertFileSrc` de Tauri

## 🏗️ Arquitectura

### Stack Tecnológico Unificado (STU)

| Capa | Tecnología | Función Principal |
| --- | --- | --- |
| Frontend UI | React + TypeScript + Vite | Interfaz reactiva y tipado estricto |
| Diseño y Estados | Shadcn UI + TanStack Query | Estilizado accesible y gestión asíncrona |
| Contenedor Desktop | Tauri v2 + WebView2 | Puente IPC nativo ligero |
| Backend Core | Rust (Edición 2021) + Tokio | Concurrencia segura |
| Hardware & OCR | windows-rs | WIA/TWAIN + Windows Media OCR |
| Motor PDF | lopdf + image | Ensamble binario con texto invisible |
| Red y API | reqwest + serde | Cliente HTTP para Gemini |
| Persistencia | SQLite + sqlx | Validación SQL compile-time |

## 🔧 Requisitos

### Sistema Operativo
- **Windows 10/11** (requerido para WIA/TWAIN y Windows.Media.Ocr)

### Dependencias
- [Rust](https://rustup.rs/) (Edition 2021+)
- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/) o npm

## 🚀 Instalación

```bash
# Instalar dependencias frontend
pnpm install

# Configurar variables de entorno
cp .env.example .env
# Editar .env con tu GEMINI_API_KEY

# Ejecutar en modo desarrollo
pnpm tauri dev

# Build de producción
pnpm tauri build
```

## 📁 Estructura del Proyecto

```
oficialia-dsa/
├── src/                    # Frontend React + TypeScript
│   ├── components/         # Componentes UI (Shadcn)
│   ├── hooks/              # Custom hooks (TanStack Query)
│   └── types/              # Tipos TypeScript
├── src-tauri/              # Backend Rust
│   ├── src/
│   │   ├── core/           # Lógica principal
│   │   │   ├── scanner.rs      # WIA/TWAIN integration
│   │   │   ├── ocr.rs          # Windows.Media.Ocr
│   │   │   ├── gemini_client.rs # Gemini API client
│   │   │   └── pdf_builder.rs   # PDF generation
│   │   ├── db/             # Database layer (SQLx)
│   │   ├── models/         # Domain models
│   │   └── commands/       # Tauri commands (IPC)
│   └── Cargo.toml          # Rust dependencies
└── package.json            # Frontend dependencies
```

## 🔑 Características Principales

### Pipeline Desacoplado
- **Fase 1**: Ingesta + Análisis (OCR + IA) ≤ 5s
- **Auditoría**: Revisión y edición manual en UI
- **Fase 2**: Persistencia transaccional en SQLite

### Seguridad
- **RNF-SEC-01**: Base de datos 100% local
- **RNF-SEC-02**: Cero retención en Gemini (DELETE inmediato)

### Rendimiento
- **RNF-PERF-01**: 60 FPS con I/O en threads dedicados
- **RNF-PERF-02**: OCR < 1.5s, Fase 1 < 5s

## 📊 Modelo de Datos

```sql
CREATE TABLE oficios (
    id TEXT PRIMARY KEY,
    numero_oficio TEXT UNIQUE NOT NULL,
    remitente TEXT NOT NULL,
    asunto TEXT NOT NULL,
    clasificacion TEXT NOT NULL,
    fecha_oficio TEXT NOT NULL,
    fecha_registro TEXT NOT NULL,
    ruta_pdf_local TEXT NOT NULL
);

CREATE TABLE palabras_clave (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    oficio_id TEXT NOT NULL,
    palabra TEXT NOT NULL,
    FOREIGN KEY (oficio_id) REFERENCES oficios(id) ON DELETE CASCADE
);
```

## 🧪 Testing

```bash
cd src-tauri
cargo test
```

## 📝 Licencia

Propietario - Todos los derechos reservados
