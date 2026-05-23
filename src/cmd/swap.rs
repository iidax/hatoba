use crate::db::Db;
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    db_path: Option<PathBuf>,
    path1: &str,
    path2: &str,
    msg: &Msg,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::open(db_path)?;
    db.swap_directories(path1, path2)?;
    println!("{}: {path1} ↔ {path2}", msg.swapped);
    Ok(())
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
            "/tmp/a",
            "/tmp/c",
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
        let result = run(Some(db_path), "/tmp/a", "/tmp/a", &crate::messages::EN);
        assert!(result.is_err());
    }

    #[test]
    fn swap_fails_when_path_not_found() {
        let (db_path, _dir) = setup_db(&["/tmp/a"]);
        let result = run(
            Some(db_path),
            "/tmp/nonexistent",
            "/tmp/a",
            &crate::messages::EN,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn swap_hints_trailing_slash() {
        let (db_path, _dir) = setup_db(&["/tmp/a", "/tmp/b"]);
        let result = run(Some(db_path), "/tmp/a/", "/tmp/b", &crate::messages::EN);
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("hint:"));
    }
}
