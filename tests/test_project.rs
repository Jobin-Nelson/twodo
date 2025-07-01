use twodo::objects::Project;

mod common;
use common::Result;

#[tokio::test]
async fn test_add_project() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;

    // -- Exec
    let project_name = "'part-time-work-1'";
    common::exec_cli(&db, vec!["twodo", "project", "add", project_name]).await?;

    // -- Check
    let project: Project = sqlx::query_as("SELECT * FROM PROJECTS WHERE name = ?1")
        .bind(project_name)
        .fetch_one(&db)
        .await?;

    let result_name: String = project.name;
    assert_eq!(result_name, project_name);
    Ok(())
}

// #[tokio::test]
// async fn test_list_tasks() -> Result<()> {
//     // -- Setup & Fixtures
//     let db = common::init_db().await?;
//
//     // -- Exec
//     let task_title = "'Read Harry Potter'";
//     common::exec_cli(&db, vec!["twodo", "add", task_title]).await?;
//
//     // -- Check
//     let mut stdout = Vec::new();
//     let args = Cli::try_parse_from(["twodo", "list"])?;
//     match args.op {
//         Some(twodo::cli::Op::List(list_arg)) => list_task(&db, list_arg, &mut stdout).await?,
//         _ => panic!("Expected list operation"),
//     };
//     assert!(stdout
//         .windows(task_title.len())
//         .any(move |sub_slice| sub_slice == task_title.as_bytes()));
//     Ok(())
// }
//
#[tokio::test]
async fn test_edit_project() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;
    let project_name = "'School'";
    common::exec_cli(&db, vec!["twodo", "project", "add", project_name]).await?;

    // -- Exec
    let edited_project_id = common::get_project_id(&db).await?;

    let edited_project_name = "'College'";
    common::exec_cli(
        &db,
        vec![
            "twodo",
            "project",
            "edit",
            &edited_project_id.to_string(),
            "-n",
            edited_project_name,
        ],
    )
    .await?;

    // -- Check
    let project: Project = sqlx::query_as("SELECT * FROM projects WHERE name = ?1")
        .bind(edited_project_name)
        .fetch_one(&db)
        .await?;

    let result_name: String = project.name;
    assert_eq!(result_name, edited_project_name);
    Ok(())
}

#[tokio::test]
async fn test_delete_project() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;
    let project_name = "'test delete project'";
    common::exec_cli(&db, vec!["twodo", "project", "add", project_name]).await?;

    // -- Exec
    let project_id = 2;

    common::exec_cli(&db, vec!["twodo", "project", "delete", &project_id.to_string()]).await?;

    // -- Check
    let task: Option<Project> = sqlx::query_as("SELECT * FROM projects WHERE name = ?1")
        .bind(project_name)
        .fetch_optional(&db)
        .await?;

    assert_eq!(None, task);
    Ok(())
}

// #[tokio::test]
// async fn test_done_task() -> Result<()> {
//     // -- Setup & Fixtures
//     let db = common::init_db().await?;
//     let task_title = "'test done task'";
//     common::exec_cli(&db, vec!["twodo", "add", task_title]).await?;
//     let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
//         .bind(task_title)
//         .fetch_one(&db)
//         .await?;
//
//     assert!(!task.done);
//
//     // -- Exec
//     let task_id = common::get_task_id(&db).await?;
//     common::exec_cli(&db, vec!["twodo", "done", &task_id.to_string()]).await?;
//
//     // -- Check
//     let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE title = ?1")
//         .bind(task_title)
//         .fetch_one(&db)
//         .await?;
//
//     assert!(task.done);
//     Ok(())
// }
