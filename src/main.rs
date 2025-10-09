use anyhow::Result;
use clap::Parser;
use cli::{commands::Cli, handle};

mod cli;
mod config;

fn main() -> Result<()> {
    // Parse arguments
    let args = Cli::try_parse().unwrap_or_else(|e| e.exit());

    // Handle CLI arguments
    handle::handle_cli(args);

    Ok(())
}
