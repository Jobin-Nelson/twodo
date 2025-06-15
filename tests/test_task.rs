use clap::Parser;
use twodo::{
    controller::{delegate, list_task},
    objects::Task,
    Cli,
};

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

#[tokio::test]
async fn test_list_tasks() -> Result<()> {
    // -- Setup & Fixtures
    let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
    // create table
    sqlx::migrate!("./migrations").run(&db).await?;

    // -- Exec
    let task_title = "'Read Harry Potter'";
    let args = Cli::try_parse_from(["twodo", "add", task_title])?;
    delegate(&db, args).await?;

    // -- Check
    let mut stdout = Vec::new();
    let args = Cli::try_parse_from(["twodo", "list"])?;
    match args.op {
        Some(twodo::cli::Op::List(list_arg)) => list_task(&db, list_arg, &mut stdout).await?,
        _ => panic!("Expected list operation"),
    };
    assert!(stdout
        .windows(task_title.len())
        .any(move |sub_slice| sub_slice == task_title.as_bytes()));
    Ok(())
}
