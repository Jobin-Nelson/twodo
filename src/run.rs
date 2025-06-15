use crate::controller::delegate;
use crate::Cli;
use crate::Result;
use clap::Parser;
use sqlx::migrate::MigrateDatabase;
use sqlx::SqlitePool;

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let db = init_db().await?;
    delegate(&db, cli).await?;
    Ok(())
}

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
