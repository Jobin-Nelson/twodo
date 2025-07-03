use crate::{
    Result,
    app::Message,
    cli::Item,
    controller::delegater::{project::delegate_project_op, task::delegate_task_op},
};
use sqlx::SqlitePool;

pub async fn delegate_item(db: &SqlitePool, item: Item) -> Result<Message> {
    match item {
        Item::Project(project_op) => delegate_project_op(db, project_op).await,
        Item::Task(task_op) => delegate_task_op(db, task_op).await,
    }
}
