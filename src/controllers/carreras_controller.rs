use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::services::carreras_services;
use serde::Deserialize;
use crate::models::carreras_model::Carreras;

#[derive(Deserialize)]
pub struct CarrerasRequest {
    pub nombre_carrera: String,
    pub facultad: String,
}
pub async fn crear_carrera(
    State(pool): State<PgPool>,
    Json(body): Json<CarrerasRequest>,
) -> Json<Result<Carreras, String>> {
    let result = carreras_services::crear_carrera_service(&pool, &body.nombre_carrera, &body.facultad).await;
    match result {
        Ok(carrera) => Json(Ok(carrera)),
        Err(e) => Json(Err(format!("{:?}", e))),
    }
}
pub async fn listar_carreras(
    State(pool): State<PgPool>,
) -> Json<Result<Vec<Carreras>, String>> {
    let result = carreras_services::listar_carreras_service(&pool).await;

    match result {
        Ok(data) => Json(Ok(data)),
        Err(e) => Json(Err(format!("{:?}", e))),
    }
}
pub async fn obtener_carrera(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Result<Carreras, String>> {
    let result = carreras_services::obtener_carrera_service(&pool, id).await;
    match result {
        Ok(data) => Json(Ok(data)),
        Err(e) => Json(Err(format!("{:?}", e))),
    }
}
pub async fn actualizar_carrera(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(body): Json<CarrerasRequest>,
) -> Json<Result<Carreras, String>> {
    let result = carreras_services::actualizar_carrera_service(
        &pool,
        id,
        &body.nombre_carrera,
        &body.facultad,
    )
        .await;
    match result {
        Ok(data) => Json(Ok(data)),
        Err(e) => Json(Err(format!("{:?}", e))),
    }
}
pub async fn eliminar_carrera(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Result<&'static str, String>> {
    let result = carreras_services::eliminar_carrera_service(&pool, id).await;

    match result {
        Ok(_) => Json(Ok("Carrera eliminada")),
        Err(e) => Json(Err(format!("{:?}", e))),
    }
}