use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize,FromRow)]
pub struct Profesor{
    id_profesor:i32,
    nombre:String,
    especialidad:String,
    correo:String,
}