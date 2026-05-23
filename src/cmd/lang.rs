use crate::config::{Language, config_path_default};
use std::path::PathBuf;

pub fn run(
    config_path: Option<PathBuf>,
    lang: Option<Language>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = config_path.map(Ok).unwrap_or_else(config_path_default)?;

    match lang {
        None => {
            if !file_path.exists() {
                println!("en");
                return Ok(());
            }
            let content = std::fs::read_to_string(&file_path)?;
            let config: crate::config::Config = toml::from_str(&content)?;
            println!("{}", config.settings.language);
        }
        Some(new_lang) => {
            if !file_path.exists() {
                if let Some(dir) = file_path.parent() {
                    std::fs::create_dir_all(dir)?;
                }
                std::fs::write(&file_path, "")?;
            }
            let content = std::fs::read_to_string(&file_path)?;
            let mut doc = content.parse::<toml_edit::DocumentMut>()?;

            let lang_str = new_lang.to_string();
            doc["settings"]["language"] = toml_edit::value(lang_str.as_str());
            std::fs::write(&file_path, doc.to_string())?;
            println!("{new_lang}");
        }
    }
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
    fn set_language_writes_to_config() {
        let file = make_config_file("");
        run(Some(file.path().to_path_buf()), Some(Language::Ja)).unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(config.settings.language, Language::Ja);
    }

    #[test]
    fn set_language_back_to_en() {
        let file = make_config_file("[settings]\nlanguage = \"ja\"\n");
        run(Some(file.path().to_path_buf()), Some(Language::En)).unwrap();
        let config = crate::config::load(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(config.settings.language, Language::En);
    }

    #[test]
    fn show_returns_default_when_no_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        // file doesn't exist → should print "en" without error
        run(Some(path), None).unwrap();
    }
}
