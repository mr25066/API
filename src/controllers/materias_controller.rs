use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::materias_model::CreateMateriaDto;
use crate::repository::materias_repository;

pub async fn listar_materias(State(pool): State<PgPool>) -> impl IntoResponse {
    match materias_repository::listar_materias(&pool).await {
        Ok(materias) => (StatusCode::OK, Json(materias)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn obtener_materia(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match materias_repository::obtener_materia(&pool, id).await {
        Ok(materia) => (StatusCode::OK, Json(materia)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn crear_materia(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateMateriaDto>,
) -> impl IntoResponse {
    // Pasamos los datos del payload uno por uno al repo
    match materias_repository::crear_materia(
        &pool,
        &payload.nombre_materia,
        payload.uv,
        payload.id_profesor
    ).await {
        Ok(materia) => (StatusCode::CREATED, Json(materia)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn actualizar_materia(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateMateriaDto>,
) -> impl IntoResponse {
    match materias_repository::actualizar_materia(
        &pool,
        id,
        &payload.nombre_materia,
        payload.uv,
        payload.id_profesor
    ).await {
        Ok(materia) => (StatusCode::OK, Json(materia)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn eliminar_materia(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match materias_repository::eliminar_materia(&pool, id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}