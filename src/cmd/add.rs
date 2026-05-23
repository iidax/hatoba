use crate::db::Db;
use crate::messages::Msg;
use std::path::PathBuf;

pub fn run(
    db_path: Option<PathBuf>,
    path: &str,
    label: Option<String>,
    default: bool,
    msg: &Msg,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = Db::open(db_path)?;
    if db.path_exists(path)? {
        return Err(format!("already exists: {path}").into());
    }
    db.insert_directory(path, label.as_deref(), default)?;
    println!("{}: {path}", msg.added);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Db;

    fn open_temp_db() -> (PathBuf, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.db");
        (path, dir)
    }

    #[test]
    fn add_creates_db_when_missing() {
        let (db_path, _dir) = open_temp_db();
        assert!(!db_path.exists());
        run(
            Some(db_path.clone()),
            "/tmp/new",
            None,
            false,
            &crate::messages::EN,
        )
        .unwrap();
        assert!(db_path.exists());
        let db = Db::open(Some(db_path)).unwrap();
        assert_eq!(db.list_directories().unwrap().len(), 1);
    }

    #[test]
    fn add_appends_new_entry() {
        let (db_path, _dir) = open_temp_db();
        run(
            Some(db_path.clone()),
            "/tmp/a",
            None,
            false,
            &crate::messages::EN,
        )
        .unwrap();
        run(
            Some(db_path.clone()),
            "/tmp/b",
            None,
            false,
            &crate::messages::EN,
        )
        .unwrap();
        let db = Db::open(Some(db_path)).unwrap();
        assert_eq!(db.list_directories().unwrap().len(), 2);
    }

    #[test]
    fn add_with_label() {
        let (db_path, _dir) = open_temp_db();
        run(
            Some(db_path.clone()),
            "/tmp/a",
            Some("alpha".to_string()),
            false,
            &crate::messages::EN,
        )
        .unwrap();
        let db = Db::open(Some(db_path)).unwrap();
        assert_eq!(
            db.list_directories().unwrap()[0].label,
            Some("alpha".to_string())
        );
    }

    #[test]
    fn add_with_default_clears_existing_defaults() {
        let (db_path, _dir) = open_temp_db();
        run(
            Some(db_path.clone()),
            "/tmp/a",
            None,
            true,
            &crate::messages::EN,
        )
        .unwrap();
        run(
            Some(db_path.clone()),
            "/tmp/b",
            None,
            true,
            &crate::messages::EN,
        )
        .unwrap();
        let db = Db::open(Some(db_path)).unwrap();
        let directories = db.list_directories().unwrap();
        assert!(!directories[0].default);
        assert!(directories[1].default);
    }

    #[test]
    fn add_fails_on_duplicate_path() {
        let (db_path, _dir) = open_temp_db();
        run(
            Some(db_path.clone()),
            "/tmp/a",
            None,
            false,
            &crate::messages::EN,
        )
        .unwrap();
        let result = run(Some(db_path), "/tmp/a", None, false, &crate::messages::EN);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
}
