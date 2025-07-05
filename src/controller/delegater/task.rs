use crate::{
    Result,
    app::Message,
    cli::{TaskAddArg, TaskDeleteArg, TaskDoneArg, TaskEditArg, TaskListArg, TaskOp},
    objects::Task,
};
use sqlx::SqlitePool;

pub async fn delegate_task_op(db: &SqlitePool, op: TaskOp) -> Result<Message> {
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
    sqlx::query(
        "INSERT INTO tasks (title, description, project_id)
        VALUES (?1, ?2, ?3)",
    )
    .bind(add_arg.title)
    .bind(add_arg.description)
    .bind(add_arg.project_id)
    .execute(db)
    .await?;

    Ok(Message::ReloadTask)
}

pub async fn list_task<T: std::io::Write>(
    db: &SqlitePool,
    _list_arg: TaskListArg,
    mut writer: T,
) -> Result<Message> {
    let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM tasks").fetch_all(db).await?;

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

async fn delete_task(db: &SqlitePool, edit_arg: TaskDeleteArg) -> Result<Message> {
    sqlx::query("DELETE FROM tasks WHERE id = ?1")
        .bind(edit_arg.id)
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
