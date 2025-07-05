use std::collections::HashSet;

use crate::{
    app::model::Twodo,
    cli::TaskListArg,
    constants::PROJECT_INBOX_ID,
    controller::delegater::read_task,
    objects::{Project, Task},
    Result,
};

pub async fn get_twodo(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Twodo> {
    let task_list_arg = TaskListArg {
        project_id: Some(PROJECT_INBOX_ID),
        number: None,
    };
    let tasks = read_task(db, task_list_arg).await?;
    let projects = get_projects(db).await?;
    Ok(Twodo { tasks, projects })
}

async fn get_projects(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<Project>> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

pub fn reorder_tasks(mut tasks: Vec<Task>) -> Vec<Task> {
    let root_tasks: Vec<&Task> = tasks.iter().filter(|t| t.parent_id.is_none()).collect();
    let edges = tasks
        .iter()
        .map(|t| (t.parent_id.unwrap_or(0), t.id))
        .collect::<Vec<_>>();

    vec![]
}

fn get_edges(tasks: &[Task]) -> &[()] {
    todo!()
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    #[tokio::test]
    async fn test_reorder_tasks() -> Result<()> {
        // -- Setup & Fixtures
        let parent_id_task_id = [
            (None, 1),
            (Some(1), 2),
            (Some(4), 5),
            (Some(7), 8),
            (Some(2), 3),
            (Some(3), 4),
            (Some(7), 9),
            (Some(7), 9),
            (Some(5), 6),
            (Some(2), 7),
        ];

        let original_tasks = parent_id_task_id
            .into_iter()
            .map(|(parent_id, id)| Task {
                id,
                title: "test reorder".to_string(),
                description: None,
                done: false,
                project_id: 1,
                parent_id,
                sub_task_ids: sqlx::types::Json(Vec::new()),
            })
            .collect::<Vec<_>>();

        // -- Exec
        let reordered_tasks = reorder_tasks(original_tasks);

        // -- Check
        let expected = [
            (None, 1),
            (Some(1), 2),
            (Some(2), 3),
            (Some(3), 4),
            (Some(4), 5),
            (Some(5), 6),
            (Some(2), 7),
            (Some(7), 8),
            (Some(7), 9),
            (Some(7), 9),
        ];

        let actual = reordered_tasks
            .into_iter()
            .map(|t| (t.parent_id, t.id))
            .collect::<Vec<_>>();
        assert_eq!(expected, actual.as_slice());
        Ok(())
    }
}

// endregion: --- Tests
