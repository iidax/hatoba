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

#[derive(Deserialize, Debug, Default)]
pub struct Settings {
    #[serde(default)]
    pub language: Language,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub dirs: Vec<Dir>,
    #[serde(default)]
    pub settings: Settings,
}

#[derive(Deserialize, Debug)]
pub struct Dir {
    pub path: String,
    pub label: Option<String>,
    #[serde(default)]
    pub default: bool,
}

impl Dir {
    pub fn display(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.path)
    }
}

pub fn load(config_path: Option<std::path::PathBuf>) -> Result<Config, Box<dyn std::error::Error>> {
    let path = match config_path {
        Some(p) => p,
        None => config_path_default()?,
    };
    if !path.exists() {
        let dir = path.parent().unwrap().display().to_string();
        return Err(format!(
            "config file not found: {}\nhint: mkdir -p {dir} && $EDITOR {}/config.toml",
            path.display(),
            dir,
        )
        .into());
    }
    let content = std::fs::read_to_string(&path)?;
    let mut config: Config = toml::from_str(&content)?;
    expand_paths(&mut config)?;
    Ok(config)
}

fn expand_paths(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    for dir in &mut config.dirs {
        dir.path = shellexpand::full(&dir.path)?.into_owned();
    }
    Ok(())
}

pub fn load_language(config_path: Option<PathBuf>) -> Language {
    let path = match config_path {
        Some(p) => p,
        None => match config_path_default() {
            Ok(p) => p,
            Err(_) => return Language::default(),
        },
    };
    if !path.exists() {
        return Language::default();
    }
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|c| toml::from_str::<Config>(&c).ok())
        .map(|c| c.settings.language)
        .unwrap_or_default()
}

pub fn config_path_default() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("cannot determine home directory")?;
    Ok(home.join(".config").join("hatoba").join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;

    #[test]
    fn display_returns_label_when_present() {
        let dir = Dir {
            path: "/home/user".to_string(),
            label: Some("myproject".to_string()),
            default: false,
        };
        assert_eq!(dir.display(), "myproject");
    }

    #[test]
    fn display_returns_path_when_no_label() {
        let dir = Dir {
            path: "/home/user".to_string(),
            label: None,
            default: false,
        };
        assert_eq!(dir.display(), "/home/user");
    }

    #[test]
    fn load_returns_error_when_file_missing() {
        let result = load(Some(PathBuf::from("/nonexistent/path/config.toml")));
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("config file not found"));
        assert!(msg.contains("hint:"));
    }

    #[test]
    fn load_parses_valid_toml() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[[dirs]]
path = "/tmp/foo"
label = "foo"
default = true
"#
        )
        .unwrap();

        let config = load(Some(file.path().to_path_buf())).unwrap();
        assert_eq!(config.dirs.len(), 1);
        assert_eq!(config.dirs[0].path, "/tmp/foo");
        assert_eq!(config.dirs[0].label, Some("foo".to_string()));
        assert!(config.dirs[0].default);
    }

    #[test]
    fn load_expands_home_variable() {
        let home = dirs::home_dir().unwrap();
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
[[dirs]]
path = "$HOME/workspace"
"#
        )
        .unwrap();

        let config = load(Some(file.path().to_path_buf())).unwrap();
        let expected = format!("{}/workspace", home.display());
        assert_eq!(config.dirs[0].path, expected);
    }
}
