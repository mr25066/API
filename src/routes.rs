use axum::{
    routing::{get, post},
    Router,
};

use sqlx::PgPool;

use crate::controllers::profesor_comtroller::obtener_Profesores;
use crate::controllers::materias_controller::{obtener_materias, crear_materia};

pub fn crear_rutas(
    db: PgPool,
) -> Router {

    Router::new()
        .route(
            "/profesores",
            get(obtener_Profesores),
        )
        .route(
            "/materias",
            get(obtener_materias).post(crear_materia),
        )
        .with_state(db)
}