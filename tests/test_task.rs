use clap::Parser;
use twodo::Cli;
use sqlx::Row;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

#[tokio::test]
async fn test_add_tasks() -> Result<()> {
    // -- Setup & Fixtures
    let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
    // create table
    let result = sqlx::query(
        "CREATE TABLE tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT
            ) STRICT",
    )
    .execute(&db)
    .await?;
    println!("Created table tasks {:?}", result);

    // -- Exec
    let task_title = "'Buy Milk'";
    let _ = Cli::try_parse_from(["twodo", "add", task_title])?;

    // -- Check
    let result = sqlx::query(
        format!("SELECT * from tasks where title={task_title};").as_str()
    )
    .fetch_one(&db)
    .await?;

    let result_title: String = result.get("title");
    assert_eq!(result_title, task_title);
    Ok(())
}
