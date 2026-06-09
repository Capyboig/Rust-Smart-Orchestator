use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
    /* El pub basicamente es para hacer que sea de facil acceso a la informacion de dentro tanto para crear la estructura como para quitarla
        A parte el struct es básicamente un DTO de java, para que tengamos la informacion.
    */
pub struct Tarea {
    pub id: String,
    pub status: String,
    pub payload: String,
    pub target_workers: String,
    pub retries: i32,
}