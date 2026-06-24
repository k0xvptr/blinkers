use tauri::{AppHandle, Manager};
use crate::containers::Container;
use tauri_plugin_sql::SqlitePool;


pub async fn insert_container(app_handle: &AppHandle, container: &Container) -> Result<(), String>{
    let pool = app_handle.state::<SqlitePool>();
    let serialized_type = serde_json::to_string(&container.container_type)
        .map_err(|e| e.to_string())?;

    sqlx::query(
        "INSERT INTO containers (id, container_type, quality, notes, is_active)
        VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&container.id)
    .bind(serialized_type)
    .bind(container.quality)
    .bind(&container.notes)
    .bind(container.is_active as i32)
    .execute(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
