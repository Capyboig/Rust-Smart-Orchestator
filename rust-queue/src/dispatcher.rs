use reqwest::Client;
use std::time::Duration;
use crate::models::Tarea;

// Función pública que intenta enviar la tarea al trabajador correspondiente
pub async fn enviar_a_worker(cliente: &Client, tarea: &Tarea) -> Result<String, reqwest::Error> {
    // En Docker, los contenedores se comunican por su nombre. 
    // Si target_worker es "java-worker", la URL será http://java-worker:8080/procesar
    let url = format!("http://{}:8080/procesar", tarea.target_worker);

    println!("Enviando Tarea {} hacia {}", tarea.id, url);

    // Ejecutamos la petición HTTP POST
    let respuesta = cliente
        .post(&url)
        .timeout(Duration::from_secs(5)) // EL SEGURO: Si el worker tarda más de 5s, cortamos la llamada
        .header("Content-Type", "application/json")
        .body(tarea.payload.clone()) // JSON  guardado
        .send()
        .await?;

    //(ej: "PDF generado con éxito")
    let texto_respuesta = respuesta.text().await?;
    
    Ok(texto_respuesta)
}