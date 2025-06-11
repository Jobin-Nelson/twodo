use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Task {
    id: usize,
    title: String,
    description: String,
}

