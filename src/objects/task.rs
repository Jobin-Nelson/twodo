use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub parent_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub position: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, FromRow, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct TaskNode {
    pub id: i64,
    pub project_id: i64,
    pub parent_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub position: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub level: i64,
}

