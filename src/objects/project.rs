use sqlx::prelude::FromRow;

#[derive(Debug, PartialEq, FromRow)]
pub struct Project {
    pub id: i64,
    pub name: String,
}
