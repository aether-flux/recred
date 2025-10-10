use std::collections::HashMap;
use anyhow::Result;

pub fn read_csv(path: &str) -> Result<Vec<HashMap<String, String>>> {
    let mut reader = csv::Reader::from_path(path)?;
    let headers = reader.headers()?.clone();

    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut map = HashMap::new();

        for (i, field) in record.iter().enumerate() {
            map.insert(headers[i].to_string(), field.to_string());
        }

        records.push(map);
    }

    Ok(records)
}
