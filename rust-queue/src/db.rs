use sqlx::SqlitePool;
use uuid::Uuid;

// Función pública de alta velocidad para encolar trabajos
pub async fn encolar_tarea(pool: &SqlitePool, worker_destino: &str, datos_json: &str) -> Result<String, sqlx::Error> {
    let id_unico = Uuid::new_v4().to_string();
    
    sqlx::query(
        r#"
        INSERT INTO tasks (id, status, payload, target_worker)
        VALUES (?, 'PENDING', ?, ?)
        "#
    )
    .bind(&id_unico)
    .bind(datos_json)
    .bind(worker_destino)
    .execute(pool)
    .await?;

    println!("📥 [ENCOLADO] Tarea {} asignada a {}", id_unico, worker_destino);
    Ok(id_unico)
}