use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;

use crate::models::estudiantes_model::{ActualizarEstudiante, CrearEstudiante};
use crate::services::estudiantes_services;

pub async fn obtener_estudiantes(
    State(db): State<PgPool>,
) -> impl IntoResponse {
    match estudiantes_services::obtener_estudiantes_service(&db).await {
        Ok(estudiantes) => {
            (StatusCode::OK, Json(estudiantes)).into_response()
        }

        Err(err) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err),
            ).into_response()
        }
    }
}

pub async fn crear_estudiante(
    State(db): State<PgPool>,
    Json(payload): Json<CrearEstudiante>,
) -> impl IntoResponse {
    match estudiantes_services::crear_estudiante_service(&db, payload).await {
        Ok(estudiante) => {
            (
                StatusCode::CREATED,
                Json(estudiante),
            ).into_response()
        }

        Err(err) => {
            (
                StatusCode::BAD_REQUEST,
                Json(err),
            ).into_response()
        }
    }
}

pub async fn obtener_estudiante_por_id(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
) -> impl IntoResponse {
    match estudiantes_services::obtener_estudiante_por_id_service(&db, id).await {
        Ok(estudiante) => {
            (
                StatusCode::OK,
                Json(estudiante),
            ).into_response()
        }

        Err(err) => {
            (
                StatusCode::NOT_FOUND,
                Json(err),
            ).into_response()
        }
    }
}

pub async fn actualizar_estudiante(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
    Json(payload): Json<ActualizarEstudiante>,
) -> impl IntoResponse {
    match estudiantes_services::actualizar_estudiante_service(&db, id, payload).await {
        Ok(estudiante) => {
            (
                StatusCode::OK,
                Json(estudiante),
            ).into_response()
        }

        Err(err) => {
            (
                StatusCode::BAD_REQUEST,
                Json(err),
            ).into_response()
        }
    }
}

pub async fn eliminar_estudiante(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
) -> impl IntoResponse {
    match estudiantes_services::eliminar_estudiante_service(&db, id).await {
        Ok(_) => {
            (
                StatusCode::OK,
                "Estudiante eliminado",
            ).into_response()
        }

        Err(err) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(err),
            ).into_response()
        }
    }
}
