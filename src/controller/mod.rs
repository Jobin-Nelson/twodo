use crate::{
    app::Message, cli::{AddArg, DeleteArg, DoneArg, EditArg, ListArg, Op}, objects::Task, App, Cli, Result
};
use sqlx::{migrate::MigrateDatabase, SqlitePool};

pub async fn init_db() -> Result<SqlitePool> {
    // get db path
    let db_path = std::env::home_dir()
        .map(|h| h.join(".local/share/twodo/twodo.db"))
        .expect("Failed to get xdg base directory");

    // create db path
    std::fs::create_dir_all(
        db_path
            .parent()
            .expect("Failed to get parent of the twodo db path"),
    )?;

    let db_url = db_path
        .to_str()
        .expect("Failed to convert twodo_db_path to str");

    // create database if not existing
    if !sqlx::Sqlite::database_exists(db_url).await? {
        sqlx::Sqlite::create_database(db_url).await?;
    }

    // connect to database
    let db = sqlx::SqlitePool::connect(db_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn delegate(cli: Cli) -> Result<()> {
    // Start TUI if no operation is specified
    let db = init_db().await?;

    match cli.op {
        Some(op) => {
            delegate_op(&db, op).await?;
            Ok(())
        },
        None => start_tui(db).await,
    }
}

pub async fn delegate_op(db: &SqlitePool, op: Op) -> Result<Message> {
    match op {
        Op::List(list_arg) => list_task(db, list_arg, &mut std::io::stdout()).await,
        Op::Add(add_arg) => add_task(db, add_arg).await,
        Op::Edit(edit_arg) => edit_task(db, edit_arg).await,
        Op::Done(done_arg) => done_task(db, done_arg).await,
        Op::Delete(delete_arg) => delete_task(db, delete_arg).await,
    }
}

pub async fn add_task(db: &SqlitePool, add_arg: AddArg) -> Result<Message> {
    let result: i64 = sqlx::query_scalar(
        "INSERT INTO tasks (title, description)
        VALUES (?1, ?2) RETURNING id",
    )
    .bind(add_arg.title)
    .bind(add_arg.description)
    .fetch_one(db)
    .await?;

    Ok(Message::Noop)
}

pub async fn list_task<T: std::io::Write>(
    db: &SqlitePool,
    add_arg: ListArg,
    mut writer: T,
) -> Result<Message> {
    let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM tasks").fetch_all(db).await?;

    for task in tasks {
        writeln!(writer, "{}. {}", task.id, task.title)?;
    }

    Ok(Message::Noop)
}

pub async fn edit_task(db: &SqlitePool, edit_arg: EditArg) -> Result<Message> {
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
    Ok(Message::Noop)
}

pub async fn delete_task(db: &SqlitePool, edit_arg: DeleteArg) -> Result<Message> {
    sqlx::query("DELETE FROM tasks WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::Noop)
}

pub async fn done_task(db: &SqlitePool, edit_arg: DoneArg) -> Result<Message> {
    sqlx::query("UPDATE tasks SET done = true WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::Noop)
}

pub async fn start_tui(db: SqlitePool) -> Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new(db).run(terminal).await;
    ratatui::restore();
    app_result
}

