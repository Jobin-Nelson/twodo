-- Add migration script here
PRAGMA foreign_keys = 1;
PRAGMA recursive_triggers = 1;

CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  done INTEGER NOT NULL DEFAULT false,
  project_id INTEGER NOT NULL DEFAULT 1,
  parent_id INTEGER,
  depth INTEGER NOT NULL DEFAULT 0,
  sub_task_ids TEXT NOT NULL DEFAULT '[]',
  FOREIGN KEY(project_id) REFERENCES projects(id) ON DELETE CASCADE
) STRICT;


-- Trigger to update parent task when deleting a subtask
CREATE TRIGGER IF NOT EXISTS tasks_after_delete_cleanup
AFTER DELETE ON tasks
FOR EACH ROW
WHEN OLD.parent_id IS NOT NULL
BEGIN
  UPDATE tasks
  SET sub_task_ids = json_remove(
    sub_task_ids,
    '$[' ||
    (SELECT json_each.key
     FROM json_each(sub_task_ids)
     WHERE CAST(json_each.value AS INTEGER) = OLD.id
     LIMIT 1)
    || ']'
  )
  WHERE id = OLD.parent_id
    AND EXISTS (
      SELECT 1
      FROM json_each(sub_task_ids)
      WHERE CAST(value AS INTEGER) = OLD.id
    );
END;

-- Trigger to cascade deletion of sub tasks
CREATE TRIGGER IF NOT EXISTS tasks_before_delete_cascade
BEFORE DELETE ON tasks
FOR EACH ROW
WHEN EXISTS (
  SELECT 1
  FROM json_each(OLD.sub_task_ids)
)
BEGIN
  DELETE FROM tasks
  WHERE id IN (
    SELECT CAST(value AS INTEGER)
    FROM json_each(OLD.sub_task_ids)
  );
END;
