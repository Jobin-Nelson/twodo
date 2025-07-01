use clap::Parser;
use twodo::{
    controller::list_task,
    objects::Task,
    Cli,
};

mod common;
use common::Result;

#[tokio::test]
async fn test_add_tasks() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;

    // -- Exec
    let task_title = "'Buy Milk'";
    common::exec_cli(&db, vec!["twodo", "add", task_title]).await?;

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
    let db = common::init_db().await?;

    // -- Exec
    let task_title = "'Read Harry Potter'";
    common::exec_cli(&db, vec!["twodo", "add", task_title]).await?;

    // -- Check
    let mut stdout = Vec::new();
    let args = Cli::try_parse_from(["twodo", "list"])?;
    match args.op {
        Some(twodo::cli::TaskOp::List(list_arg)) => list_task(&db, list_arg, &mut stdout).await?,
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
    let db = common::init_db().await?;
    let task_title = "'Read zero 2 production in rust'";
    common::exec_cli(&db, vec!["twodo", "add", task_title]).await?;

    // -- Exec
    let edited_task_id = common::get_task_id(&db).await?;

    let edited_task_title = "'Read zero 2 production book in rust'";
    common::exec_cli(
        &db,
        vec![
            "twodo",
            "edit",
            &edited_task_id.to_string(),
            "-t",
            edited_task_title,
        ],
    )
    .await?;

    // -- Check
    let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
        .bind(edited_task_title)
        .fetch_one(&db)
        .await?;

    let result_title: String = task.title;
    assert_eq!(result_title, edited_task_title);
    Ok(())
}

#[tokio::test]
async fn test_delete_task() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;
    let task_title = "'test delete task'";
    common::exec_cli(&db, vec!["twodo", "add", task_title]).await?;

    // -- Exec
    let task_id = common::get_task_id(&db).await?;

    common::exec_cli(&db, vec!["twodo", "delete", &task_id.to_string()]).await?;

    // -- Check
    let task: Option<Task> = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
        .bind(task_title)
        .fetch_optional(&db)
        .await?;

    assert_eq!(None, task);
    Ok(())
}

#[tokio::test]
async fn test_done_task() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;
    let task_title = "'test done task'";
    common::exec_cli(&db, vec!["twodo", "add", task_title]).await?;
    let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
        .bind(task_title)
        .fetch_one(&db)
        .await?;

    assert!(!task.done);

    // -- Exec
    let task_id = common::get_task_id(&db).await?;
    common::exec_cli(&db, vec!["twodo", "done", &task_id.to_string()]).await?;

    // -- Check
    let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
        .bind(task_title)
        .fetch_one(&db)
        .await?;

    assert!(task.done);
    Ok(())
}
