use sqlx::PgPool;
use crate::models::carreras_model::Carreras; 
pub async fn crear_carrera(
    pool: &PgPool,
    nombre: &str,
    facultad: &str,
) -> Result<Carreras, sqlx::Error> {
    sqlx::query_as::<_, Carreras>(
        r#"
        INSERT INTO Carreras (nombre_carrera, facultad)
        VALUES ($1, $2)
        RETURNING id_carrera, nombre_carrera, facultad
        "#
    )
        .bind(nombre)
        .bind(facultad)
        .fetch_one(pool)
        .await
}
pub async fn listar_carreras(
    pool: &PgPool,
) -> Result<Vec<Carreras>, sqlx::Error> {
    sqlx::query_as::<_, Carreras>(
        "SELECT id_carrera, nombre_carrera, facultad FROM Carreras"
    )
        .fetch_all(pool)
        .await
}

pub async fn obtener_carrera(
    pool: &PgPool,
    id: i32,
) -> Result<Carreras, sqlx::Error> {
    sqlx::query_as::<_, Carreras>(
        "SELECT id_carrera, nombre_carrera, facultad FROM Carreras WHERE id_carrera = $1"
    )
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn actualizar_carrera(
    pool: &PgPool,
    id: i32,
    nombre: &str,
    facultad: &str,
) -> Result<Carreras, sqlx::Error> {
    sqlx::query_as::<_, Carreras>(
        r#"
        UPDATE Carreras
        SET nombre_carrera = $1,
            facultad = $2
        WHERE id_carrera = $3
        RETURNING id_carrera, nombre_carrera, facultad
        "#
    )
        .bind(nombre)
        .bind(facultad)
        .bind(id)
        .fetch_one(pool)
        .await
}
pub async fn eliminar_carrera(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM Carreras WHERE id_carrera = $1"
    )
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected()) 
}