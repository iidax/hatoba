use crate::db::Db;
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    db_path: Option<PathBuf>,
    path: &str,
    msg: &Msg,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::open(db_path)?;
    db.remove_directory(path)?;
    println!("{}: {path}", msg.removed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;

    fn setup_db_with_entries(paths: &[&str]) -> (PathBuf, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let mut db = Db::open(Some(db_path.clone())).unwrap();
        for path in paths {
            db.insert_directory(path, None, false).unwrap();
        }
        (db_path, dir)
    }

    #[test]
    fn remove_deletes_entry() {
        let (db_path, _dir) = setup_db_with_entries(&["/tmp/a", "/tmp/b"]);
        run(Some(db_path.clone()), "/tmp/a", &crate::messages::EN).unwrap();
        let db = Db::open(Some(db_path)).unwrap();
        let directories = db.list_directories().unwrap();
        assert_eq!(directories.len(), 1);
        assert_eq!(directories[0].path, "/tmp/b");
    }

    #[test]
    fn remove_fails_when_path_not_found() {
        let (db_path, _dir) = setup_db_with_entries(&["/tmp/a"]);
        let result = run(Some(db_path), "/tmp/nonexistent", &crate::messages::EN);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn remove_hints_trailing_slash_difference() {
        let (db_path, _dir) = setup_db_with_entries(&["/tmp/a"]);
        let result = run(Some(db_path), "/tmp/a/", &crate::messages::EN);
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("hint:"));
        assert!(msg.contains("/tmp/a"));
    }
}
