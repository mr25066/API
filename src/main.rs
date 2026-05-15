use sqlx::{postgres::PgPoolOptions};
mod models {
    pub mod carreras_model;
    pub mod inscripciones_model;
    pub mod materias_model;
    pub mod profesor_model;
}
mod controllers {
    pub mod carreras_controller;
    pub mod inscripciones_controller;
    pub mod materias_controller;
    pub mod profesor_controller;
}
mod services {
    pub mod carreras_services;
    pub mod inscripciones_services;
    pub mod materias_services;
    pub mod profesor_services;
}

mod repository {
    pub mod carreras_repository;
    pub mod inscripciones_repository;
    pub mod materias_repository;
    pub mod profesor_repository;
}
pub mod routes;

#[tokio::main]
async fn main() {
    let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(
        "postgresql://postgres.vdokkavvfsctthvhbvlb:proyecto123@aws-1-us-east-1.pooler.supabase.com:6543/postgres?sslmode=require&pgbouncer=true"
    )
    .await
    .unwrap();
    println!("Base de datos conectada");

    let app = routes::crear_rutas(db);

    let mostrar = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor en puerto 3000");

    axum::serve(mostrar, app).await.unwrap();
}
