use crate::controller::delegate;
use crate::Cli;
use crate::Result;
use clap::Parser;
use sqlx::SqlitePool;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let db = sqlx::SqlitePool::connect("twodo.db")
        .await?;
    delegate(db, cli);
    Ok(())
}

