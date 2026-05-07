use sqlx::PgPool;

use crate::models::profesor_model::Profesor;

pub async fn obtener_Profesores_service(
    db: &PgPool,
) -> Vec<Profesor> {
let profesores = sqlx::query_as::<_, Profesor>(
    "SELECT * FROM profesores"
)
.persistent(false)
.fetch_all(db)
.await
.unwrap();
   
   profesores
}