use dialoguer::Select;

use crate::config::Directory;
use crate::messages::Msg;

pub fn run(
    directories: &[Directory],
    msg: &Msg,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if directories.len() == 1 {
        return Ok(Some(directories[0].path.clone()));
    }

    let default_idx = directories.iter().position(|dir| dir.default).unwrap_or(0);

    let items: Vec<String> = directories
        .iter()
        .map(|dir| {
            if dir.default {
                format!("{}{}", dir.display(), msg.default_marker)
            } else {
                dir.display().to_string()
            }
        })
        .collect();

    let current = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".to_string());

    let selection = Select::new()
        .with_prompt(format!("{} → {current})", msg.select_prompt))
        .items(&items)
        .default(default_idx)
        .interact_opt()?;

    Ok(selection.map(|i| directories[i].path.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages;

    fn make_directories(paths: &[&str]) -> Vec<Directory> {
        paths
            .iter()
            .map(|p| Directory {
                path: p.to_string(),
                label: None,
                default: false,
            })
            .collect()
    }

    #[test]
    fn run_returns_single_dir_without_interaction() {
        let directories = make_directories(&["/tmp/only"]);
        let result = run(&directories, &messages::EN).unwrap();
        assert_eq!(result, Some("/tmp/only".to_string()));
    }
}
