use crate::config::config_path_default;
use std::path::PathBuf;

pub fn run(config_path: Option<PathBuf>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    fn remove_deletes_entry() {
        let file = make_config_file(
            r#"
[[dirs]]
path = "/tmp/a"

[[dirs]]
path = "/tmp/b"
"#,
        );
        run(Some(file.path().to_path_buf()), "/tmp/a").unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(config.dirs.len(), 1);
        assert_eq!(config.dirs[0].path, "/tmp/b");
    }

    #[test]
    fn remove_fails_when_path_not_found() {
        let file = make_config_file("[[dirs]]\npath = \"/tmp/a\"\n");
        let result = run(Some(file.path().to_path_buf()), "/tmp/nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
