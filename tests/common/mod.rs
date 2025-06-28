use clap::Parser;
use twodo::{
    controller::{delegate_op, list_task},
    Cli,
};

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

pub async fn init_db() -> Result<sqlx::SqlitePool> {
    let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
    // create table
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn exec_cli(db: &sqlx::SqlitePool, args: Vec<&str>) -> Result<()> {
    let args = Cli::try_parse_from(args)?;
    delegate_op(db, args.op.unwrap()).await?;
    Ok(())
}

pub async fn get_task_id(db: &sqlx::SqlitePool) -> Result<i64> {
    let mut stdout = Vec::new();
    let args = Cli::try_parse_from(["twodo", "list"])?;
    match args.op {
        Some(twodo::cli::Op::List(list_arg)) => list_task(db, list_arg, &mut stdout).await?,
        _ => panic!("Expected list operation"),
    };

    let id = std::str::from_utf8(&stdout)?
        .split('.')
        .next()
        .ok_or("No dot found")?
        .trim()
        .parse::<i64>()?;

    Ok(id)
}

