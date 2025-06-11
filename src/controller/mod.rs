use crate::{cli::ListArg, Cli, Result};
use crate::cli::{AddArg, Op};
use sqlx::SqlitePool;

pub async fn delegate(db: SqlitePool, cli: Cli) -> Result<()> {
    let op = cli.op.unwrap_or_else(|| Op::List(ListArg::default()));
    match op {
        Op::List(list_arg) => todo!(),
        Op::Add(add_arg) => add_task(&db, add_arg).await?,
        Op::Done => todo!(),
        Op::Edit => todo!(),
        Op::Delete => todo!(),
    }

    Ok(())
}

pub async fn add_task(db: &SqlitePool, add_arg: AddArg) -> Result<()> {
    let result = sqlx::query(
        "INSERT INTO tasks (title, description)
        VALUES (?1, ?2) RETURNING id",
    )
    .bind(add_arg.title)
    .bind(add_arg.description)
    .execute(db)
    .await?;
    println!("Created table tasks {:?}", result);

    Ok(())
}
