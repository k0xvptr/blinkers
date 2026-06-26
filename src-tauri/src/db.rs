use crate::containers::FocusBlock;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions, SqlitePool};
use std::fs;
use tauri::State;
use tauri::{AppHandle, Manager};
use std::str::FromStr;
use std::time::Duration;

pub struct DbState {
    pub pool: SqlitePool,
}

pub async fn db_init(app_handle: &AppHandle) -> Result<DbState, Box<dyn std::error::Error>> {
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

    let connection_options = SqliteConnectOptions::from_str(&db_url)?
    .create_if_missing(true)
    .busy_timeout(Duration::from_secs(5));

    // 3. Connect to the SQLite instance (creates file if missing)
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connection_options)
        .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS focus_blocks (
            id TEXT PRIMARY KEY NOT NULL,
            is_completed INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS containers (
            id TEXT PRIMARY KEY NOT NULL,
            focus_block_id TEXT NOT NULL,
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

pub async fn insert_container(
    db_state: State<'_, DbState>,
    focus_block: &FocusBlock,
) -> Result<(), String> {
    let mut tx = db_state.pool.begin().await.map_err(|e| e.to_string())?;

    // 2. Insert the parent FocusBlock metadata
    sqlx::query("INSERT INTO focus_blocks (id, is_completed) VALUES ($1, $2)")
        .bind(&focus_block.id)
        .bind(focus_block.is_completed as i32)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    let containers = [
        &focus_block.setup_container,
        &focus_block.work_container,
        &focus_block.break_container,
    ];

    for container in containers {
        let serialized_type = serde_json::to_string(&container.container_type)
            .map_err(|e| e.to_string())?;
        sqlx::query(
            "INSERT INTO containers (id, focus_block_id, container_type, quality, notes)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&container.id)
        .bind(&focus_block.id)
        .bind(serialized_type)
        .bind(container.quality)
        .bind(&container.notes)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
