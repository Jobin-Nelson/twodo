use sqlx::prelude::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub project_id: i64,
}
