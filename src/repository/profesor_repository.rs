use crate::models::profesor_model::{self, Crearprofesor, Profesor,ActualizarProfesor};
use sqlx::PgPool;

pub async fn obtener_Profesores(pool: &PgPool) -> Result<Vec<Profesor>, sqlx::Error> {
    let profesores = sqlx::query_as::<_, Profesor>("SELECT * FROM profesores")
        .fetch_all(pool)
        .await?;

    Ok(profesores)
}

pub async fn create(pool: &PgPool, data: Crearprofesor) -> Result<Profesor, sqlx::Error> {
    let profesor = sqlx::query_as::<_, Profesor>(
        "INSERT INTO profesores (nombre, especialidad, correo)
VALUES ($1, $2, $3)
RETURNING id_profesor, nombre, especialidad, correo",
    )
    .bind(data.nombre)
    .bind(data.especialidad)
    .bind(data.correo)
    .fetch_one(pool)
    .await?;

    Ok(profesor)
}

pub async fn actualizar_profesor(
    pool: &PgPool,
    id: i32,
    data: ActualizarProfesor,
) -> Result<Profesor, sqlx::Error> {
    let profesor = sqlx::query_as::<_, Profesor>(
        "UPDATE profesores
         SET nombre = $1,
             especialidad = $2,
             correo = $3
         WHERE id_profesor = $4
         RETURNING id_profesor, nombre, especialidad, correo",
    )
    .bind(data.nombre)
    .bind(data.especialidad)
    .bind(data.correo)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(profesor)
}

pub async fn eliminar_profesor(
    pool: &PgPool,
    id: i32,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        "DELETE FROM profesores WHERE id_profesor = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn obtener_por_id(
    pool: &PgPool,
    id: i32,
) -> Result<Profesor, sqlx::Error> {

    let profesor = sqlx::query_as::<_, Profesor>(
        "SELECT * FROM profesores WHERE id_profesor = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(profesor)
}
