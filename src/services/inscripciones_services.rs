use sqlx::PgPool;
use crate::models::inscripciones_model::{CrearInscripcion, Inscripcion};
use crate::repository::inscripciones_repository;

#[derive(Debug)]
pub enum ServiceError {
    ValidationError(String),
    NotFound(String),
    DatabaseError(String),
}

pub async fn crear_inscripcion_service(
    pool: &PgPool,
    data: CrearInscripcion,
) -> Result<Inscripcion, ServiceError> {
    if data.ciclo.trim().is_empty() {
        return Err(ServiceError::ValidationError(
            "El ciclo no puede estar vacío".to_string(),
        ));
    }

    if let Some(nota) = data.nota_final {
        if nota < 0.0 || nota > 10.0 {
            return Err(ServiceError::ValidationError(
                "La nota final debe estar entre 0 y 10".to_string(),
            ));
        }
    }

    inscripciones_repository::crear_inscripcion(
        pool,
        data.id_estudiante,
        data.id_materia,
        &data.ciclo,
        data.nota_final,
    )
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

pub async fn listar_inscripciones_service(
    pool: &PgPool,
) -> Result<Vec<Inscripcion>, ServiceError> {
    inscripciones_repository::listar_inscripciones(pool)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

pub async fn obtener_inscripcion_service(
    pool: &PgPool,
    id: i32,
) -> Result<Inscripcion, ServiceError> {
    if id <= 0 {
        return Err(ServiceError::ValidationError(
            "ID inválido".to_string(),
        ));
    }

    inscripciones_repository::obtener_inscripcion(pool, id)
        .await
        .map_err(|_| {
            ServiceError::NotFound("Inscripción no encontrada".to_string())
        })
}

pub async fn actualizar_inscripcion_service(
    pool: &PgPool,
    id: i32,
    data: CrearInscripcion,
) -> Result<Inscripcion, ServiceError> {
    if id <= 0 {
        return Err(ServiceError::ValidationError(
            "ID inválido".to_string(),
        ));
    }

    if data.ciclo.trim().is_empty() {
        return Err(ServiceError::ValidationError(
            "El ciclo no puede estar vacío".to_string(),
        ));
    }

    if let Some(nota) = data.nota_final {
        if nota < 0.0 || nota > 10.0 {
            return Err(ServiceError::ValidationError(
                "La nota final debe estar entre 0 y 10".to_string(),
            ));
        }
    }

    inscripciones_repository::actualizar_inscripcion(
        pool,
        id,
        data.id_estudiante,
        data.id_materia,
        &data.ciclo,
        data.nota_final,
    )
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))
}

pub async fn eliminar_inscripcion_service(
    pool: &PgPool,
    id: i32,
) -> Result<(), ServiceError> {
    if id <= 0 {
        return Err(ServiceError::ValidationError(
            "ID inválido".to_string(),
        ));
    }

    let filas_afectadas = inscripciones_repository::eliminar_inscripcion(pool, id)
        .await
        .map_err(|e| ServiceError::DatabaseError(e.to_string()))?;

    if filas_afectadas == 0 {
        return Err(ServiceError::NotFound(
            "Inscripción no encontrada".to_string(),
        ));
    }

    Ok(())
}