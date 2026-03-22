-- Create the new sessions table
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    init_page INTEGER,
    finish_page INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert a default session so existing phrases have a home
INSERT INTO sessions (id, name, description) 
VALUES (1, 'Default Session', 'Auto-created for existing phrases');

-- Recreate phrases table with the new foreign key
CREATE TABLE phrases_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    phrase TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Migrate old data into the new table, assigning it to session 1
INSERT INTO phrases_new (id, session_id, phrase, created_at)
SELECT id, 1, phrase, created_at FROM phrases;

-- Drop old table and rename the new one
DROP TABLE phrases;
ALTER TABLE phrases_new RENAME TO phrases;
