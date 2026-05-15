use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;

use crate::models::inscripciones_model::CrearInscripcion;
use crate::services::inscripciones_services::{
    self,
    ServiceError,
};

pub async fn crear_inscripcion(
    State(pool): State<PgPool>,
    Json(data): Json<CrearInscripcion>,
) -> impl IntoResponse {
    match inscripciones_services::crear_inscripcion_service(&pool, data).await {
        Ok(inscripcion) => (StatusCode::CREATED, Json(inscripcion)).into_response(),
        Err(error) => manejar_error(error),
    }
}

pub async fn listar_inscripciones(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    match inscripciones_services::listar_inscripciones_service(&pool).await {
        Ok(inscripciones) => (StatusCode::OK, Json(inscripciones)).into_response(),
        Err(error) => manejar_error(error),
    }
}

pub async fn obtener_inscripcion(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match inscripciones_services::obtener_inscripcion_service(&pool, id).await {
        Ok(inscripcion) => (StatusCode::OK, Json(inscripcion)).into_response(),
        Err(error) => manejar_error(error),
    }
}

pub async fn actualizar_inscripcion(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(data): Json<CrearInscripcion>,
) -> impl IntoResponse {
    match inscripciones_services::actualizar_inscripcion_service(&pool, id, data).await {
        Ok(inscripcion) => (StatusCode::OK, Json(inscripcion)).into_response(),
        Err(error) => manejar_error(error),
    }
}

pub async fn eliminar_inscripcion(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match inscripciones_services::eliminar_inscripcion_service(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(error) => manejar_error(error),
    }
}

fn manejar_error(error: ServiceError) -> axum::response::Response {
    match error {
        ServiceError::ValidationError(mensaje) => {
            (StatusCode::BAD_REQUEST, mensaje).into_response()
        }
        ServiceError::NotFound(mensaje) => {
            (StatusCode::NOT_FOUND, mensaje).into_response()
        }
        ServiceError::DatabaseError(mensaje) => {
            (StatusCode::INTERNAL_SERVER_ERROR, mensaje).into_response()
        }
    }
}