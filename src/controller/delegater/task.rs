use crate::{
    app::Message,
    cli::{TaskAddArg, TaskDeleteArg, TaskDoneArg, TaskEditArg, TaskListArg, TaskOp},
    objects::Task,
    Result,
};
use sqlx::SqlitePool;

pub(crate) async fn delegate_task_op(db: &SqlitePool, op: TaskOp) -> Result<Message> {
    match op {
        TaskOp::List(list_arg) => list_task(db, list_arg, &mut std::io::stdout()).await,
        TaskOp::Add(add_arg) => add_task(db, add_arg).await,
        TaskOp::Edit(edit_arg) => edit_task(db, edit_arg).await,
        TaskOp::Done(done_arg) => done_task(db, done_arg).await,
        TaskOp::Delete(delete_arg) => delete_task(db, delete_arg).await,
        TaskOp::UnDone(undone_arg) => undone_task(db, undone_arg).await,
    }
}

async fn add_task(db: &SqlitePool, add_arg: TaskAddArg) -> Result<Message> {
    let query_str = if add_arg.parent_id.is_some() {
        "INSERT INTO tasks (title, description, project_id, parent_id)
        SELECT ?1, ?2, project_id, id as parent_id
        FROM tasks
        WHERE id = ?4
        RETURNING id"
    } else {
        "INSERT INTO tasks (title, description, project_id, parent_id)
        VALUES (?1, ?2, ?3, ?4)
        RETURNING id"
    };
    let task_id: i64 = sqlx::query_scalar(query_str)
        .bind(add_arg.title)
        .bind(add_arg.description)
        .bind(add_arg.project_id)
        .bind(add_arg.parent_id)
        .fetch_one(db)
        .await?;

    if let Some(parent_id) = add_arg.parent_id {
        sqlx::query(
            "UPDATE tasks
            SET sub_task_ids = json_insert(sub_task_ids,'$[#]',?1)
            WHERE id = ?2",
        )
        .bind(task_id)
        .bind(parent_id)
        .execute(db)
        .await?;
    };

    Ok(Message::ReloadTask)
}

pub async fn read_task(db: &SqlitePool, list_arg: TaskListArg) -> Result<Vec<Task>> {
    let mut query_str = "SELECT * FROM tasks".to_string();
    let mut where_clauses = Vec::new();
    let mut args = Vec::new();

    if let Some(project_id) = list_arg.project_id {
        where_clauses.push("project_id = ?");
        args.push(project_id.to_string());
    }

    let where_str = where_clauses.join(" AND ");

    if !where_str.is_empty() {
        query_str.push_str(" WHERE ");
        query_str.push_str(&where_str);
    }

    if let Some(number) = list_arg.number {
        query_str.push_str(" LIMIT ?");
        args.push(number.to_string());
    }

    let mut query = sqlx::query_as::<_, Task>(&query_str);
    for arg in args {
        query = query.bind(arg);
    }
    query.fetch_all(db).await.map_err(Into::into)
}

async fn list_task<T: std::io::Write>(
    db: &SqlitePool,
    _list_arg: TaskListArg,
    mut writer: T,
) -> Result<Message> {
    let tasks: Vec<Task> = read_task(db, _list_arg).await?;

    for task in tasks {
        writeln!(writer, "{}. {}", task.id, task.title)?;
    }

    Ok(Message::Noop)
}

async fn edit_task(db: &SqlitePool, edit_arg: TaskEditArg) -> Result<Message> {
    let mut query_str = "UPDATE tasks SET ".to_string();
    let mut args = Vec::new();
    let mut set_clauses = Vec::new();

    if let Some(title) = edit_arg.title {
        set_clauses.push("title = ?");
        args.push(title);
    }

    if let Some(description) = edit_arg.description {
        set_clauses.push("description = ?");
        args.push(description);
    }

    query_str.push_str(&set_clauses.join(", "));
    query_str.push_str(" WHERE id = ?");
    args.push(edit_arg.id.to_string());

    let mut query = sqlx::query::<sqlx::Sqlite>(&query_str);
    for arg in args {
        query = query.bind(arg);
    }
    query.execute(db).await?;
    Ok(Message::ReloadTask)
}

async fn delete_task(db: &SqlitePool, delete_arg: TaskDeleteArg) -> Result<Message> {
    sqlx::query("DELETE FROM tasks WHERE id = ?1")
        .bind(delete_arg.id)
        .execute(db)
        .await?;

    Ok(Message::ReloadTask)
}

