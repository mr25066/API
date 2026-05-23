use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, FromRow)]
pub struct Carreras {
    pub id_carrera: i32,
    pub nombre_carrera: String,
    pub facultad: String,
}