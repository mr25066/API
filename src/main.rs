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
    pub mod materias_model;
}
mod controllers{
    pub mod profesor_comtroller;
    pub mod materias_controller;
}
mod services{
    pub mod profesor_services;
    pub mod materias_services;
}

mod repository{
    pub mod materias_repository;
}
mod routes;

#[tokio::main]
async fn main() {

    let db = PgPool::connect(
        //"postgresql://postgres.vdokkavvfsctthvhbvlb:proyecto123@aws-1-us-east-1.pooler.supabase.com:6543/postgres?sslmode=require&pgbouncer=true"
        "postgresql://postgres.xunyklqfgevolgprpmpb:e5wT9nlrBqToWYig@aws-1-us-west-2.pooler.supabase.com:5432/postgres?sslmode=require"
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