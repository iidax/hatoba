use dialoguer::Select;

use crate::db::Db;
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    db_path: Option<PathBuf>,
    path1: Option<&str>,
    path2: Option<&str>,
    msg: &Msg,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::open(db_path)?;

    let (path1, path2) = match (path1, path2) {
        (Some(p1), Some(p2)) => (p1.to_string(), p2.to_string()),
        _ => interactive_select(&db, msg)?.ok_or("cancelled")?,
    };

    db.swap_directories(&path1, &path2)?;
    println!("{}: {path1} ↔ {path2}", msg.swapped);
    Ok(())
}

fn interactive_select(
    db: &Db,
    msg: &Msg,
) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
    let directories = db.list_directories()?;
    if directories.is_empty() {
        return Err("no directories registered. Use 'hatoba add <path>' to add one.".into());
    }
    if directories.len() < 2 {
        return Err("need at least 2 directories to swap".into());
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

    let sel1 = Select::new()
        .with_prompt(msg.swap_prompt_first)
        .items(&items)
        .interact_opt()?;

    let idx1 = match sel1 {
        Some(i) => i,
        None => return Ok(None),
    };

    // Second pass: first selected item blinks to indicate it is "held"
    let items2: Vec<String> = items
        .iter()
        .enumerate()
        .map(|(i, label)| {
            if i == idx1 {
                format!("\x1b[5m{label}\x1b[25m")
            } else {
                label.clone()
            }
        })
        .collect();

    let sel2 = Select::new()
        .with_prompt(msg.swap_prompt_second)
        .items(&items2)
        .interact_opt()?;

    let idx2 = match sel2 {
        Some(i) => i,
        None => return Ok(None),
    };

    if idx1 == idx2 {
        return Err("cannot swap a directory with itself".into());
    }

    Ok(Some((
        directories[idx1].path.clone(),
        directories[idx2].path.clone(),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;

    fn setup_db(paths: &[&str]) -> (PathBuf, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let mut db = Db::open(Some(db_path.clone())).unwrap();
        for path in paths {
            db.insert_directory(path, None, false).unwrap();
        }
        (db_path, dir)
    }

    #[test]
    fn swap_changes_order() {
        let (db_path, _dir) = setup_db(&["/tmp/a", "/tmp/b", "/tmp/c"]);
        run(
            Some(db_path.clone()),
            Some("/tmp/a"),
            Some("/tmp/c"),
            &crate::messages::EN,
        )
        .unwrap();
        let db = Db::open(Some(db_path)).unwrap();
        let directories = db.list_directories().unwrap();
        assert_eq!(directories[0].path, "/tmp/c");
        assert_eq!(directories[1].path, "/tmp/b");
        assert_eq!(directories[2].path, "/tmp/a");
    }

    #[test]
    fn swap_fails_same_path() {
        let (db_path, _dir) = setup_db(&["/tmp/a", "/tmp/b"]);
        let result = run(
            Some(db_path),
            Some("/tmp/a"),
            Some("/tmp/a"),
            &crate::messages::EN,
        );
        assert!(result.is_err());
    }

    #[test]
    fn swap_fails_when_path_not_found() {
        let (db_path, _dir) = setup_db(&["/tmp/a"]);
        let result = run(
            Some(db_path),
            Some("/tmp/nonexistent"),
            Some("/tmp/a"),
            &crate::messages::EN,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn swap_hints_trailing_slash() {
        let (db_path, _dir) = setup_db(&["/tmp/a", "/tmp/b"]);
        let result = run(
            Some(db_path),
            Some("/tmp/a/"),
            Some("/tmp/b"),
            &crate::messages::EN,
        );
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("hint:"));
    }
}
