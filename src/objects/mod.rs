use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
}

