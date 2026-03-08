-- Add migration script here
CREATE TABLE IF NOT EXISTS projects (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert INBOX project for default tasks
INSERT INTO projects (id, name) VALUES (1, 'INBOX');
