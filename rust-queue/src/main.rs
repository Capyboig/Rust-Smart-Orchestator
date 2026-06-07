// Declaramos los módulos (archivos) que vamos a usar
mod models;
mod db;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando Orquestador de Colas en Rust...");

    dotenvy::dotenv().ok();
    
    let url_db = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://queue.db".to_string());

    println!("🔗 Intentando conectar a: {}", url_db);

    let connection_options = SqliteConnectOptions::from_str(&url_db)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(connection_options)
        .await?;

    println!("✅ Conectado a SQLite exitosamente (y archivo creado si faltaba).");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            payload TEXT NOT NULL,
            target_worker TEXT NOT NULL,
            retries INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await?;

    println!("🏗️  Tabla 'tasks' verificada y lista para operar.");

    // Simulamos la llegada de tráfico llamando al módulo 'db'
    db::encolar_tarea(&pool, "java-worker", r#"{"accion": "generar_pdf", "usuario": 12}"#).await?;
    db::encolar_tarea(&pool, "python-worker", r#"{"accion": "analizar_fraude", "usuario": 12}"#).await?;
    db::encolar_tarea(&pool, "java-worker", r#"{"accion": "enviar_email", "usuario": 15}"#).await?;

    Ok(())
}