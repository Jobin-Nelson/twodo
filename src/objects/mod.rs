use sqlx::prelude::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub done: bool,
}

