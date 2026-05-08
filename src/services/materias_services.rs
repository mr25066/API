use sqlx::PgPool;
use crate::models::materias_model::{Materia, CreateMateriaDto};
use crate::repository::materias_repository;

pub async fn obtener_todas(pool: &PgPool) -> Result<Vec<Materia>, String> {
    materias_repository::find_all(pool)
        .await
        .map_err(|e| format!("Error en BD: {}", e))
}

pub async fn crear_nueva(pool: &PgPool, data: CreateMateriaDto) -> Result<Materia, String> {
    if data.uv <= 0 {
        return Err("Las UV deben ser mayores a 0".to_string());
    }
    materias_repository::create(pool, data)
        .await
        .map_err(|e| format!("Error al crear: {}", e))
}