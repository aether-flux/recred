use std::collections::HashMap;
use anyhow::Result;
use printpdf::*;
use rusttype::{point, Font, Scale};
use std::io::Cursor;
use lopdf::{content::{Content, Operation}, dictionary, Document, Object};
use crate::{config::config_loader::Config, utils::{output_pattern::render_output_name, text_props::{resolve_color, resolve_size}}};

const FALLBACK_FONT_DATA: &[u8] = include_bytes!("../../assets/fonts/Roboto-Regular.ttf");

pub fn generate_certificate(record: &HashMap<String, String>, config: &Config, template_bytes: &[u8], output_dir: &str, font_bytes: &Option<Vec<u8>>) -> Result<()> {
    let mut doc = Document::load_mem(template_bytes)?;

    let font_id = b"Helvetica".to_vec();

    // Load the first page
    let page_id = doc.get_pages().values().next().copied().ok_or_else(|| anyhow::anyhow!("Template has no pages"))?;

    // Get the Resources dict reference from the page
    let resources_id = {
        let page_dict = doc.get_dictionary(page_id)?;
        match page_dict.get(b"Resources")? {
            Object::Reference(r) => *r,
            Object::Dictionary(_) => page_id, // inline, handle differently
            _ => anyhow::bail!("No resources found"),
        }
    };

    // Add Helvetica to the font dict
    let helvetica_id = doc.add_object(
        lopdf::Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Font".to_vec())),
            ("Subtype", Object::Name(b"Type1".to_vec())),
            ("BaseFont", Object::Name(b"Helvetica".to_vec())),
        ])
    );

    let resources_dict = doc.get_object_mut(resources_id)?.as_dict_mut()?;
    if let Ok(fonts) = resources_dict.get_mut(b"Font").and_then(|f| f.as_dict_mut()) {
        fonts.set("Helvetica", Object::Reference(helvetica_id));
    } else {
        // Font dict doesn't exist yet, create it
        let mut font_dict = lopdf::Dictionary::new();
        font_dict.set("Helvetica", Object::Reference(helvetica_id));
        resources_dict.set("Font", Object::Dictionary(font_dict));
    }

    // Extract existing content
    // let existing_content = doc.get_page_content(page_id)?;
    // let mut content = Content::decode(&existing_content)?;

    let rt_font = font_bytes.as_ref().and_then(|bytes| Font::try_from_vec(bytes.clone())).unwrap_or_else(|| Font::try_from_bytes(FALLBACK_FONT_DATA).expect("Error loading internal fallback font"));

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

    // for (field_name, position) in &config.fields {
    //     if let Some(value) = record.get(field_name) {
    //         let mut size = resolve_size(position.size, config.font_size);
    //
    //         // Auto-width math
    //         if let (Some(font), Some(max_w)) = (&rt_font, position.max_width) {
    //             let scale = Scale::uniform(size);
    //             let v_metrics = font.v_metrics(scale);
    //             let glyphs: Vec<_> = font.layout(value, scale, point(0.0, v_metrics.ascent)).collect();
    //
    //             let actual_width = glyphs.iter()
    //                 .rev()
    //                 .next()
    //                 .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
    //                 .unwrap_or(0.0);
    //
    //             if actual_width > max_w {
    //                 let scale_factor = max_w / actual_width;
    //                 size *= scale_factor;
    //             }
    //         }
    //
    //         let text_col = resolve_color(position.color, config.text_color);
    //
    //         content.operations.push(Operation::new("BT", vec![]));
    //         content.operations.push(Operation::new("Tf", vec![
    //             Object::Name(font_id.clone()),
    //             Object::Real(size),
    //         ]));
    //         content.operations.push(Operation::new("rg", vec![
    //             Object::Real(text_col[0]), Object::Real(text_col[1]), Object::Real(text_col[2]),
    //         ]));
    //         content.operations.push(Operation::new("Td", vec![
    //             Object::Real(position.x), Object::Real(position.y),
    //         ]));
    //         content.operations.push(Operation::new("Tj", vec![Object::string_literal(value.to_owned())]));
    //         content.operations.push(Operation::new("ET", vec![]));
    //     }
    // }

    // Build ONLY your new operations, don't touch existing content
    let mut new_ops: Vec<Operation> = Vec::new();

    for (field_name, position) in &config.fields {
        if let Some(value) = record.get(field_name) {
            let mut size = resolve_size(position.size, config.font_size);
            let max_w = position.max_width.unwrap_or(450.0);

            let scale = Scale::uniform(size);
            let v_metrics = rt_font.v_metrics(scale);
            let glyphs: Vec<_> = rt_font.layout(value, scale, point(0.0, v_metrics.ascent)).collect();
            let actual_width = glyphs.iter().rev().next()
                .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
                .unwrap_or(0.0);

            if actual_width > max_w {
                size *= max_w / actual_width;
            }

            let text_col = resolve_color(position.color, config.text_color);

            new_ops.push(Operation::new("BT", vec![]));
            new_ops.push(Operation::new("Tf", vec![
                Object::Name(b"Helvetica".to_vec()),
                Object::Real(size),
            ]));
            new_ops.push(Operation::new("rg", vec![
                Object::Real(text_col[0]),
                Object::Real(text_col[1]),
                Object::Real(text_col[2]),
            ]));
            new_ops.push(Operation::new("Tm", vec![
                Object::Real(1.0), Object::Real(0.0),
                Object::Real(0.0), Object::Real(1.0),
                Object::Real(position.x), Object::Real(position.y),
            ]));
            new_ops.push(Operation::new("Tj", vec![
                Object::string_literal(value.as_str())
            ]));
            new_ops.push(Operation::new("ET", vec![]));
        }
    }

    let new_content = Content { operations: new_ops };
    let new_content_bytes = new_content.encode()?;

    // Add as a new stream object
    let new_stream = lopdf::Stream::new(
        lopdf::Dictionary::new(),
        new_content_bytes
    );
    let new_stream_id = doc.add_object(new_stream);

    // Append to page's Contents array
    let page_dict = doc.get_object_mut(page_id)?.as_dict_mut()?;
    match page_dict.get(b"Contents")? {
        Object::Reference(r) => {
            let existing_ref = *r;
            page_dict.set("Contents", Object::Array(vec![
                Object::Reference(existing_ref),
                Object::Reference(new_stream_id),
            ]));
        },
        Object::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(Object::Reference(new_stream_id));
            page_dict.set("Contents", Object::Array(new_arr));
        },
        _ => {}
    }

    // let new_content_bytes = content.encode()?;
    // doc.add_page_contents(page_id, new_content_bytes)?;

    // // Append the content to the page
    // doc.change_page_content(page_id, content.encode()?);
    //
    // Save new PDF
    let filename = format!("{}/{}", output_dir, render_output_name(&config.output_name, record));
    doc.save(&filename)?;

    // println!("Generated: {}", &filename);
    Ok(())
}
