use dialoguer::Select;

use crate::config::Dir;
use crate::messages::Msg;

pub fn run(dirs: &[Dir], msg: &Msg) -> Result<Option<String>, Box<dyn std::error::Error>> {
    if dirs.len() == 1 {
        return Ok(Some(dirs[0].path.clone()));
    }

    let default_idx = dirs.iter().position(|dir| dir.default).unwrap_or(0);

    let items: Vec<String> = dirs
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

    Ok(selection.map(|i| dirs[i].path.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages;

    fn make_dirs(paths: &[&str]) -> Vec<Dir> {
        paths
            .iter()
            .map(|p| Dir {
                path: p.to_string(),
                label: None,
                default: false,
            })
            .collect()
    }

    #[test]
    fn run_returns_single_dir_without_interaction() {
        let dirs = make_dirs(&["/tmp/only"]);
        let result = run(&dirs, &messages::EN).unwrap();
        assert_eq!(result, Some("/tmp/only".to_string()));
    }
}
