use axum::{
    routing::get,
    Router,
};

use sqlx::PgPool;

use crate::controllers::profesor_comtroller::obtener_Profesores;

pub fn crear_rutas(
    db: PgPool,
) -> Router {

    Router::new()
        .route(
            "/profesores",
            get(obtener_Profesores),
        )
        .with_state(db)
}