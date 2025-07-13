use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Project {
    pub id: i64,
    pub name: String,
}
