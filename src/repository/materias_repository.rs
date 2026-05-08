use sqlx::PgPool;
use crate::models::materias_model::{Materia, CreateMateriaDto};

pub async fn find_all(pool: &PgPool) -> Result<Vec<Materia>, sqlx::Error> {
    let materias = sqlx::query_as::<_, Materia>("SELECT id_materia, nombre_materia, uv, id_profesor FROM Materias")
        .fetch_all(pool)
        .await?;
    Ok(materias)
}

pub async fn create(pool: &PgPool, data: CreateMateriaDto) -> Result<Materia, sqlx::Error> {
    let materia = sqlx::query_as::<_, Materia>(
        "INSERT INTO Materias (nombre_materia, uv, id_profesor) VALUES ($1, $2, $3) RETURNING id_materia, nombre_materia, uv, id_profesor"
    )
        .bind(data.nombre_materia)
        .bind(data.uv)
        .bind(data.id_profesor)
        .fetch_one(pool)
        .await?;

    Ok(materia)
}