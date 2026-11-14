# 🦀 Rust Smart Orchestrator

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust&style=flat-square)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Async_Runtime-Tokio_1.35-blue?style=flat-square)](https://tokio.rs/)
[![Database](https://img.shields.io/badge/Persistence-SQLite_+_SQLx-003B57?logo=sqlite&style=flat-square)](https://github.com/launchbadge/sqlx)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)

Un orquestador y gestor de colas de tareas asíncronas de alto rendimiento y baja latencia desarrollado en **Rust**, diseñado para el despacho concurrente y confiable de trabajos distribuidos hacia microservicios heterogéneos (Java, Python).

---

## 🚀 Arquitectura y Diseño

El sistema implementa un patrón de procesamiento asíncrono no bloqueante respaldado por **Tokio** y una capa de persistencia transaccional mediante **SQLx**:
