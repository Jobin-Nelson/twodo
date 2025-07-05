use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub project_id: i64,
    pub parent_id: Option<i64>,
    pub sub_task_ids: sqlx::types::Json<Vec<i64>>,
    pub depth: i64,
}

