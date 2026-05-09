use sqlx::PgPool;
use crate::models::carreras_model::Carreras;
use crate::repository::carreras_repository;
#[derive(Debug)]
pub enum ServiceError {
    ValidationError(String),
    NotFound(String),
    DatabaseError(String),
}
pub async fn crear_carrera_service(
    pool: &PgPool,
    nombre: &str,
    facultad: &str,
) -> Result<Carreras, ServiceError> {
    if nombre.trim().is_empty() {
        return Err(ServiceError::ValidationError(
            "El nombre de la carrera no puede estar vacío".to_string(),
        ));
    }

    if facultad.trim().is_empty() {
        return Err(ServiceError::ValidationError(
            "La facultad no puede estar vacía".to_string(),
        ));
    }
    carreras_repository::crear_carrera(pool, nombre, facultad)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

pub async fn listar_carreras_service(
    pool: &PgPool,
) -> Result<Vec<Carreras>, ServiceError> {
    carreras_repository::listar_carreras(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}
pub async fn obtener_carrera_service(
    pool: &PgPool,
    id: i32,
) -> Result<Carreras, ServiceError> {
    if id <= 0 {
        return Err(ServiceError::ValidationError(
            "ID inválido".to_string(),
        ));
    }
    carreras_repository::obtener_carrera(pool, id)
        .await
        .map_err(|_| {
            ServiceError::NotFound("Carrera no encontrada".to_string())
        })
}
pub async fn actualizar_carrera_service(
    pool: &PgPool,
    id: i32,
    nombre: &str,
    facultad: &str,
) -> Result<Carreras, ServiceError> {
    if id <= 0 {
        return Err(ServiceError::ValidationError(
            "ID inválido".to_string(),
        ));
    }

    if nombre.trim().is_empty() || facultad.trim().is_empty() {
        return Err(ServiceError::ValidationError(
            "Datos inválidos".to_string(),
        ));
    }

    carreras_repository::actualizar_carrera(pool, id, nombre, facultad)
        .await
        .map_err(|_| {
            ServiceError::DatabaseError(
                "Error al actualizar la carrera".to_string(),
            )
        })
}
pub async fn eliminar_carrera_service(
    pool: &PgPool,
    id: i32,
) -> Result<(), ServiceError> {
    if id <= 0 {
        return Err(ServiceError::ValidationError(
            "ID inválido".to_string(),
        ));
    }
    carreras_repository::eliminar_carrera(pool, id)
        .await
        .map_err(|_| {
            ServiceError::DatabaseError(
                "Error al eliminar carrera".to_string(),
            )
        })?;
    Ok(())
}