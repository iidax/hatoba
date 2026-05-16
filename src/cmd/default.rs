use crate::config::config_path_default;
use std::path::PathBuf;

pub fn run(config_path: Option<PathBuf>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    fn default_sets_target_and_clears_others() {
        let file = make_config_file(
            r#"
[[dirs]]
path = "/tmp/a"
default = true

[[dirs]]
path = "/tmp/b"
"#,
        );
        run(Some(file.path().to_path_buf()), "/tmp/b").unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert!(!config.dirs[0].default);
        assert!(config.dirs[1].default);
    }

    #[test]
    fn default_fails_when_path_not_found() {
        let file = make_config_file("[[dirs]]\npath = \"/tmp/a\"\n");
        let result = run(Some(file.path().to_path_buf()), "/tmp/nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
