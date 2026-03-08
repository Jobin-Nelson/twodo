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

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use std::collections::HashSet;

    use super::*;

    #[tokio::test]
    async fn test_reorder_tasks() -> Result<()> {
        // -- Setup & Fixtures
        let parent_id_task_id = [
            (None, 1),
            (Some(1), 2),
            (Some(2), 5),
            (Some(3), 4),
            (Some(7), 8),
            (Some(5), 6),
            (Some(2), 7),
            (Some(2), 3),
            (Some(7), 9),
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
        let (reordered_tasks, actual_depth) = reorder_tasks(original_tasks);

        // -- Check
        let expected = [
            (None, 1),
            (Some(1), 2),
            (Some(2), 3),
            (Some(3), 4),
            (Some(2), 7),
            (Some(7), 9),
            (Some(7), 8),
            (Some(2), 5),
            (Some(5), 6),
        ];
        let expected_depth = [0, 1, 2, 3, 2, 3, 3, 2, 3];
        let mut visited_task_ids = HashSet::new();
        for (parent_id, task_id) in expected {
            assert!(visited_task_ids.insert(task_id));
            if let Some(parent_id) = parent_id {
                assert!(visited_task_ids.contains(&parent_id));
            }
        }
        let actual = reordered_tasks
            .into_iter()
            .map(|t| (t.parent_id, t.id))
            .collect::<Vec<_>>();
        assert_eq!(expected, actual.as_slice());
        assert_eq!(expected_depth, actual_depth.as_slice());
        Ok(())
    }
}

// endregion: --- Tests
