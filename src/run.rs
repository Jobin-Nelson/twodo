use crate::Cli;
use crate::Result;
use crate::controller::delegater::delegate;
use clap::Parser;

pub async fn run() -> Result<()> {
    delegate(Cli::parse()).await
}
