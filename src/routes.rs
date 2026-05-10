use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::controllers::carreras_controller::{
    crear_carrera, listar_carreras, obtener_carrera, actualizar_carrera, eliminar_carrera,
};
use crate::controllers::inscripciones_controller::{actualizar_inscripcion, crear_inscripcion, eliminar_inscripcion, listar_inscripciones, obtener_inscripcion};
use crate::controllers::profesor_comtroller::obtener_Profesores;
use crate::controllers::materias_controller::{
    listar_materias, crear_materia, obtener_materia, actualizar_materia,eliminar_materia};


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
            get(listar_materias)
                .post(crear_materia))
        .route(
            "/materias/{id}",
            get(obtener_materia)
                .put(actualizar_materia)
                .delete(eliminar_materia))
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
        ).route(
        "/inscripciones",
        get(listar_inscripciones)
            .post(crear_inscripcion)
    ).route(
        "/inscripciones/{id}",
        get(obtener_inscripcion)
            .put(actualizar_inscripcion)
            .delete(eliminar_inscripcion),
    )
        .with_state(db)
}