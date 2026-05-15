use sqlx::PgPool;

use crate::models::profesor_model::{Profesor,Crearprofesor,ActualizarProfesor};
use crate::repository::profesor_repository;

pub async fn obtener_profesores_service(pool:&PgPool) -> Result<Vec<Profesor>, String>{
    profesor_repository::obtener_profesores(pool)
    .await
    .map_err(|e| format!("Error en BD: {}", e))
}
pub async fn crear_nueva(pool: &PgPool, data:Crearprofesor ) -> Result<Profesor, String> {
    profesor_repository::create(pool, data)
        .await
        .map_err(|e| format!("Error al crear: {}", e))
}

pub async fn actualizar_profesor_service(
    pool: &PgPool,
    id: i32,
    data: ActualizarProfesor,
) -> Result<Profesor, String> {

    profesor_repository::actualizar_profesor(pool, id, data)
        .await
        .map_err(|e| format!("Error DB: {}", e))
}

pub async fn eliminar_profesor_service(
    pool: &PgPool,
    id: i32,
) -> Result<(), String> {

    profesor_repository::eliminar_profesor(pool, id)
        .await
        .map_err(|e| format!("Error DB: {}", e))
}
pub async fn obtener_por_id_service(
    pool: &PgPool,
    id: i32,
) -> Result<Profesor, String> {

    profesor_repository::obtener_por_id(pool, id)
        .await
        .map_err(|e| format!("Error DB: {}", e))
}