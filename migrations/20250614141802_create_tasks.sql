-- Add migration script here
CREATE TABLE IF NOT EXISTS projects (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  done INTEGER NOT NULL DEFAULT false,
  project_id INTEGER NOT NULL,
  FOREIGN KEY(project_id) REFERENCES projects(id)
) STRICT;

-- Insert INBOX project for default tasks
INSERT INTO projects (id, name) VALUES (1, 'INBOX');
