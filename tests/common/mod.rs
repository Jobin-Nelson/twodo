use clap::Parser;
use twodo::{Cli, controller::delegater::delegate_item};

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

pub async fn init_db() -> Result<sqlx::SqlitePool> {
    let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
    // create table
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

pub async fn exec_cli(db: &sqlx::SqlitePool, args: Vec<&str>) -> Result<()> {
    let args = Cli::try_parse_from(args)?;
    delegate_item(db, args.item.unwrap()).await?;
    Ok(())
}
