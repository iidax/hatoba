use crate::config::config_path_default;
use std::path::PathBuf;

pub fn run(
    config_path: Option<PathBuf>,
    path: &str,
    label: Option<String>,
    default: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = config_path.map(Ok).unwrap_or_else(config_path_default)?;
    let content = std::fs::read_to_string(&file_path)?;
    let mut doc = content.parse::<toml_edit::DocumentMut>()?;

    if let Some(dirs) = doc["dirs"].as_array_of_tables() {
        if dirs.iter().any(|d| d["path"].as_str() == Some(path)) {
            return Err(format!("already exists: {path}").into());
        }
    }

    if default {
        if let Some(dirs) = doc["dirs"].as_array_of_tables_mut() {
            for dir in dirs.iter_mut() {
                dir["default"] = toml_edit::value(false);
            }
        }
    }

    let mut table = toml_edit::Table::new();
    table["path"] = toml_edit::value(path);
    if let Some(l) = label {
        table["label"] = toml_edit::value(l);
    }
    if default {
        table["default"] = toml_edit::value(true);
    }
    doc["dirs"]
        .as_array_of_tables_mut()
        .ok_or("dirs is not an array of tables")?
        .push(table);

    std::fs::write(&file_path, doc.to_string())?;
    Ok(())
}
