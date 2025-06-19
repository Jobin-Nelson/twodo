// region:    --- Modules

#![allow(unused)]
/// CLI module
pub mod cli;

pub mod error;
pub mod run;
pub mod objects;
pub mod controller;
pub mod app;

// -- Public use
pub use error::{Error, Result};
pub use cli::Cli;
pub use run::run;
pub use app::App;

// endregion: --- Modules
