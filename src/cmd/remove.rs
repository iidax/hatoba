use crate::config::config_path_default;
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    config_path: Option<PathBuf>,
    path: &str,
    msg: &Msg,
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
        .ok_or_else(|| not_found_error(dirs, path))?;

    dirs.remove(idx);

    std::fs::write(&file_path, doc.to_string())?;
    println!("{}: {path}", msg.removed);
    Ok(())
}

fn not_found_error(dirs: &toml_edit::ArrayOfTables, path: &str) -> Box<dyn std::error::Error> {
    let alt = if path.ends_with('/') {
        path.trim_end_matches('/').to_string()
    } else {
        format!("{path}/")
    };
    if dirs
        .iter()
        .any(|d| d["path"].as_str() == Some(alt.as_str()))
    {
        format!("not found: {path}\nhint: did you mean '{alt}'?").into()
    } else {
        format!("not found: {path}").into()
    }
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
        run(
            Some(file.path().to_path_buf()),
            "/tmp/a",
            &crate::messages::EN,
        )
        .unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(config.dirs.len(), 1);
        assert_eq!(config.dirs[0].path, "/tmp/b");
    }

    #[test]
    fn remove_fails_when_path_not_found() {
        let file = make_config_file("[[dirs]]\npath = \"/tmp/a\"\n");
        let result = run(
            Some(file.path().to_path_buf()),
            "/tmp/nonexistent",
            &crate::messages::EN,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn remove_hints_trailing_slash_difference() {
        let file = make_config_file("[[dirs]]\npath = \"/tmp/a\"\n");
        let result = run(
            Some(file.path().to_path_buf()),
            "/tmp/a/",
            &crate::messages::EN,
        );
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("hint:"));
        assert!(msg.contains("/tmp/a"));
    }
}
