use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::{config::config_loader::Config, pdf::pdfgen::generate_certificate};
use crate::data::data_loader::read_csv;

use super::commands::{Cli, Commands};

pub fn handle_cli(cmd: Cli) -> Result<()> {
    match cmd.command {
        Commands::Generate { config, data } => {
            let conf = Arc::new(Config::from_file(config.as_str())?);
            conf.validate()?;

            // Create directory with timestamp
            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
            let output_dir = format!("output/batch_{}", timestamp);
            let log_dir = "./logs";
            std::fs::create_dir_all(&output_dir)?;
            std::fs::create_dir_all(log_dir)?;

            // Load files
            let template_bytes = std::fs::read(&conf.template)?;
            let records = read_csv(&data)?;
            let total_records = records.len();

            // Load font bytes
            let font_data = if let Some(path) = &conf.font_path {
                Some(std::fs::read(path)?)
            } else {
                None
            };
            let font_data = Arc::new(font_data);

            // Progress bar
            let pb = ProgressBar::new(total_records as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
                .progress_chars("#>-"));

            // Shared state for logs
            let failed_records = Arc::new(Mutex::new(Vec::new()));
            let log_filepath = format!("{}/run_{}.log", log_dir, timestamp);
            let mut log_file = OpenOptions::new().create(true).append(true).open(&log_filepath)?;

            writeln!(log_file, "Starting at {}\nTotal: {}\n---", timestamp, total_records)?;

            // Working
            records.par_iter().for_each(|record| {
                let conf = Arc::clone(&conf);
                let font_ref = Arc::clone(&font_data);
                pb.set_message(format!("Generating: {}", record.get("name").unwrap_or(&"Unknown".into())));

                if let Err(e) = generate_certificate(&record, &conf, &template_bytes, &output_dir, &font_ref) {
                    let mut failed = failed_records.lock().unwrap();
                    failed.push((record.clone(), e.to_string()));
                    // eprintln!("Error for {:?}: {}", record.get("name"), e);
                }
                pb.inc(1);
            });

            pb.finish_with_message("Batch complete!");

            // Handle failures
            let failed = failed_records.lock().unwrap();
            if !failed.is_empty() {
                println!("\n⚠️ Done with {} errors. Check failed_entries.csv", failed.len());

                let mut wtr = csv::Writer::from_path("failed_entries.csv")?;
                if let Some((first_map, _)) = failed.first() {
                    let headers: Vec<&String> = first_map.keys().collect();
                    wtr.write_record(&headers)?;
                    for (map, err) in failed.iter() {
                        let row: Vec<&String> = headers.iter().map(|&h| map.get(h).unwrap()).collect();
                        wtr.write_record(&row)?;
                        writeln!(log_file, "FAILED: {:?} | Error: {}", map.get("name"), err)?;
                    }
                }
            } else {
                println!("\n✅ All certificates generated successfully!");
                writeln!(log_file, "All entries processed successfully.")?;
            }

            println!("Log saved to: {}", log_filepath);
        }
    }

    Ok(())
}
