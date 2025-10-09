use std::fs;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct FieldPos {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub template: String,
    pub font: String,
    pub font_size: f32,
    pub output_dir: String,
    pub output_name: String,
    pub text_color: Option<[u8; 3]>,
    pub fields: HashMap<String, FieldPos>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }
}
