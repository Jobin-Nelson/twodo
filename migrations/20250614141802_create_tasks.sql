-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  done INTEGER NOT NULL DEFAULT false
) STRICT;
