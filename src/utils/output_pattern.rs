use std::collections::HashMap;

pub fn render_output_name(pattern: &str, record: &HashMap<String, String>) -> String {
    let mut filename = pattern.to_string();
    for (key, value) in record {
        let placeholder = format!("{{{}}}", key);
        filename = filename.replace(&placeholder, value);
    }

    // Remove illegal filename characters ---------------------------
    filename = filename.replace("/", "_");
    filename
}
