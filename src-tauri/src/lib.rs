mod db;
mod repository;

use sqlx::SqlitePool;
use tauri::Manager;
use tauri::Emitter;
use reqwest::Client;
use serde_json::json;
use dotenvy::dotenv;
use std::fs;
use uuid::Uuid;

use log::{error, info};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn create_session(name: String, description: Option<String>, init_page: Option<i64>, finish_page: Option<i64>, pool: tauri::State<'_, SqlitePool>) -> Result<i64, String> {
    info!("Creating session: {}", name);
    repository::create_session(&pool, &name, description.as_deref(), init_page, finish_page).await.map_err(|e| {
        error!("Error creating session: {}", e);
        e.to_string()
    })
}

#[tauri::command]
async fn get_sessions(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<repository::Session>, String> {
    repository::get_sessions(&pool).await.map_err(|e| {
        error!("Error getting sessions: {}", e);
        e.to_string()
    })
}

#[tauri::command]
fn get_user_guid(guid: tauri::State<'_, String>) -> String {
    guid.inner().clone()
}

#[tauri::command]
async fn toggle_favorite(session_id: i64, is_favorite: bool, pool: tauri::State<'_, SqlitePool>) -> Result<(), String> {
    info!("Toggling favorite for session {} to {}", session_id, is_favorite);
    repository::toggle_favorite_session(&pool, session_id, is_favorite).await.map_err(|e| {
        error!("Error toggling favorite: {}", e);
        e.to_string()
    })
}

#[tauri::command]
async fn save_phrase(session_id: i64, phrase: String, source_lang: String, target_lang: String, api_key: String, app: tauri::AppHandle, pool: tauri::State<'_, SqlitePool>) -> Result<i64, String> {
    info!("Saving phrase for session {}", session_id);
    let phrase_id = repository::save_phrase(&pool, session_id, &phrase).await.map_err(|e| {
        error!("Error saving phrase: {}", e);
        e.to_string()
    })?;

    let pool_clone = pool.inner().clone();
    let phrase_clone = phrase.clone();

    tauri::async_runtime::spawn(async move {
        if api_key.is_empty() || api_key == "your_gemini_api_key_here" {
            error!("Gemini API key not found or not set.");
            return;
        }

        let client = Client::new();

        let system_prompt = format!("You are a professional translator. Translate the following text from {} to {}. Reply ONLY with the translated text, no quotation marks and no extra conversation.", source_lang, target_lang);
        
        println!("====== GEMINI PROMPT ======\nSystem Instruction:\n{}\n\nUser Content:\n{}\n===========================", system_prompt, phrase_clone);

        let request_body = json!({
            "system_instruction": {
                "parts": [{ "text": system_prompt }]
            },
            "contents": [{
                "parts": [{ "text": phrase_clone }]
            }],
            "generationConfig": {
                "temperature": 0.3
            }
        });

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);

        match client.post(&url)
                .json(&request_body)
                .send()
                .await {
                Ok(response) => {
                    if let Ok(json_response) = response.json::<serde_json::Value>().await {
                        if let Some(candidates) = json_response.get("candidates").and_then(|c| c.as_array()) {
                            if let Some(first_candidate) = candidates.first() {
                                if let Some(content) = first_candidate.get("content").and_then(|c| c.get("parts")).and_then(|p| p.as_array()).and_then(|p| p.first()).and_then(|p| p.get("text")).and_then(|t| t.as_str()) {
                                    let translated_text = content.trim();
                                    if let Err(e) = repository::save_translation(&pool_clone, phrase_id, &target_lang, translated_text).await {
                                        error!("Failed to save translation {}: {}", target_lang, e);
                                    }
                                }
                            }
                        } else if json_response.get("error").is_some() {
                            error!("Gemini API error response: {}", json_response);
                        } else {
                            error!("Gemini unexpected response: {}", json_response);
                        }
                    }
                },
                Err(e) => {
                    error!("Gemini Request error: {}", e);
                }
            }

        app.emit("translations-ready", phrase_id).unwrap_or_else(|e| {
            error!("Failed to emit translations-ready event: {}", e);
        });
    });

    Ok(phrase_id)
}

#[tauri::command]
async fn get_phrases(session_id: i64, pool: tauri::State<'_, SqlitePool>) -> Result<Vec<repository::PhraseWithTranslations>, String> {
    repository::get_phrases_with_translations(&pool, session_id).await.map_err(|e| {
        error!("Error getting phrases: {}", e);
        e.to_string()
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let pool = tauri::async_runtime::block_on(async {
                db::init(app_data_dir).await.expect("Failed to initialize database")
            });
            app.manage(pool);

            if let Ok(home) = app.path().home_dir() {
                let guid_path = home.join(".capture-words-guid");
                let guid: String;
                if guid_path.exists() {
                    guid = fs::read_to_string(&guid_path).unwrap_or_default().trim().to_string();
                    info!("Loaded user GUID: {}", guid);
                } else {
                    guid = Uuid::new_v4().to_string();
                    if let Err(e) = fs::write(&guid_path, &guid) {
                        error!("Failed to write GUID to {:?}: {}", guid_path, e);
                    } else {
                        info!("Generated new user GUID: {}", guid);
                    }
                }
                app.manage(guid);
            } else {
                error!("Could not resolve home directory for GUID storage.");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_session, get_sessions, save_phrase, get_phrases, toggle_favorite, get_user_guid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
