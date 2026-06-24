use tauri_plugin_sql::{Migration, MigrationKind};

mod containers;
pub mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![Migration {
        version: 1,
        description: "containers_table",
        sql: "CREATE TABLE IF NOT EXISTS containers (
            id TEXT PRIMARY KEY NOT NULL,
            container_type TEXT NOT NULL,
            quality INTEGER,
            notes TEXT,
            is_active INTEGER NOT NULL
        );",
        kind: MigrationKind::Up, 
    }];
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default()
            .add_migrations("sqlite:blinkers.db", migrations)
            .build()
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![containers::create_container])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
