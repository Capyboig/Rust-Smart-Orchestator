use serde::{Deserialize, Serialize};

// El molde interno para la base de datos y el worker
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tarea {
    pub id: String,
    pub status: String,
    pub payload: String,
    pub target_worker: String,
    pub retries: i32,
}

// El molde externo para recibir peticiones limpias desde Postman
#[derive(Debug, Deserialize)]
pub struct TareaRequest {
    pub payload: String,
    pub target_worker: String,
}