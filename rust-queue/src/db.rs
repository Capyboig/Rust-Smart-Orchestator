use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::Tarea;

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

    println!("[ENCOLADO] Tarea {} asignada a {}", id_unico, worker_destino);

    println!("Tarea {} asignada a {}", id_unico, worker_destino);
    Ok(id_unico)
}


pub async fn extraer_siguiente_tarea(pool: &SqlitePool) -> Result<Option<Tarea>, sqlx::Error> {
    // Buscamos la más antigua, la bloqueamos poniéndola en PROCESSING, y la devolvemos.
    // Al hacerlo todo en una sola orden SQL, garantizamos seguridad absoluta frente a hilos paralelos.
    let tarea = sqlx::query_as::<_, Tarea>(
        r#"
        UPDATE tasks
        SET status = 'PROCESSING'
        WHERE id = (
            SELECT id FROM tasks
            WHERE status = 'PENDING'
            ORDER BY created_at ASC
            LIMIT 1
        )
        RETURNING id, status, payload, target_worker, retries
        "#
    )
    .fetch_optional(pool) // fetch_optional devuelve Some(Tarea) si hay una, o None si la cola está vacía
    .await?;

    Ok(tarea)
}




pub async fn completar_tarea(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}



pub async fn registrar_fallo_tarea(pool: &SqlitePool, id: &str, retries_actuales: i32) -> Result<(), sqlx::Error> {
    let nuevos_retries = retries_actuales + 1;


    let nuevo_estado = if nuevos_retries >= 3 {"DLQ" } else {"PENDING"};

    sqlx::query(
        r#"
        UPDATE tasks 
        SET status = ?, retries = ?
        WHERE id = ?
        "#
    )
    .bind(nuevo_estado)
    .bind(nuevos_retries)
    .bind(id)
    .execute(pool)
    .await?;

    if nuevo_estado == "DLQ" {
        println!("Error | DLQ Tarea {} enviada a la cola de colas muertas tras 3 fallos", id);
    } else {
        println!("Reintento | Tarea {} devuelta a PENDING (Intento {}/3)", id, nuevos_retries);
    }

    Ok(())
}