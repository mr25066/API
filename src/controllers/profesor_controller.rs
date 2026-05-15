use axum::{
    extract::{State,Path},
    Json,
};
use axum::http::StatusCode;
use sqlx::PgPool;
use axum::response::IntoResponse;
use crate::models::profesor_model::{Crearprofesor,ActualizarProfesor};
use crate::services::profesor_services::{self};


pub async fn obtener_profesor(State(db): State<PgPool>) -> impl IntoResponse {
   match profesor_services::obtener_profesores_service(&db).await {
    Ok(profesor) => {
        println!("{:?}", profesor);

        (StatusCode::OK, Json(profesor)).into_response()
    }

    Err(err) => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(err)
    ).into_response(),
}
}
pub async fn crear_profesor(
    State(db): State<PgPool>,
    Json(payload): Json<Crearprofesor>,
) -> impl IntoResponse {
    match profesor_services::crear_nueva(&db, payload).await {
        Ok(profesor) => (StatusCode::CREATED, Json(profesor)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

pub async fn actualizar_profesor(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
    Json(payload): Json<ActualizarProfesor>,
) -> impl IntoResponse {

    match profesor_services::actualizar_profesor_service(&db, id, payload).await {
        Ok(profesor) => (
            StatusCode::OK,
            Json(profesor),
        ).into_response(),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(err),
        ).into_response(),
    }
}

pub async fn eliminar_profesor(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
) -> impl IntoResponse {

    match profesor_services::eliminar_profesor_service(&db, id).await {

        Ok(_) => (
            StatusCode::OK,
            "Profesor eliminado"
        ).into_response(),

        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            err
        ).into_response(),
    }
}

pub async fn obtener_profesor_por_id(
    Path(id): Path<i32>,
    State(db): State<PgPool>,
) -> impl IntoResponse {

    match profesor_services::obtener_por_id_service(&db, id).await {

        Ok(profesor) => (
            StatusCode::OK,
            Json(profesor),
        ).into_response(),

        Err(err) => (
            StatusCode::NOT_FOUND,
            Json(err),
        ).into_response(),
    }
}