use twodo::objects::{Project, Task};

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

#[tokio::test]
async fn test_edit_project() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;
    let project_name = "'School'";
    common::exec_cli(&db, vec!["twodo", "project", "add", project_name]).await?;

    // -- Exec
    let edited_project_id = 2;

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

    common::exec_cli(
        &db,
        vec!["twodo", "project", "delete", &project_id.to_string()],
    )
    .await?;

    // -- Check
    let task: Option<Project> = sqlx::query_as("SELECT * FROM projects WHERE name = ?1")
        .bind(project_name)
        .fetch_optional(&db)
        .await?;

    assert_eq!(None, task);
    Ok(())
}

#[tokio::test]
async fn test_default_project() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;

    // -- Exec
    let task = "'test default project'";
    common::exec_cli(&db, vec!["twodo", "task", "add", task]).await?;

    // -- Check
    let task_id = 1;
    let added_task: Task = sqlx::query_as("SELECT * FROM tasks WHERE id = ?1")
        .bind(task_id)
        .fetch_one(&db)
        .await?;
    let project_id = added_task.project_id;
    let project: Project = sqlx::query_as("SELECT * FROM projects WHERE id = ?1")
        .bind(project_id)
        .fetch_one(&db)
        .await?;
    let expected_project = "INBOX";
    assert_eq!(expected_project, project.name);

    Ok(())
}

#[tokio::test]
async fn test_cascade_delete_project() -> Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;

    // -- Exec
    let task = "'test cascade delete project'";
    common::exec_cli(&db, vec!["twodo", "task", "add", task]).await?;

    // -- Check
    let task_id = 1;
    let added_task: Task = sqlx::query_as("SELECT * FROM tasks WHERE id = ?1")
        .bind(task_id)
        .fetch_one(&db)
        .await?;
    let project_id = added_task.project_id;
    sqlx::query("DELETE FROM projects WHERE id = ?1")
        .bind(project_id)
        .execute(&db)
        .await?;
    let task: Option<Task> = sqlx::query_as("SELECT * FROM tasks WHERE id = ?1")
        .bind(task_id)
        .fetch_optional(&db)
        .await?;

    assert!(task.is_none());
    Ok(())
}
