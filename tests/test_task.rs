use clap::Parser;
use twodo::{
    controller::{delegate, list_task},
    objects::Task,
    Cli,
};

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

async fn init_db() -> Result<sqlx::SqlitePool> {
    let db = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await?;
    // create table
    sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

async fn exec_cli(db: &sqlx::SqlitePool, args: Vec<&str>) -> Result<()> {
    let args = Cli::try_parse_from(args)?;
    delegate(db, args).await?;
    Ok(())
}

#[tokio::test]
async fn test_add_tasks() -> Result<()> {
    // -- Setup & Fixtures
    let db = init_db().await?;

    // -- Exec
    let task_title = "'Buy Milk'";
    exec_cli(&db, vec!["twodo", "add", task_title]).await?;

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
    let db = init_db().await?;

    // -- Exec
    let task_title = "'Read Harry Potter'";
    exec_cli(&db, vec!["twodo", "add", task_title]).await?;

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

#[tokio::test]
async fn test_edit_task() -> Result<()> {
    // -- Setup & Fixtures
    let db = init_db().await?;
    let task_title = "'Read zero 2 production in rust'";
    exec_cli(&db, vec!["twodo", "add", task_title]).await?;

    // -- Exec
    let mut stdout = Vec::new();
    let args = Cli::try_parse_from(["twodo", "list"])?;
    match args.op {
        Some(twodo::cli::Op::List(list_arg)) => list_task(&db, list_arg, &mut stdout).await?,
        _ => panic!("Expected list operation"),
    };

    let edited_task_id = std::str::from_utf8(&stdout)?
        .split('.')
        .next()
        .ok_or("No dot found")?
        .trim()
        .parse::<i64>()?;

    let edited_task_title = "'Read zero 2 production book in rust'";
    exec_cli(
        &db,
        vec!["twodo", "edit", &edited_task_id.to_string(), "-t", edited_task_title],
    )
    .await?;

    // -- Check
    let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
        .bind(task_title)
        .fetch_one(&db)
        .await?;

    let result_title: String = task.title;
    assert_eq!(result_title, task_title);
    Ok(())
}
