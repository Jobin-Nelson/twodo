use crate::{
    cli::{AddArg, ListArg, Op},
    objects::Task,
    Cli, Result,
};
use sqlx::SqlitePool;

pub async fn delegate(db: &SqlitePool, cli: Cli) -> Result<()> {
    let op = cli.op.unwrap_or_else(|| Op::List(ListArg::default()));
    match op {
        Op::List(list_arg) => list_task(db, list_arg, &mut std::io::stdout()).await?,
        Op::Add(add_arg) => add_task(db, add_arg).await?,
        Op::Done => todo!(),
        Op::Edit => todo!(),
        Op::Delete => todo!(),
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
