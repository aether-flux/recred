use anyhow::Result;
use rayon::prelude::*;
use crate::config::config_loader::Config;
use crate::data::data_loader::read_csv;

use super::commands::{Cli, Commands};

pub fn handle_cli(cmd: Cli) -> Result<()> {
    match cmd.command {
        Commands::Generate { config, data } => {
            println!("Config: {}, Data: {}", config, data);

            let conf = Config::from_file(config.as_str())?;
            conf.validate()?;

            // CSV
            let records = read_csv("data.csv")?;

            // Working
            records.par_iter().for_each(|record| {
                for (field_name, position) in &conf.fields {
                    if let Some(value) = record.get(field_name) {
                        println!("Placing '{}' at ({}, {})", value, position.x, position.y);
                        // TODO: PDF editing and drawing
                    }
                }

                // TODO: Save PDF
            });
        }
    }

    Ok(())
}
