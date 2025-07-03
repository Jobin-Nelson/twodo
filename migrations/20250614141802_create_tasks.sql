-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  done INTEGER NOT NULL DEFAULT false,
  project_id INTEGER NOT NULL DEFAULT 1,
  FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
) STRICT;

