use std::fs;
use tauri::{AppHandle, Manager};
use crate::containers::Container;
use sqlx::sqlite::SqlitePool;
use tauri::State;

pub struct DbState {
    pub pool : SqlitePool
}

pub async fn db_init(app_handle: &AppHandle) -> Result<DbState, Box<dyn std::error::Error>>{
    
    let app_dir = app_handle.path().app_data_dir()?;

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }
    
    let mut db_path = app_dir.clone();
    db_path.push("blinkers.db");
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());

    if !db_path.exists() {
        fs::File::create(&db_path)?;
    }

    // 3. Connect to the SQLite instance (creates file if missing)
    let pool = SqlitePool::connect(&db_url).await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS containers (
            id TEXT PRIMARY KEY NOT NULL,
            container_type TEXT NOT NULL,
            quality INTEGER,
            notes TEXT,
            is_active INTEGER NOT NULL
        );",
    ) 
    .execute(&pool)
    .await?;
    
    Ok(DbState { pool }) 
}


pub async fn insert_container(db_state: State<'_, DbState>, container: &Container) -> Result<(), String>{ 
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
    .execute(&db_state.pool)
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
