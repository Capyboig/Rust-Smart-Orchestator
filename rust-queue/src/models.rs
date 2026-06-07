use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tarea {
    pub id: String,
    pub status: String,
    pub payload: String,
    pub target_workers: String,
    pub retries: i32,
}