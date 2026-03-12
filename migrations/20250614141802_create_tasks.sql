-- Add migration script here
PRAGMA foreign_keys = ON;


-- Tasks
CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER NOT NULL DEFAULT 1,
  parent_id INTEGER,
  title TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL,
  position INTEGER NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE,
  FOREIGN KEY(parent_id) REFERENCES tasks(id) ON DELETE CASCADE
);


-- Indexes
CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_tasks_parent ON tasks(parent_id);
CREATE INDEX IF NOT EXISTS idx_tasks_sibling_order ON tasks(project_id, parent_id, position);


-- Enforce: parent and child must share the same project
CREATE TRIGGER IF NOT EXISTS prevent_cross_project_parent
BEFORE INSERT ON tasks
WHEN NEW.project_id IS NOT NULL
AND NEW.project_id != (
  SELECT project_id FROM tasks WHERE id = NEW.parent_id
)
BEGIN
  SELECT RAISE(ABORT, 'Parent task belongs to a different project');
END;


-- Position normalization on INSERT
CREATE TRIGGER IF NOT EXISTS normalize_position_insert
BEFORE INSERT ON tasks
BEGIN
  UPDATE tasks
  SET position = position + 1
  WHERE project_id = NEW.project_id
    AND IFNULL(parent_id, -1) = IFNULL(NEW.parent_id, -1)
    AND position >= NEW.position;
END;


-- Reorder siblings when position changes
CREATE TRIGGER IF NOT EXISTS reorder_position_update
BEFORE UPDATE OF position ON tasks
FOR EACH ROW
WHEN OLD.project_id = NEW.project_id
AND IFNULL(OLD.parent_id, -1) = IFNULL(NEW.parent_id, -1)
BEGIN
  -- moving down
  UPDATE tasks
  SET position = position - 1
  WHERE project_id = OLD.project_id
    AND IFNULL(OLD.parent_id, -1) = IFNULL(NEW.parent_id, -1)
    AND position > OLD.position
    AND position <= NEW.position;

  -- moving up
  UPDATE tasks
  SET position = position + 1
  WHERE project_id = OLD.project_id
    AND IFNULL(OLD.parent_id, -1) = IFNULL(NEW.parent_id, -1)
    AND position < OLD.position
    AND position >= NEW.position;
END;


-- Reorder when parent changes
CREATE TRIGGER IF NOT EXISTS reorder_on_parent_change
BEFORE UPDATE OF parent_id on tasks
FOR EACH ROW
BEGIN
  -- close gap in old parent
  UPDATE tasks
  SET position = position - 1
  WHERE project_id = OLD.project_id
    AND IFNULL(parent_id, -1) = IFNULL(OLD.parent_id, -1)
    AND position > OLD.position;

  -- open space in new parent
  UPDATE tasks
  SET position = position + 1
  WHERE project_id = NEW.project_id
    AND IFNULL(parent_id, -1) = IFNULL(NEW.parent_id, -1)
    AND position >= NEW.position;
END;


-- Cascade project change to all descendants
CREATE TRIGGER IF NOT EXISTS cascade_project_change
AFTER UPDATE OF project_id ON tasks
FOR EACH ROW
WHEN OLD.project_id != NEW.project_id
BEGIN
  UPDATE tasks
  SET project_id = NEW.project_id
  WHERE id IN (
    WITH RECURSIVE descendants AS (
      SELECT id FROM tasks WHERE parent_id = NEW.id
      UNION ALL
      SELECT t.id
      FROM tasks t
      JOIN descendants d ON t.parent_id = d.id
    )
    SELECT id FROM descendants
  );
END;


-- Cascade completed status to all descendants
CREATE TRIGGER IF NOT EXISTS cascade_task_completed
AFTER UPDATE OF status ON tasks
FOR EACH ROW
WHEN OLD.status != 'completed' AND NEW.status = 'completed'
BEGIN
  UPDATE tasks
  SET status = 'completed'
  WHERE ID IN (
    WITH RECURSIVE descendants AS (
      SELECT id FROM tasks WHERE parent_id = NEW.id
      UNION ALL
      SELECT t.id
      FROM tasks t
      JOIN descendants d ON t.parent_id = d.id
    )
    SELECT id FROM descendants
    )
  AND status != 'completed';
END;


-- Prevent open tasks under completed parent
CREATE TRIGGER IF NOT EXISTS prevent_open_under_completed
BEFORE UPDATE OF status ON tasks
FOR EACH ROW
WHEN NEW.status != 'completed'
AND NEW.parent_id IS NOT NULL
AND (SELECT status FROM tasks WHERE id = NEW.parent_id)
BEGIN
  SELECT RAISE(abort, 'Cannot reopen task under a compoleted parent');
END;


