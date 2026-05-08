use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use crate::models::materias_model::CreateMateriaDto;
use crate::services::materias_services;

pub async fn obtener_materias(State(db): State<PgPool>) -> impl IntoResponse {
    match materias_services::obtener_todas(&db).await {
        Ok(materias) => (StatusCode::OK, Json(materias)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
    }
}

pub async fn crear_materia(
    State(db): State<PgPool>,
    Json(payload): Json<CreateMateriaDto>,
) -> impl IntoResponse {
    match materias_services::crear_nueva(&db, payload).await {
        Ok(materia) => (StatusCode::CREATED, Json(materia)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}