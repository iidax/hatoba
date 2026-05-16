use crate::config::config_path_default;
use std::path::PathBuf;

pub fn run(
    config_path: Option<PathBuf>,
    path: &str,
    label: Option<String>,
    default: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = config_path.map(Ok).unwrap_or_else(config_path_default)?;
    if !file_path.exists() {
        if let Some(dir) = file_path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(&file_path, "")?;
    }
    let content = std::fs::read_to_string(&file_path)?;
    let mut doc = content.parse::<toml_edit::DocumentMut>()?;

    if let Some(dirs) = doc.get("dirs").and_then(|v| v.as_array_of_tables())
        && dirs.iter().any(|d| d["path"].as_str() == Some(path))
    {
        return Err(format!("already exists: {path}").into());
    }

    if default && let Some(dirs) = doc.get_mut("dirs").and_then(|v| v.as_array_of_tables_mut()) {
        for dir in dirs.iter_mut() {
            dir["default"] = toml_edit::value(false);
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
    doc.entry("dirs")
        .or_insert(toml_edit::Item::ArrayOfTables(
            toml_edit::ArrayOfTables::new(),
        ))
        .as_array_of_tables_mut()
        .ok_or("dirs is not an array of tables")?
        .push(table);

    std::fs::write(&file_path, doc.to_string())?;
    println!("Added: {path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;

    fn make_config_file(content: &str) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "{content}").unwrap();
        file
    }

    #[test]
    fn add_creates_file_when_missing() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("config.toml");
        assert!(!file_path.exists());
        run(Some(file_path.clone()), "/tmp/new", None, false).unwrap();
        assert!(file_path.exists());
        let config = crate::config::load(Some(file_path)).unwrap();
        assert_eq!(config.dirs.len(), 1);
        assert_eq!(config.dirs[0].path, "/tmp/new");
    }

    #[test]
    fn add_appends_new_entry() {
        let file = make_config_file(
            r#"
[[dirs]]
path = "/tmp/existing"
"#,
        );
        run(Some(file.path().to_path_buf()), "/tmp/new", None, false).unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(config.dirs.len(), 2);
        assert_eq!(config.dirs[1].path, "/tmp/new");
    }

    #[test]
    fn add_with_label() {
        let file = make_config_file("[[dirs]]\npath = \"/tmp/a\"\n");
        run(
            Some(file.path().to_path_buf()),
            "/tmp/b",
            Some("B".to_string()),
            false,
        )
        .unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(config.dirs[1].label, Some("B".to_string()));
    }

    #[test]
    fn add_with_default_clears_existing_defaults() {
        let file = make_config_file("[[dirs]]\npath = \"/tmp/a\"\ndefault = true\n");
        run(Some(file.path().to_path_buf()), "/tmp/b", None, true).unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert!(!config.dirs[0].default);
        assert!(config.dirs[1].default);
    }

    #[test]
    fn add_fails_on_duplicate_path() {
        let file = make_config_file("[[dirs]]\npath = \"/tmp/a\"\n");
        let result = run(Some(file.path().to_path_buf()), "/tmp/a", None, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
}
