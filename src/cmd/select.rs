use dialoguer::Select;

use crate::config::Config;

pub fn run(config: &Config) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let dirs = &config.dirs;

    if dirs.len() == 1 {
        return Ok(Some(dirs[0].path.clone()));
    }

    let default_idx = dirs.iter().position(|dir| dir.default).unwrap_or(0);

    let items: Vec<String> = dirs
        .iter()
        .map(|dir| {
            if dir.default {
                format!("{}  (default)", dir.display())
            } else {
                dir.display().to_string()
            }
        })
        .collect();

    let current = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".to_string());

    let selection = Select::new()
        .with_prompt(format!(
            "hatoba: 作業ディレクトリを選択  (↑/↓ j/k で移動、Enter/Space で決定、Esc/q でキャンセル → {current})"
        ))
        .items(&items)
        .default(default_idx)
        .interact_opt()?;

    Ok(selection.map(|i| dirs[i].path.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, Dir};

    fn make_config(paths: &[&str]) -> Config {
        Config {
            dirs: paths
                .iter()
                .map(|p| Dir {
                    path: p.to_string(),
                    label: None,
                    default: false,
                })
                .collect(),
        }
    }

    #[test]
    fn run_returns_single_dir_without_interaction() {
        let config = make_config(&["/tmp/only"]);
        let result = run(&config).unwrap();
        assert_eq!(result, Some("/tmp/only".to_string()));
    }
}
