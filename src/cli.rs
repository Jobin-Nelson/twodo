#![deny(missing_docs)]

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// Twodo CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Operation for twodo
    #[command(subcommand)]
    pub op: Option<Op>,
}

/// Operations on twodos
#[derive(Subcommand, Debug)]
pub enum Op {
    /// List all twodo
    List(ListArg),

    /// Add a twodo
    Add(AddArg),

    /// Complete a twodo
    Done,

    /// Edit a twodo
    Edit(EditArg),

    /// Delete a twodo
    Delete(DeleteArg),
}

/// List arguments for twodo
#[derive(Debug, Default, Args)]
pub struct ListArg {
    /// Output format
    #[arg(short, long)]
    output: Option<bool>,

    /// Number of twodo to list
    #[arg(short)]
    number: Option<usize>,
}

/// Add arguments for twodo
#[derive(Args, Debug)]
pub struct AddArg {
    /// Title of twodo
    pub title: String,

    /// Description for twodo
    #[arg(short, long, requires = "title")]
    pub description: Option<String>,
}

/// Edit arguments for twodo
#[derive(Debug, Args)]
pub struct EditArg {
    /// Id of twodo to edit
    pub id: i64,

    /// Title of twodo
    #[arg(short, long)]
    pub title: Option<String>,

    /// Description of twodo
    #[arg(short, long)]
    pub description: Option<String>,
}

/// Delete arguments for twodo
#[derive(Debug, Args)]
pub struct DeleteArg {
    /// Id of twodo to delete
    pub id: i64,
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
