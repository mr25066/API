use sqlx::PgPool;
use crate::models::inscripciones_model::Inscripcion;

pub async fn crear_inscripcion(
    pool: &PgPool,
    id_estudiante: Option<i32>,
    id_materia: Option<i32>,
    ciclo: &str,
    nota_final: Option<f64>,
) -> Result<Inscripcion, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        r#"
        INSERT INTO Inscripciones (id_estudiante, id_materia, ciclo, nota_final)
        VALUES ($1, $2, $3, $4)
        RETURNING id_inscripcion, id_estudiante, id_materia, ciclo, nota_final
        "#
    )
        .bind(id_estudiante)
        .bind(id_materia)
        .bind(ciclo)
        .bind(nota_final)
        .fetch_one(pool)
        .await
}

pub async fn listar_inscripciones(
    pool: &PgPool,
) -> Result<Vec<Inscripcion>, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        r#"
        SELECT id_inscripcion, id_estudiante, id_materia, ciclo, nota_final
        FROM Inscripciones
        ORDER BY id_inscripcion
        "#
    )
    .persistent(false)
        .fetch_all(pool)
        .await
}

pub async fn obtener_inscripcion(
    pool: &PgPool,
    id: i32,
) -> Result<Inscripcion, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        r#"
        SELECT id_inscripcion, id_estudiante, id_materia, ciclo, nota_final
        FROM Inscripciones
        WHERE id_inscripcion = $1
        "#
    )
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn actualizar_inscripcion(
    pool: &PgPool,
    id: i32,
    id_estudiante: Option<i32>,
    id_materia: Option<i32>,
    ciclo: &str,
    nota_final: Option<f64>,
) -> Result<Inscripcion, sqlx::Error> {
    sqlx::query_as::<_, Inscripcion>(
        r#"
        UPDATE Inscripciones
        SET id_estudiante = $1,
            id_materia = $2,
            ciclo = $3,
            nota_final = $4
        WHERE id_inscripcion = $5
        RETURNING id_inscripcion, id_estudiante, id_materia, ciclo, nota_final
        "#
    )
        .bind(id_estudiante)
        .bind(id_materia)
        .bind(ciclo)
        .bind(nota_final)
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn eliminar_inscripcion(
    pool: &PgPool,
    id: i32,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM Inscripciones
        WHERE id_inscripcion = $1
        "#
    )
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}