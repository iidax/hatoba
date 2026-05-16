use crate::config::config_path_default;
use std::path::PathBuf;

pub fn run(
    config_path: Option<PathBuf>,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = config_path.map(Ok).unwrap_or_else(config_path_default)?;
    let content = std::fs::read_to_string(&file_path)?;
    let mut doc = content.parse::<toml_edit::DocumentMut>()?;

    let dirs = doc["dirs"]
        .as_array_of_tables_mut()
        .ok_or("dirs is not an array of tables")?;

    let idx = dirs
        .iter()
        .position(|d| d["path"].as_str() == Some(path))
        .ok_or_else(|| format!("not found: {path}"))?;

    dirs.remove(idx);

    std::fs::write(&file_path, doc.to_string())?;
    Ok(())
}
