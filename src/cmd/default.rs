use dialoguer::Select;

use crate::db::Db;
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    db_path: Option<PathBuf>,
    path: Option<&str>,
    msg: &Msg,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::open(db_path)?;

    let path = match path {
        Some(p) => p.to_string(),
        None => {
            let directories = db.list_directories()?;
            if directories.is_empty() {
                return Err(
                    "no directories registered. Use 'hatoba add <path>' to add one.".into(),
                );
            }
            let items: Vec<String> = directories
                .iter()
                .map(|d| {
                    if d.default {
                        format!("{}{}", d.display(), msg.default_marker)
                    } else {
                        d.display().to_string()
                    }
                })
                .collect();
            let default_idx = directories.iter().position(|d| d.default).unwrap_or(0);
            let selection = Select::new()
                .with_prompt(msg.default_prompt)
                .items(&items)
                .default(default_idx)
                .interact_opt()?;
            match selection {
                Some(i) => directories[i].path.clone(),
                None => return Ok(()),
            }
        }
    };

    db.set_default(&path)?;
    println!("{}: {path}", msg.default_set);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;

    fn setup_db_with_entries(entries: &[(&str, bool)]) -> (PathBuf, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let mut db = Db::open(Some(db_path.clone())).unwrap();
        for (path, default) in entries {
            db.insert_directory(path, None, *default).unwrap();
        }
        (db_path, dir)
    }

    #[test]
    fn default_sets_target_and_clears_others() {
        let (db_path, _dir) = setup_db_with_entries(&[("/tmp/a", true), ("/tmp/b", false)]);
        run(Some(db_path.clone()), Some("/tmp/b"), &crate::messages::EN).unwrap();
        let db = Db::open(Some(db_path)).unwrap();
        let directories = db.list_directories().unwrap();
        assert!(!directories[0].default);
        assert!(directories[1].default);
    }

    #[test]
    fn default_fails_when_path_not_found() {
        let (db_path, _dir) = setup_db_with_entries(&[("/tmp/a", false)]);
        let result = run(
            Some(db_path),
            Some("/tmp/nonexistent"),
            &crate::messages::EN,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn default_hints_trailing_slash_difference() {
        let (db_path, _dir) = setup_db_with_entries(&[("/tmp/a", false)]);
        let result = run(Some(db_path), Some("/tmp/a/"), &crate::messages::EN);
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("hint:"));
        assert!(msg.contains("/tmp/a"));
    }
}
