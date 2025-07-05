use std::collections::HashMap;

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
    let unordered_tasks = read_task(db, task_list_arg).await?;
    let tasks = reorder_tasks(unordered_tasks);
    let projects = get_projects(db).await?;
    Ok(Twodo { tasks, projects })
}

async fn get_projects(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<Project>> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(db)
        .await
        .map_err(Into::into)
}

pub fn reorder_tasks(tasks: Vec<Task>) -> Vec<Task> {
    let mut task_id_to_index: HashMap<i64, usize> = HashMap::new();
    let mut parent_to_children: HashMap<Option<i64>, Vec<i64>> = HashMap::new();

    for (index, task) in tasks.iter().enumerate() {
        task_id_to_index.insert(task.id, index);
        parent_to_children
            .entry(task.parent_id)
            .or_default()
            .push(task.id);
    }

    let mut reordered_task_ids = Vec::new();
    let mut stack = Vec::new();

    // Start traversal from root tasks (parent_id == None)
    if let Some(root_ids) = parent_to_children.get(&None) {
        for &root_id in root_ids.iter() {
            stack.push(root_id);
        }
    }

    while let Some(task_id) = stack.pop() {
        reordered_task_ids.push(task_id);
        if let Some(children) = parent_to_children.get(&Some(task_id)) {
            for &child_id in children.iter() {
                stack.push(child_id);
            }
        }
    }

    reordered_task_ids
        .into_iter()
        .map(|id| task_id_to_index.get(&id).map(|&i| tasks[i].clone()).unwrap())
        .collect()
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
                depth: 0,
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
            (Some(2), 7),
            (Some(7), 9),
            (Some(7), 8),
            (Some(2), 5),
            (Some(5), 6),
        ];
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
        Ok(())
    }
}

// endregion: --- Tests
