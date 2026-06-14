use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub worker: WorkerConfig,
}


#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct WorkerConfig {
    pub timeout_seconds: u64,
}

pub fn cargar() -> AppConfig {

let contenido = fs::read_to_string("config.yaml")
.expect("❌ Error: No se encontró el archivo config.yaml en la raíz del proyecto.");

serde_yaml::from_str(&contenido)
.expect("❌ Error: El formato del config.yaml es inválido.")

}