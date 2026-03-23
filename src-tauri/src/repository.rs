use sqlx::{SqlitePool, FromRow};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Session {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub init_page: Option<i64>,
    pub finish_page: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
    pub is_favorite: bool,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Translation {
    pub id: i64,
    pub phrase_id: i64,
    pub language_code: String,
    pub translated_text: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhraseWithTranslations {
    #[serde(flatten)]
    pub phrase: Phrase,
    pub translations: Vec<Translation>,
}

#[derive(Serialize, Deserialize, Debug, FromRow, Clone)]
pub struct Phrase {
    pub id: i64,
    pub session_id: i64,
    pub phrase: String,
    pub created_at: String,
}

pub async fn create_session(
    pool: &SqlitePool, 
    name: &str, 
    description: Option<&str>, 
    init_page: Option<i64>, 
    finish_page: Option<i64>
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO sessions (name, description, init_page, finish_page) VALUES (?, ?, ?, ?)"
    )
    .bind(name)
    .bind(description)
    .bind(init_page)
    .bind(finish_page)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_sessions(pool: &SqlitePool) -> Result<Vec<Session>, sqlx::Error> {
    let sessions = sqlx::query_as::<_, Session>(
        "SELECT id, name, description, init_page, finish_page, created_at, updated_at, is_favorite FROM sessions ORDER BY is_favorite DESC, updated_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}

pub async fn toggle_favorite_session(pool: &SqlitePool, session_id: i64, is_favorite: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET is_favorite = ? WHERE id = ?")
        .bind(is_favorite)
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn save_phrase(pool: &SqlitePool, session_id: i64, phrase: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO phrases (session_id, phrase) VALUES (?, ?)"
    )
    .bind(session_id)
    .bind(phrase)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_phrases_with_translations(pool: &SqlitePool, session_id: i64) -> Result<Vec<PhraseWithTranslations>, sqlx::Error> {
    let phrases = sqlx::query_as::<_, Phrase>(
        "SELECT id, session_id, phrase, created_at FROM phrases WHERE session_id = ? ORDER BY created_at DESC"
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for phrase in phrases {
        let translations = sqlx::query_as::<_, Translation>(
            "SELECT id, phrase_id, language_code, translated_text, created_at FROM translations WHERE phrase_id = ?"
        )
        .bind(phrase.id)
        .fetch_all(pool)
        .await?;

        result.push(PhraseWithTranslations {
            phrase,
            translations,
        });
    }

    Ok(result)
}

pub async fn save_translation(pool: &SqlitePool, phrase_id: i64, language_code: &str, translated_text: &str) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO translations (phrase_id, language_code, translated_text) VALUES (?, ?, ?)"
    )
    .bind(phrase_id)
    .bind(language_code)
    .bind(translated_text)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
