mod models;
mod db;
mod dispatcher;

use axum::{routing::post, Json, Router, extract::State};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqlitePool};
use std::str::FromStr;
use std::time::Duration;
use std::net::SocketAddr;
use std::sync::Arc;

// El estado compartido para que el servidor web pueda usar la base de datos
struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando Orquestador de Colas en Rust...");

    // Carga de variables de entorno
    dotenvy::dotenv().ok();
    let url_db = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://queue.db".to_string());

    println!("Intentando conectar a: {}", url_db);

    // Opciones de conexión creando el archivo si no existe
    let connection_options = SqliteConnectOptions::from_str(&url_db)?
        .create_if_missing(true);

    // Creación del Pool corregida (sin cortes intermedios)
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(connection_options)
        .await?;

    println!("Conectado a SQLite exitosamente (y archivo creado si faltaba).");

    // Migración de la tabla
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

    println!("Tabla 'tasks' verificada y lista para operar.");

    // --- LEVANTAR EL SERVIDOR WEB (AXUM) ---
    // Envolvemos el pool en un Arc (referencia segura para hilos)
    let shared_state = Arc::new(AppState { pool: pool.clone() });

    // Definimos el endpoint POST /api/tareas
    let app = Router::new()
        .route("/api/tareas", post(recibir_tarea))
        .with_state(shared_state);

    // Usamos 0.0.0.0 para que esté listo para Docker
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌍 Servidor API escuchando en http://{}", addr);

    // Lanzamos el servidor en un hilo verde de fondo (spawn)
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    // ----------------------------------------

    // Cliente HTTP global
    let cliente_http = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    println!("Iniciando el Radar de Tareas. Esperando trabajo...");

    // El corazón de tu orquestador: un bucle infinito que no bloquea la CPU
    loop {
        match db::extraer_siguiente_tarea(&pool).await {
            Ok(Some(tarea)) => {
                println!("¡Tarea capturada! ID: {} | Dirigida a: {}", tarea.id, tarea.target_worker);
                
                match dispatcher::enviar_a_worker(&cliente_http, &tarea).await {
                    Ok(respuesta_worker) => {
                        println!("Tarea {} completada por el worker. Respuesta: {}", tarea.id, respuesta_worker);

                        if let Err(e) = db::completar_tarea(&pool, &tarea.id).await {
                            println!("Error al eliminar la tarea completada de la DB: {}", e)
                        }
                    },
                    Err(e) => {
                        println!("Error al contactar con {}: {}", tarea.target_worker, e);
                    
                        if let Err(db_err) = db::registrar_fallo_tarea(&pool, &tarea.id, tarea.retries).await {
                            println!("Error crítico al registrar en la BD: {}", db_err)
                        }
                    }
                }
            },
            Ok(None) => {
                // La cola está vacía. Dormimos el radar 1 segundo.
                tokio::time::sleep(Duration::from_secs(1)).await;
            },
            Err(e) => {
                println!("Error en el radar: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

// --- FUNCIÓN DEL ENDPOINT HTTP ---
// Este es el recepcionista que atiende las peticiones de Postman
async fn recibir_tarea(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<models::TareaRequest>,
) -> String {
    
    // Llamamos a la función de db.rs para guardar la tarea usando los datos que llegaron
    match db::encolar_tarea(&state.pool, &payload.target_worker, &payload.payload).await {
        Ok(id_generado) => {
            format!("Tarea registrada en la base de datos con éxito. ID: {}", id_generado)
        },
        Err(error) => {
            format!("Error interno al guardar la tarea: {}", error)
        }
    }
}