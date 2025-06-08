use crate::Cli;
use clap::Parser;

pub fn run() {
    let cli = Cli::parse();
    println!("{cli:?}");
}

