// region:    --- Modules
mod app;
mod run;

// -- Flatten
pub use app::App;
pub use cli::Cli;
pub use error::{Error, Result};
pub use run::run;

/// Pubilc modules
pub(crate) mod constants;

pub mod cli;
pub mod controller;
pub mod error;
pub mod objects;

// endregion: --- Modules
