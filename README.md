# Recred

[![Crates.io](https://img.shields.io/crates/v/recred)](https://crates.io/crates/recred)
[![License](https://img.shields.io/crates/l/recred)](https://github.com/aether-flux/recred/blob/main/LICENSE)

**Recred** is a CLI tool written in Rust for generating certificates in bulk from PDF templates.
It reads participant data from a CSV file and automatically places the fields like name, achievement, etc at configurable positions on your certificate template. Text color, font size, and output filenames can be customized per field or globally.

---

## Background
This project is a remake of [AutoCred](https://github.com/Vikash2984/devsoc-autocred) a similar Python tool.
AutoCred became unusable due to a critical dependency crash.
Recred brings the same functionality to Rust, with even more features like a json file for configuring the certificate generation.

---

## Features

- Bulk PDF certificate generation from a template PDF.
- Supports CSV input for participant data.
- Configurable field positions (`x`, `y`) for precise placement.
- Optional per-field **text color** and **font size**.
- Global defaults for color and font size if not specified per field.
- Flexible output filenames using patterns, e.g., `{name}.pdf`.
- Fully CLI-based, zero runtime dependencies beyond Rust.

---

## Installation

If you have **cargo** installed on your system, simply run:
```bash
cargo install recred
```

Otherwise, follow the next steps depending on your OS.

### Linux
```bash
curl -fsSL https://raw.githubusercontent.com/aether-flux/recred/main/scripts/linux/install.sh | bash
```

### Windows
```bash
curl -L https://raw.githubusercontent.com/aether-flux/recred/main/scripts/windows/install.bat -o install.bat && install.bat
```

---

## Configuration
Recred uses a JSON config file to define:
- Template PDF
- Output filename pattern
- Global text color and font size
- Field positions, optional field-specific font color and size

#### Example `config.json`:
```json
{
  "template": "template.pdf",
  "font_size": 30,
  "text_color": [0, 0, 0],
  "output_name": "{name}.pdf",
  "fields": {
    "name": { "x": 220.0, "y": 350.0, "color": [34, 139, 34] },
    "achievement": { "x": 220.0, "y": 300.0, "size": 20 }
  }
}
```

- `output_name`: pattern for PDF filenames. `{name}` will be replaced with the CSV field "name".
- `fields`: define `x` and `y` coordinates and optional `color` and `size` of field-specific text.

**Note**: There is no restriction on the name of the JSON config file, as it needs to be specified manually when running the CLI. Details on the CLI usage are provided below.

---

## Usage
```bash
recred generate --config config.json --data data.csv
```

- `--config or -c`: path to your JSON config file.
- `--data or -d`: path to your CSV file with participant data.
Generated PDFs will be saved in the `output/` directory.

---

## License
Recred is licensed under MIT.

---

Made with 🦀 in Rust.

