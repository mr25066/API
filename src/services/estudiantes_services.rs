use sqlx::PgPool;

use crate::models::estudiantes_model::{ActualizarEstudiante, CrearEstudiante, Estudiante};
use crate::repository::estudiantes_repository;

pub async fn obtener_estudiantes_service(
    pool: &PgPool,
) -> Result<Vec<Estudiante>, String> {
    estudiantes_repository::obtener_estudiantes(pool)
        .await
        .map_err(|e| format!("Error en BD: {}", e))
}

pub async fn crear_estudiante_service(
    pool: &PgPool,
    data: CrearEstudiante,
) -> Result<Estudiante, String> {
    if data.nombre.trim().is_empty() {
        return Err("El nombre es obligatorio".to_string());
    }

    if data.apellido.trim().is_empty() {
        return Err("El apellido es obligatorio".to_string());
    }

    if data.carnet.trim().is_empty() {
        return Err("El carnet es obligatorio".to_string());
    }

    if data.carnet.len() > 10 {
        return Err("El carnet no puede tener más de 10 caracteres".to_string());
    }

    estudiantes_repository::crear_estudiante(pool, data)
        .await
        .map_err(|e| format!("Error al crear estudiante: {}", e))
}

pub async fn obtener_estudiante_por_id_service(
    pool: &PgPool,
    id: i32,
) -> Result<Estudiante, String> {
    if id <= 0 {
        return Err("El id del estudiante no es válido".to_string());
    }

    estudiantes_repository::obtener_estudiante_por_id(pool, id)
        .await
        .map_err(|e| format!("Error al obtener estudiante: {}", e))
}

pub async fn actualizar_estudiante_service(
    pool: &PgPool,
    id: i32,
    data: ActualizarEstudiante,
) -> Result<Estudiante, String> {
    if id <= 0 {
        return Err("El id del estudiante no es válido".to_string());
    }

    if data.nombre.trim().is_empty() {
        return Err("El nombre es obligatorio".to_string());
    }

    if data.apellido.trim().is_empty() {
        return Err("El apellido es obligatorio".to_string());
    }

    if data.carnet.trim().is_empty() {
        return Err("El carnet es obligatorio".to_string());
    }

    if data.carnet.len() > 10 {
        return Err("El carnet no puede tener más de 10 caracteres".to_string());
    }

    estudiantes_repository::actualizar_estudiante(pool, id, data)
        .await
        .map_err(|e| format!("Error al actualizar estudiante: {}", e))
}

pub async fn eliminar_estudiante_service(
    pool: &PgPool,
    id: i32,
) -> Result<(), String> {
    if id <= 0 {
        return Err("El id del estudiante no es válido".to_string());
    }

    estudiantes_repository::eliminar_estudiante(pool, id)
        .await
        .map_err(|e| format!("Error al eliminar estudiante: {}", e))
}
