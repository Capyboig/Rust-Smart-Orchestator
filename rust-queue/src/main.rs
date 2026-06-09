// Declaramos los módulos (archivos) que vamos a usar
mod models;
mod db;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando Orquestador de Colas en Rust...");

        //Importador de contraseñas en el .env de manera segura y efectiva.
    dotenvy::dotenv().ok();
        //Url de la base de datos extraida del .env.
    let url_db = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://queue.db".to_string());

        //Este print de aqui es solo de testing, mas adelante se borrara
    println!("Intentando conectar a: {}", url_db);
        //EN la parte de aqui establecemos conexion via url prestada de .env
    let connection_options = SqliteConnectOptions::from_str(&url_db)?
        .create_if_missing(true);
        //Esta parte de aqui es la importante, creamos la pool. Osea preparamos 5 Pools abiertas para peticiones y que no tarden en llegar.
        //Ya que si no usas pools el servidor tiene que abrir una conexion, preguntar ejecutar y cerrar, con las pools abiertas se mantiene siempre activa.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(connection_options)
        .await?;

    println!("Conectado a SQLite exitosamente (y archivo creado si faltaba).");
        //Esto basicamente es para que cuando se ejecute si no existe la base de datos la cree sobre la marcha
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
    .execute(&pool) //Esto lo ejecuta por la pool abierta que ya asignamos antes.
    .await?;

    println!("Tabla 'tasks' verificada y lista para operar.");

    // Esto son unicamente para testing, mas adelante se borraran.
    db::encolar_tarea(&pool, "java-worker", r#"{"accion": "generar_pdf", "usuario": 12}"#).await?;
    db::encolar_tarea(&pool, "python-worker", r#"{"accion": "analizar_fraude", "usuario": 12}"#).await?;
    db::encolar_tarea(&pool, "java-worker", r#"{"accion": "enviar_email", "usuario": 15}"#).await?;

    Ok(())
}