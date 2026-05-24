use sqlx::PgPool;

use crate::models::estudiantes_model::{ActualizarEstudiante, CrearEstudiante, Estudiante};

pub async fn crear_estudiante(
    pool: &PgPool,
    data: CrearEstudiante,
) -> Result<Estudiante, sqlx::Error> {
    sqlx::query_as::<_, Estudiante>(
        r#"
        INSERT INTO Estudiantes (
            nombre,
            apellido,
            carnet,
            fecha_nacimiento,
            id_carrera
        )
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            id_estudiante,
            nombre,
            apellido,
            carnet,
            fecha_nacimiento,
            id_carrera
        "#
    )
        .bind(data.nombre)
        .bind(data.apellido)
        .bind(data.carnet)
        .bind(data.fecha_nacimiento)
        .bind(data.id_carrera)
        .fetch_one(pool)
        .await
}

pub async fn obtener_estudiantes(
    pool: &PgPool,
) -> Result<Vec<Estudiante>, sqlx::Error> {
    sqlx::query_as::<_, Estudiante>(
        r#"
        SELECT
            id_estudiante,
            nombre,
            apellido,
            carnet,
            fecha_nacimiento,
            id_carrera
        FROM Estudiantes
        ORDER BY id_estudiante
        "#
    )
        .fetch_all(pool)
        .await
}

pub async fn obtener_estudiante_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Estudiante, sqlx::Error> {
    sqlx::query_as::<_, Estudiante>(
        r#"
        SELECT
            id_estudiante,
            nombre,
            apellido,
            carnet,
            fecha_nacimiento,
            id_carrera
        FROM Estudiantes
        WHERE id_estudiante = $1
        "#
    )
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn actualizar_estudiante(
    pool: &PgPool,
    id: i32,
    data: ActualizarEstudiante,
) -> Result<Estudiante, sqlx::Error> {
    sqlx::query_as::<_, Estudiante>(
        r#"
        UPDATE Estudiantes
        SET nombre = $1,
            apellido = $2,
            carnet = $3,
            fecha_nacimiento = $4,
            id_carrera = $5
        WHERE id_estudiante = $6
        RETURNING
            id_estudiante,
            nombre,
            apellido,
            carnet,
            fecha_nacimiento,
            id_carrera
        "#
    )
        .bind(data.nombre)
        .bind(data.apellido)
        .bind(data.carnet)
        .bind(data.fecha_nacimiento)
        .bind(data.id_carrera)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn eliminar_estudiante(
    pool: &PgPool,
    id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM Estudiantes
        WHERE id_estudiante = $1
        "#
    )
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
