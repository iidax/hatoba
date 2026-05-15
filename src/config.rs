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

pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path()?;
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
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("cannot determine config directory")?;
    Ok(config_dir.join("hatoba").join("config.toml"))
}
