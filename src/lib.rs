// region:    --- Modules

#![allow(unused)]
/// CLI module
pub mod cli;

pub mod error;
pub mod run;
pub mod objects;
pub mod controller;

// -- Public use
pub use error::{Error, Result};
pub use cli::Cli;
pub use run::run;

// endregion: --- Modules
