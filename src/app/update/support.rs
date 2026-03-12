use crate::{
    app::model::Twodo,
    cli::TaskListArg,
    constants::PROJECT_INBOX_ID,
    controller::delegater::{read_projects, read_tasks},
    Result,
};

pub async fn get_twodo(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Twodo> {
    let task_list_arg = TaskListArg {
        project_id: Some(PROJECT_INBOX_ID),
        number: None,
    };
    let tasknodes = read_tasks(db, task_list_arg).await?;
    let projects = read_projects(db).await?;
    Ok(Twodo {
        tasknodes,
        projects,
    })
}

