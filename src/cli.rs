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

    /// Edit a project
    Edit(ProjectEditArg),

    /// Delete a project
    Delete(ProjectDeleteArg),
}

/// Add arguments for project
#[derive(Debug, PartialEq, Args)]
pub struct ProjectAddArg {
    /// Name of project
    pub name: String,
}

/// Edit arguments for project
#[derive(Debug, PartialEq, Args)]
pub struct ProjectEditArg {
    /// Id of project to edit
    pub id: i64,

    /// Name of project
    #[arg(short, long)]
    pub name: String,
}

/// Delete arguments for project
#[derive(Debug, PartialEq, Args)]
pub struct ProjectDeleteArg {
    /// Id of project to delete
    pub id: i64,
}

/// Task operations
#[derive(Subcommand, Debug, PartialEq)]
pub enum TaskOp {
    /// List all task
    List(TaskListArg),

    /// Add a task
    Add(TaskAddArg),

    /// Complete a task
    Done(TaskDoneArg),

    /// Edit a task
    Edit(TaskEditArg),

    /// Delete a task
    Delete(TaskDeleteArg),
}

/// List arguments for task
#[derive(Debug, Default, PartialEq, Args)]
pub struct TaskListArg {
    /// Output format
    #[arg(short, long)]
    output: Option<bool>,

    /// Number of task to list
    #[arg(short)]
    number: Option<usize>,
}

/// Add arguments for task
#[derive(Debug, PartialEq, Args)]
pub struct TaskAddArg {
    /// Title of task
    pub title: String,

    /// Description for task
    #[arg(short, long, requires = "title")]
    pub description: Option<String>,

    /// Project id for task
    #[arg(short, long, default_value_t = 1)]
    pub project_id: i64,
}

/// Edit arguments for task
#[derive(Debug, PartialEq, Args)]
pub struct TaskEditArg {
    /// Id of task to edit
    pub id: i64,

    /// Title of task
    #[arg(short, long)]
    pub title: Option<String>,

    /// Description of task
    #[arg(short, long)]
    pub description: Option<String>,
}

/// Delete arguments for task
#[derive(Debug, PartialEq, Args)]
pub struct TaskDeleteArg {
    /// Id of task to delete
    pub id: i64,
}

/// Done arguments for task
#[derive(Debug, PartialEq, Args)]
pub struct TaskDoneArg {
    /// Id of task to complete
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
