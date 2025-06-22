use crate::controller::delegate;
use crate::Cli;
use crate::Result;
use clap::Parser;

pub async fn run() -> Result<()> {
    delegate(Cli::parse()).await
}

