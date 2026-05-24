#![allow(dead_code)]
use sqlx::{postgres::PgPoolOptions};
use dotenvy::dotenv; // <-- Agregamos esta importación
use std::env;
mod models {
    pub mod carreras_model;
    pub mod inscripciones_model;
    pub mod materias_model;
    pub mod profesor_model;
    pub mod estudiantes_model;
}
mod controllers {
    pub mod carreras_controller;
    pub mod inscripciones_controller;
    pub mod materias_controller;
    pub mod profesor_controller;
    pub mod estudiantes_controller;
}
mod services {
    pub mod carreras_services;
    pub mod inscripciones_services;
    pub mod materias_services;
    pub mod profesor_services;
    pub mod estudiantes_services;
}

mod repository {
    pub mod carreras_repository;
    pub mod inscripciones_repository;
    pub mod materias_repository;
    pub mod profesor_repository;
    pub mod estudiantes_repository;
}
pub mod routes;

#[tokio::main]
async fn main() {
    // 1. Cargar las variables del archivo .env
    dotenv().ok();

    // 2. Leer la variable DATABASE_URL
    let database_url = env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL no está configurada en el archivo .env");

    // 3. Conectar usando la variable
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url) // <-- Pasamos la variable aquí
        .await
        .unwrap();
        
    println!("Base de datos conectada");

    let app = routes::crear_rutas(db);

    let mostrar = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor en puerto 3000");

    axum::serve(mostrar, app).await.unwrap();
}