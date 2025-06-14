use crate::controller::delegate;
use crate::Cli;
use crate::Result;
use clap::Parser;
use sqlx::SqlitePool;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let db = init_db().await?;
    delegate(&db, cli).await?;
    Ok(())
}

pub async fn init_db() -> Result<SqlitePool> {
    let db = sqlx::SqlitePool::connect("twodo.db").await?;
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}
