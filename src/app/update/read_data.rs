use crate::{app::model::Twodo, objects::{Project, Task}, Result};

pub async fn get_twodo(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Twodo> {
    let tasks = get_tasks(db).await?;
    let projects = get_projects(db).await?;
    Ok(Twodo { tasks, projects })
}

async fn get_tasks(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<Task>> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks where done = 0")
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

async fn get_projects(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<Project>> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(db)
        .await
        .map_err(Into::into)
}
