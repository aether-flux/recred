use anyhow::Result;
use rayon::prelude::*;
use std::sync::Arc;
use crate::{config::config_loader::Config, pdf::pdfgen::generate_certificate};
use crate::data::data_loader::read_csv;

use super::commands::{Cli, Commands};

pub fn handle_cli(cmd: Cli) -> Result<()> {
    match cmd.command {
        Commands::Generate { config, data } => {
            println!("Config: {}, Data: {}\n\n", config, data);

            let conf = Config::from_file(config.as_str())?;
            let conf = Arc::new(conf);
            conf.validate()?;

            // CSV
            let records = read_csv(&data)?;

            // Working
            records.par_iter().for_each(|record| {
                let conf = Arc::clone(&conf);
                if let Err(e) = generate_certificate(&record, &conf) {
                    eprintln!("Error generating certificate: {}", e);
                }
            });
        }
    }

    Ok(())
}
