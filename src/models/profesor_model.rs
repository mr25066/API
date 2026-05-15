use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize,FromRow,Debug)]
pub struct Profesor{
    id_profesor:i32,
    nombre:String,
    especialidad:String,
    correo:String,
}

#[derive(Deserialize,Debug)]
pub struct Crearprofesor{
   pub nombre:String,
   pub especialidad:String,
   pub correo:String,
}

#[derive(Deserialize, Debug)]
pub struct ActualizarProfesor {
    pub nombre: String,
    pub especialidad: String,
    pub correo: String,
}