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

    let selection = Select::new()
        .with_prompt("hatoba: 作業ディレクトリを選択")
        .items(&items)
        .default(default_idx)
        .interact_opt()?;

    Ok(selection.map(|i| dirs[i].path.clone()))
}
