use crate::{
    cli::{AddArg, DeleteArg, DoneArg, EditArg, ListArg, Op},
    objects::Task,
    Cli, Result,
    App,
};
use sqlx::SqlitePool;

pub async fn delegate(db: &SqlitePool, cli: Cli) -> Result<()> {
    // Start TUI if no operation is specified
    if cli.op.is_none() {
        return start_tui().await;
    }

    let op = cli.op.unwrap();
    match op {
        Op::List(list_arg) => list_task(db, list_arg, &mut std::io::stdout()).await?,
        Op::Add(add_arg) => add_task(db, add_arg).await?,
        Op::Edit(edit_arg) => edit_task(db, edit_arg).await?,
        Op::Done(done_arg) => done_task(db, done_arg).await?,
        Op::Delete(delete_arg) => delete_task(db, delete_arg).await?,
    }

    Ok(())
}

pub async fn add_task(db: &SqlitePool, add_arg: AddArg) -> Result<()> {
    let result: i64 = sqlx::query_scalar(
        "INSERT INTO tasks (title, description)
        VALUES (?1, ?2) RETURNING id",
    )
    .bind(add_arg.title)
    .bind(add_arg.description)
    .fetch_one(db)
    .await?;

    Ok(())
}

pub async fn list_task<T: std::io::Write>(
    db: &SqlitePool,
    add_arg: ListArg,
    mut writer: T,
) -> Result<()> {
    let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM tasks").fetch_all(db).await?;

    for task in tasks {
        writeln!(writer, "{}. {}", task.id, task.title)?;
    }

    Ok(())
}

pub async fn edit_task(db: &SqlitePool, edit_arg: EditArg) -> Result<()> {
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
    Ok(())
}

pub async fn delete_task(db: &SqlitePool, edit_arg: DeleteArg) -> Result<()> {
    sqlx::query("DELETE FROM tasks WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn done_task(db: &SqlitePool, edit_arg: DoneArg) -> Result<()> {
    sqlx::query("UPDATE tasks SET done = true WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn start_tui() -> Result<()> {
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}
