use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="recred", version, about="A tool to generate certificates or documents in bulk.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Generate
    Generate {
        #[arg(short, long, help="Path to config file")]
        config: String,

        #[arg(short, long, help="Path to data CSV file")]
        data: String,
    },
}
