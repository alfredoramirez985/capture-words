mod db;
mod repository;

use sqlx::SqlitePool;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn create_session(name: String, description: Option<String>, init_page: Option<i64>, finish_page: Option<i64>, pool: tauri::State<'_, SqlitePool>) -> Result<i64, String> {
    repository::create_session(&pool, &name, description.as_deref(), init_page, finish_page).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_sessions(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<repository::Session>, String> {
    repository::get_sessions(&pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_phrase(session_id: i64, phrase: String, pool: tauri::State<'_, SqlitePool>) -> Result<i64, String> {
    repository::save_phrase(&pool, session_id, &phrase).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_phrases(session_id: i64, pool: tauri::State<'_, SqlitePool>) -> Result<Vec<repository::Phrase>, String> {
    repository::get_phrases(&pool, session_id).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let pool = tauri::async_runtime::block_on(async {
                db::init(app_data_dir).await.expect("Failed to initialize database")
            });
            app.manage(pool);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_session, get_sessions, save_phrase, get_phrases])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
