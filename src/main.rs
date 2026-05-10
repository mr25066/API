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
    pub mod carreras_model;
    pub mod inscripciones_model;
}
mod controllers{
    pub mod profesor_comtroller;
    pub mod materias_controller;
    pub mod carreras_controller;
    pub mod inscripciones_controller;
}
mod services{
    pub mod profesor_services;
    pub mod materias_services;
    pub mod carreras_services;
    pub mod inscripciones_services;
}

mod repository{
    pub mod materias_repository;
    pub mod carreras_repository;
    pub mod inscripciones_repository;
}
pub mod routes;

#[tokio::main]
async fn main() {

    let db = PgPool::connect(
        //"postgresql://postgres.vdokkavvfsctthvhbvlb:proyecto123@aws-1-us-east-1.pooler.supabase.com:6543/postgres?sslmode=require&pgbouncer=true"
       "postgresql://postgres.xunyklqfgevolgprpmpb:e5wT9nlrBqToWYig@aws-1-us-west-2.pooler.supabase.com:5432/postgres?sslmode=require"
       // "postgresql://postgres.rwxsbbflnarrfxjcxvpw:Diosmeama1516@@aws-1-us-east-2.pooler.supabase.com:6543/postgres"

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