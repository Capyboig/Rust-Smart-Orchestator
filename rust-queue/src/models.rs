use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tarea {
    pub id: String,
    pub status: String,
    pub payload: String,
    pub target_worker: String,
    pub retries: i32,
}