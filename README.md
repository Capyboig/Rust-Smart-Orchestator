# 🦀 Rust Smart Orchestrator

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust&style=flat-square)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Async_Runtime-Tokio_1.35-blue?style=flat-square)](https://tokio.rs/)
[![Database](https://img.shields.io/badge/Persistence-SQLite_+_SQLx-003B57?logo=sqlite&style=flat-square)](https://github.com/launchbadge/sqlx)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)

Un orquestador y gestor de colas de tareas asíncronas de alto rendimiento y baja latencia desarrollado en **Rust**, diseñado para el despacho concurrente y confiable de trabajos distribuidos hacia microservicios heterogéneos (Java, Python).

---

## 🚀 Arquitectura y Diseño

El sistema implementa un patrón de procesamiento asíncrono no bloqueante respaldado por **Tokio** y una capa de persistencia transaccional mediante **SQLx**:



### Características Principales
* **Ejecución Asíncrona Concurrente:** Procesamiento de eventos y tareas sin bloqueo de hilos utilizando el runtime `tokio`.
* **Pool de Conexiones Optimizado:** Gestión de conexiones SQLite con `SqlitePoolOptions` (reutilización de descriptores, timeouts y concurrencia controlada).
* **Persistencia Fiable:** Registro transaccional de estados (`PENDING`, `PROCESSING`, `COMPLETED`, `FAILED`) con reintentos programados.
* **Payload Acelerado con Serde:** Serialización y tipado estricto de estructuras de tareas con identificación única mediante UUID v4.

---

## 🛠️ Stack Tecnológico

* **Lenguaje:** Rust (Edición 2024).
* **Runtime Asíncrono:** `tokio` (con características completas multi-thread).
* **Capa de Persistencia:** `sqlx` (con soporte para `sqlite`, `runtime-tokio-rustls`, `uuid`).
* **Serialización:** `serde` y `serde_json`.
* **Variables de Entorno:** `dotenvy`.

---


## ⚙️ Puesta en Marcha

### Prerrequisitos
* [Rust y Cargo](https://www.rust-lang.org/tools/install) (versión 1.75 o superior recomendada).

### Configuración
1. Clona el repositorio:
   ```bash
   git clone https://github.com/Capyboig/Rust-Smart-Orchestator.git
   cd Rust-Smart-Orchestator/rust-queue








1. Base de datos
```bash
   CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    target_worker TEXT NOT NULL,
    retries INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
