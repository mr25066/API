use sqlx::PgPool;
use crate::models::materias_model::Materia;

pub async fn crear_materia(
    pool: &PgPool,
    nombre: &str,
    uv: i32,
    id_profesor: Option<i32>,
) -> Result<Materia, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        r#"
        INSERT INTO Materias (nombre_materia, uv, id_profesor)
        VALUES ($1, $2, $3)
        RETURNING id_materia, nombre_materia, uv, id_profesor
        "#
    )
        .bind(nombre)
        .bind(uv)
        .bind(id_profesor)
        .fetch_one(pool)
        .await
}

pub async fn listar_materias(
    pool: &PgPool,
) -> Result<Vec<Materia>, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        "SELECT id_materia, nombre_materia, uv, id_profesor FROM Materias"
    )
        .fetch_all(pool)
        .await
}

pub async fn obtener_materia(
    pool: &PgPool,
    id: i32,
) -> Result<Materia, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        "SELECT id_materia, nombre_materia, uv, id_profesor FROM Materias WHERE id_materia = $1"
    )
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn actualizar_materia(
    pool: &PgPool,
    id: i32,
    nombre: &str,
    uv: i32,
    id_profesor: Option<i32>,
) -> Result<Materia, sqlx::Error> {
    sqlx::query_as::<_, Materia>(
        r#"
        UPDATE Materias
        SET nombre_materia = $1,
            uv = $2,
            id_profesor = $3
        WHERE id_materia = $4
        RETURNING id_materia, nombre_materia, uv, id_profesor
        "#
    )
        .bind(nombre)
        .bind(uv)
        .bind(id_profesor)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn eliminar_materia(
    pool: &PgPool,
    id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM Materias WHERE id_materia = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}