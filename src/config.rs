use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub dirs: Vec<Dir>,
}

#[derive(Deserialize)]
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

fn config_path_default() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("cannot determine home directory")?;
    Ok(home.join(".config").join("hatoba").join("config.toml"))
}
