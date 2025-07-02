#![deny(missing_docs)]
use clap::{Args, Subcommand};

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

