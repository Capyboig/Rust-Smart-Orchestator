use sqlx::SqlitePool;
use uuid::Uuid;

// La funcion esta es para colocar en PENDING una tarea, como se puede ver pedimos prestados los values.
/* Usamos el &str básicamente como valor estático, para usarlo de referencia a los dinamicos del heap
   Lo hacemos asincrono por que es el principal motivo del proyecto
*/
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

    println!("Tarea {} asignada a {}", id_unico, worker_destino);
    Ok(id_unico)
}