use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, FromRow)]
pub struct Estudiante {
    pub id_estudiante: i32,
    pub nombre: String,
    pub apellido: String,
    pub carnet: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_carrera: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrearEstudiante {
    pub nombre: String,
    pub apellido: String,
    pub carnet: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_carrera: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActualizarEstudiante {
    pub nombre: String,
    pub apellido: String,
    pub carnet: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_carrera: Option<i32>,
}