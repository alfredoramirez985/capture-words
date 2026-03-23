CREATE TABLE translations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    phrase_id INTEGER NOT NULL,
    language_code TEXT NOT NULL,
    translated_text TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(phrase_id) REFERENCES phrases(id) ON DELETE CASCADE
);
