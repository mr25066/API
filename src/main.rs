use axum::{
    routing::get,
    Json,
    Router,
    extract::State,
};

use serde::Serialize;
use sqlx::{FromRow, PgPool};
mod models{
    pub mod profesor_model;
}
mod controllers{
    pub mod profesor_comtroller;
}
mod services{
    pub mod profesor_services;
}
mod routes;

#[tokio::main]
async fn main() {

    let db = PgPool::connect(
        "postgresql://postgres.vdokkavvfsctthvhbvlb:proyecto123@aws-1-us-east-1.pooler.supabase.com:6543/postgres?sslmode=require&pgbouncer=true"
    )
    .await
    .unwrap();

    println!("Base de datos conectada");

    let app = routes::crear_rutas(db);
        

    let mostrar = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Servidor en puerto 3000");

    axum::serve(mostrar, app)
        .await
        .unwrap();
}