use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub default: Option<String>,
    #[serde(default)]
    pub dirs: Vec<Dir>,
}

#[derive(Deserialize)]
pub struct Dir {
    pub path: String,
    pub label: Option<String>,
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
    if let Some(default) = &config.default {
        config.default = Some(shellexpand::full(default)?.into_owned());
    }
    for dir in &mut config.dirs {
        dir.path = shellexpand::full(&dir.path)?.into_owned();
    }
    Ok(())
}

fn config_path_default() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("cannot determine config directory")?;
    Ok(config_dir.join("hatoba").join("config.toml"))
}
