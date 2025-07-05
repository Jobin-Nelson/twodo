use std::fmt::Display;

use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from(String, &String, &str)]
    Custom(String),

    // Tui
    MissingProjectId,
    MissingTaskId,

    // -- Externals
    #[from]
    Io(std::io::Error),

    #[from]
    Sql(sqlx::error::Error),

    #[from]
    Migrate(sqlx::migrate::MigrateError),
}

// region:    --- Custom

impl Error {
    pub fn custom_from_err(err: impl std::error::Error) -> Self {
        Self::Custom(err.to_string())
    }

    pub fn custom(val: impl Into<String>) -> Self {
        Self::Custom(val.into())
    }
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
