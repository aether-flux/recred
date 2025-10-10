use std::collections::HashMap;
use anyhow::Result;
use lopdf::{Document, content::{Content, Operation}, Object};
use crate::{config::config_loader::Config, utils::{output_pattern::render_output_name, text_props::{resolve_color, resolve_size}}};

pub fn generate_certificate(record: &HashMap<String, String>, config: &Config) -> Result<()> {
    let mut doc = Document::load(&config.template)?;

    // Load the first page
    let page_id = doc.get_pages().values().next().copied().unwrap();

    // Extract existing content
    let existing_content = doc.get_page_content(page_id)?;
    let mut content = Content::decode(&existing_content)?;

    for (field_name, position) in &config.fields {
        // Font properties
        let size = resolve_size(position.size, config.font_size);
        let text_col = resolve_color(position.color, config.text_color);
        let (r, g, b) = (text_col[0], text_col[1], text_col[2]);

        if let Some(value) = record.get(field_name) {
            // BT: Begin Text
            content.operations.push(Operation::new("BT", vec![]));

            // Tf: Font properties
            content.operations.push(Operation::new("Tf", vec![
                Object::Name(b"F1".to_vec()),
                Object::Real(size),
            ]));

            // rg: Fill properties
            content.operations.push(Operation::new("rg", vec![
                Object::Real(r),
                Object::Real(g),
                Object::Real(b),
            ]));

            // Td: Text positioning
            content.operations.push(Operation::new("Td", vec![
                Object::Real(position.x),
                Object::Real(position.y),
            ]));

            // Tj: Add the text content
            content.operations.push(Operation::new("Tj", vec![Object::string_literal(value.to_owned().as_str())]));

            // ET: End Text
            content.operations.push(Operation::new("ET", vec![]));
        }
    }

    // Append the content to the page
    doc.change_page_content(page_id, content.encode()?);

    // Save new PDF
    let filename = format!("output/{}", render_output_name(&config.output_name, record));
    std::fs::create_dir_all("output")?;
    doc.save(&filename)?;

    println!("Generated: {}", &filename);
    Ok(())
}
