use crate::{
    app::model::Twodo,
    objects::{Project, Task},
    Result,
};

pub async fn get_twodo(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Twodo> {
    let tasks = get_tasks_by_project(db, 1).await?;
    let projects = get_projects(db).await?;
    Ok(Twodo { tasks, projects })
}

pub(super) async fn get_tasks_by_project(db: &sqlx::Pool<sqlx::Sqlite>, project_id: i64) -> Result<Vec<Task>> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE done = 0 AND project_id = ?1")
        .bind(project_id)
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