async fn done_task(db: &SqlitePool, edit_arg: TaskDoneArg) -> Result<Message> {
    sqlx::query("UPDATE tasks SET done = true WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::ReloadTask)
}

async fn undone_task(db: &SqlitePool, edit_arg: TaskDoneArg) -> Result<Message> {
    sqlx::query("UPDATE tasks SET done = false WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::ReloadTask)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    async fn init_db() -> Result<sqlx::SqlitePool> {
        let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
        // create table
        sqlx::migrate!("./migrations").run(&db).await?;
        Ok(db)
    }

    use super::*;

    #[tokio::test]
    async fn test_add_tasks() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let parent_task_title = "'Parent Task'";
        let op = TaskOp::Add(TaskAddArg {
            title: parent_task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        // -- Exec
        let subtask_title = "'Sub Task title'";
        let op = TaskOp::Add(TaskAddArg {
            title: subtask_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        // -- Check
        let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(parent_task_title)
            .fetch_one(&db)
            .await?;

        let result_title: String = task.title;
        assert_eq!(result_title, parent_task_title);
        Ok(())
    }

    #[tokio::test]
    async fn test_add_subtasks() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let parent_task_title = "parent task";
        let op = TaskOp::Add(TaskAddArg {
            title: parent_task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        // -- Exec
        let parent_task_id = 1;
        let subtask_title = "sub task";
        let sub_task_id = 2;
        let op = TaskOp::Add(TaskAddArg {
            title: subtask_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: Some(parent_task_id),
        });
        delegate_task_op(&db, op).await?;

        // -- Check
        let sub_task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(subtask_title)
            .fetch_one(&db)
            .await?;

        assert_eq!(sub_task.parent_id, Some(parent_task_id));

        let parent_task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(parent_task_title)
            .fetch_one(&db)
            .await?;

        assert_eq!(parent_task.sub_task_ids.to_vec(), vec![sub_task_id]);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_parent_on_delete_subtasks() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let parent_task_title = "parent task";
        let parent_task_id = 1;
        let op = TaskOp::Add(TaskAddArg {
            title: parent_task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;
        let subtask_title = "sub task";
        let sub_task_id = 2;
        let op = TaskOp::Add(TaskAddArg {
            title: subtask_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: Some(parent_task_id),
        });
        delegate_task_op(&db, op).await?;

        // -- Exec
        let op = TaskOp::Delete(TaskDeleteArg { id: sub_task_id });
        delegate_task_op(&db, op).await?;

        // -- Check
        let parent_task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(parent_task_title)
            .fetch_one(&db)
            .await?;

        assert_eq!(parent_task.sub_task_ids.to_vec(), Vec::<i64>::new());
        Ok(())
    }

    #[tokio::test]
    async fn test_cascade_delete_subtasks() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        // (parent_id, task_title)
        let tasks = [
            (None, "parent task"),
            (Some(1), "child task"),
            (Some(2), "grand child task"),
        ];
        let parent_task_id = 1;
        for (parent_id, task_title) in tasks {
            let op = TaskOp::Add(TaskAddArg {
                title: task_title.to_string(),
                description: None,
                project_id: 1,
                parent_id,
            });
            delegate_task_op(&db, op).await?;
        }

        // -- Exec
        let op = TaskOp::Delete(TaskDeleteArg { id: parent_task_id });
        delegate_task_op(&db, op).await?;

        // -- Check
        for (id, _task_title) in tasks {
            let task: Option<Task> = sqlx::query_as("SELECT * FROM tasks WHERE id = ?1")
                .bind(id)
                .fetch_optional(&db)
                .await?;

            assert_eq!(None, task);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_list_tasks() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let task_title = "Test list tasks";
        let op = TaskOp::Add(TaskAddArg {
            title: task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        // -- Exec
        let mut stdout = Vec::new();
        let list_arg = TaskListArg {
            project_id: Some(1),
            number: None,
        };
        list_task(&db, list_arg, &mut stdout).await?;

        // -- Check
        assert!(stdout
            .windows(task_title.len())
            .any(move |sub_slice| sub_slice == task_title.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn test_edit_task() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let task_title = "Test edit tasks";
        let op = TaskOp::Add(TaskAddArg {
            title: task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        // -- Exec
        let edited_task_id = 1;
        let edited_task_title = "'Read zero 2 production book in rust'";
        let edit_arg = TaskOp::Edit(TaskEditArg {
            id: edited_task_id,
            title: Some(edited_task_title.to_string()),
            description: None,
        });
        delegate_task_op(&db, edit_arg).await?;

        // -- Check
        let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(edited_task_title)
            .fetch_one(&db)
            .await?;

        let result_title: String = task.title;
        assert_eq!(result_title, edited_task_title);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_task() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let task_title = "Test delete tasks";
        let op = TaskOp::Add(TaskAddArg {
            title: task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        // -- Exec
        let task_id = 1;
        let delete_arg = TaskOp::Delete(TaskDeleteArg { id: task_id });
        delegate_task_op(&db, delete_arg).await?;

        // -- Check
        let task: Option<Task> = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(task_title)
            .fetch_optional(&db)
            .await?;

        assert!(task.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_done_task() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;
        let task_title = "Test done tasks";
        let op = TaskOp::Add(TaskAddArg {
            title: task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(task_title)
            .fetch_one(&db)
            .await?;
        assert!(!task.done);

        // -- Exec
        let task_id = 1;
        let done_arg = TaskOp::Done(TaskDoneArg { id: task_id });
        delegate_task_op(&db, done_arg).await?;

        // -- Check
        let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
            .bind(task_title)
            .fetch_one(&db)
            .await?;

        assert!(task.done);
        Ok(())
    }

    #[tokio::test]
    async fn test_default_project() -> Result<()> {
        // -- Setup & Fixtures
        let db = init_db().await?;

        // -- Exec
        let task_title = "Test delete tasks";
        let op = TaskOp::Add(TaskAddArg {
            title: task_title.to_string(),
            description: None,
            project_id: 1,
            parent_id: None,
        });
        delegate_task_op(&db, op).await?;

        // -- Check
        let task_id = 1;
        let project: String = sqlx::query_scalar(
            "
            SELECT p.name
            FROM tasks AS t
            INNER JOIN projects as p
                ON t.project_id = p.id
            WHERE t.id = ?1
            ",
        )
        .bind(task_id)
        .fetch_one(&db)
        .await?;
        let expected_project = "INBOX";
        assert_eq!(expected_project, project);

        Ok(())
    }
}

// endregion: --- Tests
