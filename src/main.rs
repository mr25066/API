use axum::{
    routing::get,
    Json,
    Router,
    extract::State,
};

use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
struct Especialidad {
    id_especialidad: i32,
    nombre_especialidad: String,
    descripcion: Option<String>,
}

async fn obtener_especialidades(
    State(db): State<PgPool>,
) -> Json<Vec<Especialidad>> {

    let especialidades = sqlx::query_as::<_, Especialidad>(
        "SELECT * FROM especialidades"
    )
    .fetch_all(&db)
    .await
    .unwrap();

    Json(especialidades)
}

#[tokio::main]
async fn main() {

    let db = PgPool::connect(
        "postgresql://postgres.qgytvnuccjsubqesmirq:proyecto1234@aws-1-us-east-1.pooler.supabase.com:6543/postgres"
    )
    .await
    .unwrap();

    println!("Base de datos conectada");

    let app = Router::new()
        .route(
            "/especialidades",
            get(obtener_especialidades),
        )
        .with_state(db);

    let mostrar = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Servidor en puerto 3000");

    axum::serve(mostrar, app)
        .await
        .unwrap();
}