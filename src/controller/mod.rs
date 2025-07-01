use crate::{
    app::Message,
    cli::{
        Item, ProjectOp, TaskAddArg, TaskDeleteArg, TaskDoneArg, TaskEditArg, TaskListArg, TaskOp, ProjectAddArg,
    },
    objects::{Task, Project},
    App, Cli, Result,
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

    match cli.item {
        Some(op) => delegate_item(&db, op).await.map(|_| ()),
        None => start_tui(db).await,
    }
}

pub async fn delegate_item(db: &SqlitePool, item: Item) -> Result<Message> {
    match item {
        Item::Project(project_op) => delegate_project_op(db, project_op).await,
        Item::Task(task_op) => delegate_task_op(db, task_op).await,
    }
}

pub async fn delegate_project_op(db: &SqlitePool, op: ProjectOp) -> Result<Message> {
    match op {
        ProjectOp::List => list_project(db, &mut std::io::stdout()).await,
        ProjectOp::Add(add_arg) => add_project(db, add_arg).await,
    }
}

pub async fn delegate_task_op(db: &SqlitePool, op: TaskOp) -> Result<Message> {
    match op {
        TaskOp::List(list_arg) => list_task(db, list_arg, &mut std::io::stdout()).await,
        TaskOp::Add(add_arg) => add_task(db, add_arg).await,
        TaskOp::Edit(edit_arg) => edit_task(db, edit_arg).await,
        TaskOp::Done(done_arg) => done_task(db, done_arg).await,
        TaskOp::Delete(delete_arg) => delete_task(db, delete_arg).await,
    }
}

pub async fn add_task(db: &SqlitePool, add_arg: TaskAddArg) -> Result<Message> {
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
    add_arg: TaskListArg,
    mut writer: T,
) -> Result<Message> {
    let tasks: Vec<Task> = sqlx::query_as("SELECT * FROM tasks").fetch_all(db).await?;

    for task in tasks {
        writeln!(writer, "{}. {}", task.id, task.title)?;
    }

    Ok(Message::Noop)
}

pub async fn edit_task(db: &SqlitePool, edit_arg: TaskEditArg) -> Result<Message> {
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

pub async fn delete_task(db: &SqlitePool, edit_arg: TaskDeleteArg) -> Result<Message> {
    sqlx::query("DELETE FROM tasks WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::Noop)
}

pub async fn done_task(db: &SqlitePool, edit_arg: TaskDoneArg) -> Result<Message> {
    sqlx::query("UPDATE tasks SET done = true WHERE id = ?1")
        .bind(edit_arg.id)
        .execute(db)
        .await?;

    Ok(Message::Noop)
}

pub async fn list_project(db: &SqlitePool, mut writer: impl std::io::Write) -> Result<Message> {
    let projects: Vec<Project> = sqlx::query_as("SELECT * FROM projects")
        .fetch_all(db)
        .await?;

    for project in projects {
        writeln!(writer, "{}", project.name)?;
    }

    Ok(Message::Noop)
}

pub async fn add_project(db: &SqlitePool, add_arg: ProjectAddArg) -> Result<Message> {
    let result: i64 = sqlx::query_scalar(
        "INSERT INTO projects (name)
        VALUES (?1) RETURNING id",
    )
    .bind(add_arg.name)
    .fetch_one(db)
    .await?;

    Ok(Message::Noop)
}

pub async fn start_tui(db: SqlitePool) -> Result<()> {
    let terminal = ratatui::init();
    let app_result = App::new(db).await?.run(terminal).await;
    ratatui::restore();
    app_result
}
