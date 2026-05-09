use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::controllers::carreras_controller::{
    crear_carrera, listar_carreras, obtener_carrera, actualizar_carrera, eliminar_carrera,
};
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
        .route(
            "/carreras",
            post(crear_carrera).get(listar_carreras)
        )
        .route(
            "/carreras/{id}",
            get(obtener_carrera)
        )
        .route( "/carreras/actualizar/{id}", put(actualizar_carrera)
        )
        .route( "/carreras/eliminar/{id}", delete(eliminar_carrera)
        )
        .with_state(db)
}