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

    if !dirs.iter().any(|d| d["path"].as_str() == Some(path)) {
        return Err(format!("not found: {path}").into());
    }

    for dir in dirs.iter_mut() {
        let is_target = dir["path"].as_str() == Some(path);
        dir["default"] = toml_edit::value(is_target);
    }

    std::fs::write(&file_path, doc.to_string())?;
    Ok(())
}
