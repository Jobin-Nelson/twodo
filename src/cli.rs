#![deny(missing_docs)]

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

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

/// Project operations
#[derive(Subcommand, Debug, PartialEq)]
pub enum ProjectOp {
    /// List all projects
    List,

    /// Add a project
    Add(ProjectAddArg),
}

/// Add arguments for project
#[derive(Debug, PartialEq, Args)]
pub struct ProjectAddArg {
    /// Name of project
    pub name: String,
}

/// Task operations
#[derive(Subcommand, Debug, PartialEq)]
pub enum TaskOp {
    /// List all twodo
    List(TaskListArg),

    /// Add a twodo
    Add(TaskAddArg),

    /// Complete a twodo
    Done(TaskDoneArg),

    /// Edit a twodo
    Edit(TaskEditArg),

    /// Delete a twodo
    Delete(TaskDeleteArg),
}

/// List arguments for twodo
#[derive(Debug, Default, PartialEq, Args)]
pub struct TaskListArg {
    /// Output format
    #[arg(short, long)]
    output: Option<bool>,

    /// Number of twodo to list
    #[arg(short)]
    number: Option<usize>,
}

/// Add arguments for twodo
#[derive(Debug, PartialEq, Args)]
pub struct TaskAddArg {
    /// Title of twodo
    pub title: String,

    /// Description for twodo
    #[arg(short, long, requires = "title")]
    pub description: Option<String>,
}

/// Edit arguments for twodo
#[derive(Debug, PartialEq, Args)]
pub struct TaskEditArg {
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
#[derive(Debug, PartialEq, Args)]
pub struct TaskDeleteArg {
    /// Id of twodo to delete
    pub id: i64,
}

/// Done arguments for twodo
#[derive(Debug, PartialEq, Args)]
pub struct TaskDoneArg {
    /// Id of twodo to complete
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
