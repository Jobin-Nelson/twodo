use twodo::objects::Task;

mod common;
use common::Result;


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
