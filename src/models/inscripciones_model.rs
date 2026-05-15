use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow,PartialEq)]
pub struct Inscripcion {
    pub id_inscripcion: i32,
    pub id_estudiante: Option<i32>,
    pub id_materia: Option<i32>,
    pub ciclo: String,
    pub nota_final: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct CrearInscripcion {
    pub id_estudiante: Option<i32>,
    pub id_materia: Option<i32>,
    pub ciclo: String,
    pub nota_final: Option<f64>,
}