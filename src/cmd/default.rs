use crate::db::Db;
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    db_path: Option<PathBuf>,
    path: &str,
    msg: &Msg,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::open(db_path)?;
    db.set_default(path)?;
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
        run(Some(db_path.clone()), "/tmp/b", &crate::messages::EN).unwrap();
        let db = Db::open(Some(db_path)).unwrap();
        let directories = db.list_directories().unwrap();
        assert!(!directories[0].default);
        assert!(directories[1].default);
    }

    #[test]
    fn default_fails_when_path_not_found() {
        let (db_path, _dir) = setup_db_with_entries(&[("/tmp/a", false)]);
        let result = run(Some(db_path), "/tmp/nonexistent", &crate::messages::EN);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn default_hints_trailing_slash_difference() {
        let (db_path, _dir) = setup_db_with_entries(&[("/tmp/a", false)]);
        let result = run(Some(db_path), "/tmp/a/", &crate::messages::EN);
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("hint:"));
        assert!(msg.contains("/tmp/a"));
    }
}
