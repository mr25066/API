use axum::{
    extract::State,
    Json,
};
use sqlx::PgPool;

use crate::models::profesor_model::Profesor;
use crate::services::profesor_services::obtener_Profesores_service;

pub async fn obtener_Profesores(
    State(db): State<PgPool>,
) -> Json<Vec<Profesor>> {

    let materias =
        obtener_Profesores_service(&db).await;

    Json(materias)
}