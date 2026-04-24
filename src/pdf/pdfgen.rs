use std::collections::HashMap;
use anyhow::Result;
use printpdf::*;
use rusttype::{point, Font, Scale};
use std::io::Cursor;
use lopdf::{Document, content::{Content, Operation}, Object};
use crate::{config::config_loader::Config, utils::{output_pattern::render_output_name, text_props::{resolve_color, resolve_size}}};

pub fn generate_certificate(record: &HashMap<String, String>, config: &Config, template_bytes: &[u8], output_dir: &str, font_bytes: &Option<Vec<u8>>) -> Result<()> {
    let mut doc = Document::load_mem(template_bytes)?;

    let font_id = b"F1".to_vec();

    // Load the first page
    let page_id = doc.get_pages().values().next().copied().ok_or_else(|| anyhow::anyhow!("Template has no pages"))?;

    // Extract existing content
    let existing_content = doc.get_page_content(page_id)?;
    let mut content = Content::decode(&existing_content)?;

    let rt_font = font_bytes.as_ref().and_then(|bytes| Font::try_from_vec(bytes.clone()));

    // for (field_name, position) in &config.fields {
    //     // Font properties
    //     let size = resolve_size(position.size, config.font_size);
    //     let text_col = resolve_color(position.color, config.text_color);
    //     let (r, g, b) = (text_col[0], text_col[1], text_col[2]);
    //
    //     if let Some(value) = record.get(field_name) {
    //         // BT: Begin Text
    //         content.operations.push(Operation::new("BT", vec![]));
    //
    //         // Tf: Font properties
    //         content.operations.push(Operation::new("Tf", vec![
    //             Object::Name(font_id.clone()),
    //             Object::Real(size),
    //         ]));
    //
    //         // rg: Fill properties
    //         content.operations.push(Operation::new("rg", vec![
    //             Object::Real(r),
    //             Object::Real(g),
    //             Object::Real(b),
    //         ]));
    //
    //         // Td: Text positioning
    //         content.operations.push(Operation::new("Td", vec![
    //             Object::Real(position.x),
    //             Object::Real(position.y),
    //         ]));
    //
    //         // Tj: Add the text content
    //         content.operations.push(Operation::new("Tj", vec![Object::string_literal(value.to_owned())]));
    //
    //         // ET: End Text
    //         content.operations.push(Operation::new("ET", vec![]));
    //     }
    // }

    for (field_name, position) in &config.fields {
        if let Some(value) = record.get(field_name) {
            let mut size = resolve_size(position.size, config.font_size);

            // Auto-width math
            if let (Some(font), Some(max_w)) = (&rt_font, position.max_width) {
                let scale = Scale::uniform(size);
                let v_metrics = font.v_metrics(scale);
                let glyphs: Vec<_> = font.layout(value, scale, point(0.0, v_metrics.ascent)).collect();

                let actual_width = glyphs.iter()
                    .rev()
                    .next()
                    .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
                    .unwrap_or(0.0);

                if actual_width > max_w {
                    let scale_factor = max_w / actual_width;
                    size *= scale_factor;
                }
            }

            let text_col = resolve_color(position.color, config.text_color);

            content.operations.push(Operation::new("BT", vec![]));
            content.operations.push(Operation::new("Tf", vec![
                Object::Name(font_id.clone()),
                Object::Real(size),
            ]));
            content.operations.push(Operation::new("rg", vec![
                Object::Real(text_col[0]), Object::Real(text_col[1]), Object::Real(text_col[2]),
            ]));
            content.operations.push(Operation::new("Td", vec![
                Object::Real(position.x), Object::Real(position.y),
            ]));
            content.operations.push(Operation::new("Tj", vec![Object::string_literal(value.to_owned())]));
            content.operations.push(Operation::new("ET", vec![]));
        }
    }

    // Append the content to the page
    doc.change_page_content(page_id, content.encode()?);

    // Save new PDF
    let filename = format!("{}/{}", output_dir, render_output_name(&config.output_name, record));
    // std::fs::create_dir_all("output")?;
    doc.save(&filename)?;

    // println!("Generated: {}", &filename);
    Ok(())
}
