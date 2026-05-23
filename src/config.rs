use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Ja,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::En => write!(f, "en"),
            Language::Ja => write!(f, "ja"),
        }
    }
}

#[derive(Default)]
pub struct Config {
    pub language: Language,
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Self {
        let file_path = match path {
            Some(p) => p,
            None => match Self::path_default() {
                Ok(p) => p,
                Err(_) => return Self::default(),
            },
        };
        if !file_path.exists() {
            return Self::default();
        }
        #[derive(Deserialize, Default)]
        struct ConfigFile {
            #[serde(default)]
            settings: SettingsSection,
        }
        #[derive(Deserialize, Default)]
        struct SettingsSection {
            #[serde(default)]
            language: Language,
        }
        std::fs::read_to_string(&file_path)
            .ok()
            .and_then(|c| toml::from_str::<ConfigFile>(&c).ok())
            .map(|f| Config {
                language: f.settings.language,
            })
            .unwrap_or_default()
    }

    pub fn path_default() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("cannot determine home directory")?;
        Ok(home.join(".config").join("hatoba").join("config.toml"))
    }
}

#[derive(Debug)]
pub struct Directory {
    pub path: String,
    pub label: Option<String>,
    pub default: bool,
}

impl Directory {
    pub fn display(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;

    #[test]
    fn display_returns_label_when_present() {
        let dir = Directory {
            path: "/home/user".to_string(),
            label: Some("myproject".to_string()),
            default: false,
        };
        assert_eq!(dir.display(), "myproject");
    }

    #[test]
    fn display_returns_path_when_no_label() {
        let dir = Directory {
            path: "/home/user".to_string(),
            label: None,
            default: false,
        };
        assert_eq!(dir.display(), "/home/user");
    }

    #[test]
    fn load_returns_default_when_file_missing() {
        let config = Config::load(Some(PathBuf::from("/nonexistent/path/config.toml")));
        assert_eq!(config.language, Language::En);
    }

    #[test]
    fn load_parses_language() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "[settings]\nlanguage = \"ja\"").unwrap();
        let config = Config::load(Some(file.path().to_path_buf()));
        assert_eq!(config.language, Language::Ja);
    }
}
