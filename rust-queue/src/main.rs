mod models;
mod db;
mod dispatcher;
mod config; // <-- Nuevo módulo

use axum::{routing::post, Json, Router, extract::State};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqlitePool};
use std::str::FromStr;
use std::time::Duration;
use std::net::SocketAddr;
use std::sync::Arc;

struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Iniciando Orquestador de Colas en Rust...");

    // 1. Cargamos el YAML de forma limpia
    let config = config::cargar();
    println!("⚙️  Configuración cargada correctamente desde YAML.");

    // 2. Base de datos usando el YAML
    let connection_options = SqliteConnectOptions::from_str(&config.database.url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(config.database.max_connections)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(connection_options)
        .await?;

    println!("✅ Conectado a SQLite exitosamente.");

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

    // 3. Servidor Axum usando el puerto del YAML
    let shared_state = Arc::new(AppState { pool: pool.clone() });

    let app = Router::new()
        .route("/api/tareas", post(recibir_tarea))
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    println!("🌍 Servidor API escuchando en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 4. Cliente HTTP usando el timeout del YAML
    let cliente_http = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.worker.timeout_seconds))
        .build()?;

    // --- MAGIA: GRACEFUL SHUTDOWN ---
    // Creamos un canal de comunicación entre el sistema operativo y nuestro bucle
    let (tx_apagado, mut rx_apagado) = tokio::sync::mpsc::channel(1);
    
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("\n🛑 Señal de apagado recibida. Terminando operaciones en curso...");
        tx_apagado.send(()).await.unwrap();
    });
    // --------------------------------

    println!("📡 Iniciando el Radar de Tareas. Esperando trabajo...");

    // El corazón de tu orquestador (Ahora 100% concurrente)
    loop {
        // Verificamos si alguien ha pedido apagar el servidor
        if rx_apagado.try_recv().is_ok() {
            println!("🔌 Radar detenido. Cerrando base de datos y saliendo limpiamente. ¡Adiós!");
            break;
        }

        match db::extraer_siguiente_tarea(&pool).await {
            Ok(Some(tarea)) => {
                println!("🎯 ¡Tarea capturada! ID: {} | Dirigida a: {}", tarea.id, tarea.target_worker);
                
                // Clonamos los punteros de conexión para dárselos al hilo verde
                let pool_clon = pool.clone();
                let cliente_clon = cliente_http.clone();

                // MAGIA: Lanzamos el procesamiento en un hilo independiente (fire-and-forget)
                tokio::spawn(async move {
                    match dispatcher::enviar_a_worker(&cliente_clon, &tarea).await {
                        Ok(respuesta_worker) => {
                            println!("✅ Tarea {} completada. Respuesta: {}", tarea.id, respuesta_worker);
                            if let Err(e) = db::completar_tarea(&pool_clon, &tarea.id).await {
                                println!("⚠️ Error al eliminar tarea de la DB: {}", e)
                            }
                        },
                        Err(e) => {
                            println!("❌ Error al contactar con {}: {}", tarea.target_worker, e);
                            if let Err(db_err) = db::registrar_fallo_tarea(&pool_clon, &tarea.id, tarea.retries).await {
                                println!("⚠️ Error crítico al registrar fallo: {}", db_err)
                            }
                        }
                    }
                });
            },
            Ok(None) => {
                // Cola vacía. Reducimos el descanso a 500ms para que sea más reactivo
                tokio::time::sleep(Duration::from_millis(500)).await;
            },
            Err(e) => {
                println!("⚠️ Error en el radar: {}", e);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    Ok(())
}

// Handler HTTP
async fn recibir_tarea(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<models::TareaRequest>,
) -> String {
    match db::encolar_tarea(&state.pool, &payload.target_worker, &payload.payload).await {
        Ok(id_generado) => {
            format!("✅ Tarea registrada en la base de datos con éxito. ID: {}", id_generado)
        },
        Err(error) => {
            format!("❌ Error interno al guardar la tarea: {}", error)
        }
    }
}