use anyhow::Result;
use crate::config::config_parser::Config;

use super::commands::{Cli, Commands};

pub fn handle_cli(cmd: Cli) -> Result<()> {
    match cmd.command {
        Commands::Generate { config, data } => {
            println!("Config: {}, Data: {}", config, data);

            let conf = Config::from_file(config.as_str())?;
            println!("Config: {:#?}", conf);
        }
    }

    Ok(())
}
