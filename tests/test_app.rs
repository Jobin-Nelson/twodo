use twodo::App;
mod common;

#[tokio::test]
async fn test_app_new() -> common::Result<()> {
    // -- Setup & Fixtures
    let db = common::init_db().await?;
    let tasks = [
        "'Testing display of task'",
        "'World domination'",
        "'Rest day'",
    ];
    for task in tasks {
        common::exec_cli(&db, vec!["twodo", "task", "add", task]).await?;
    }

    // -- Exec
    let app = App::new(db).await?;

    // -- Check
    for task in tasks {
        assert!(app.twodo.tasks.iter().any(|t| t.title == task));
    }

    Ok(())
}
