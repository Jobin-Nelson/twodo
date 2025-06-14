use clap::Parser;
use twodo::{controller::delegate, objects::Task, Cli};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

#[tokio::test]
async fn test_add_tasks() -> Result<()> {
    // -- Setup & Fixtures
    let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
    // create table
    sqlx::migrate!("./migrations").run(&db).await?;

    // -- Exec
    let task_title = "'Buy Milk'";
    let args = Cli::try_parse_from(["twodo", "add", task_title])?;
    delegate(&db, args).await?;

    // -- Check
    let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
        .bind(task_title)
        .fetch_one(&db)
        .await?;

    let result_title: String = task.title;
    assert_eq!(result_title, task_title);
    Ok(())
}
