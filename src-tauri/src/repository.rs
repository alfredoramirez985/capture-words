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
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
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
        "SELECT id, name, description, init_page, finish_page, created_at, updated_at FROM sessions ORDER BY updated_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(sessions)
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

pub async fn get_phrases(pool: &SqlitePool, session_id: i64) -> Result<Vec<Phrase>, sqlx::Error> {
    let phrases = sqlx::query_as::<_, Phrase>(
        "SELECT id, session_id, phrase, created_at FROM phrases WHERE session_id = ? ORDER BY created_at DESC"
    )
    .bind(session_id)
    .fetch_all(pool)
    .await?;

    Ok(phrases)
}
