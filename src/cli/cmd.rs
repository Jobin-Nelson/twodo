#![deny(missing_docs)]

use crate::cli::{ProjectOp, TaskOp};
use clap::{Parser, Subcommand};

/// Twodo CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Operation for twodo
    #[command(subcommand)]
    pub item: Option<Item>,
}

/// Twodo items
#[derive(Subcommand, Debug, PartialEq)]
pub enum Item {
    /// Project operations
    #[command(subcommand)]
    Project(ProjectOp),

    /// Task operations
    #[command(subcommand)]
    Task(TaskOp),
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}

// endregion: --- Tests
