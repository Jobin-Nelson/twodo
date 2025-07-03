use crate::{Result, objects::Task};

pub async fn get_tasks(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<Task>> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks where done = 0")
        .fetch_all(db)
        .await
        .map_err(Into::into)
}
