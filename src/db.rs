use crate::config::Dir;
use include_dir::{Dir as IncludeDir, include_dir};
use rusqlite::{Connection, params};
use rusqlite_migration::Migrations;
use std::path::PathBuf;

static MIGRATIONS_DIR: IncludeDir<'_> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open(path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = match path {
            Some(p) => p,
            None => db_path_default()?,
        };
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut conn = Connection::open(&db_path)?;
        let migrations = Migrations::from_directory(&MIGRATIONS_DIR)?;
        migrations.to_latest(&mut conn)?;
        Ok(Self { conn })
    }

    pub fn list_dirs(&self) -> Result<Vec<Dir>, Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("SELECT path, label, is_default FROM dirs ORDER BY position, id")?;
        let dirs = stmt
            .query_map([], |row| {
                Ok(Dir {
                    path: row.get(0)?,
                    label: row.get(1)?,
                    default: row.get::<_, bool>(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(dirs)
    }

    pub fn path_exists(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM dirs WHERE path = ?1",
            params![path],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn all_paths(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("SELECT path FROM dirs ORDER BY position, id")?;
        let paths = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(paths)
    }

    pub fn insert_dir(
        &mut self,
        path: &str,
        label: Option<&str>,
        default: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let position: i64 = self.conn.query_row(
            "SELECT COALESCE(MAX(position), -1) + 1 FROM dirs",
            [],
            |row| row.get(0),
        )?;
        if default {
            self.conn
                .execute("UPDATE dirs SET is_default = ?1", params![false])?;
        }
        self.conn.execute(
            "INSERT INTO dirs (path, label, position, is_default) VALUES (?1, ?2, ?3, ?4)",
            params![path, label, position, default],
        )?;
        Ok(())
    }

    pub fn remove_dir(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rows = self
            .conn
            .execute("DELETE FROM dirs WHERE path = ?1", params![path])?;
        if rows == 0 {
            return Err(not_found_error(&self.all_paths()?, path));
        }
        Ok(())
    }

    pub fn set_default(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.path_exists(path)? {
            return Err(not_found_error(&self.all_paths()?, path));
        }
        self.conn
            .execute("UPDATE dirs SET is_default = ?1", params![false])?;
        self.conn.execute(
            "UPDATE dirs SET is_default = ?1 WHERE path = ?2",
            params![true, path],
        )?;
        Ok(())
    }

    fn get_id_and_position(&self, path: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
        self.conn
            .query_row(
                "SELECT id, position FROM dirs WHERE path = ?1",
                params![path],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(|_| not_found_error(&self.all_paths().unwrap_or_default(), path))
    }

    pub fn swap_dirs(&mut self, path1: &str, path2: &str) -> Result<(), Box<dyn std::error::Error>> {
        if path1 == path2 {
            return Err("cannot swap a directory with itself".into());
        }
        let (id1, pos1) = self.get_id_and_position(path1)?;
        let (id2, pos2) = self.get_id_and_position(path2)?;
        let tx = self.conn.transaction()?;
        tx.execute("UPDATE dirs SET position = ?1 WHERE id = ?2", params![pos2, id1])?;
        tx.execute("UPDATE dirs SET position = ?1 WHERE id = ?2", params![pos1, id2])?;
        tx.commit()?;
        Ok(())
    }
}

pub fn db_path_default() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("cannot determine home directory")?;
    Ok(home
        .join(".local")
        .join("share")
        .join("hatoba")
        .join("hatoba.db"))
}

fn not_found_error(paths: &[String], path: &str) -> Box<dyn std::error::Error> {
    let alt = if path.ends_with('/') {
        path.trim_end_matches('/').to_string()
    } else {
        format!("{path}/")
    };
    if paths.iter().any(|p| p == &alt) {
        format!("not found: {path}\nhint: did you mean '{alt}'?").into()
    } else {
        format!("not found: {path}").into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn open_temp_db() -> (Db, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let db = Db::open(Some(dir.path().join("test.db"))).unwrap();
        (db, dir)
    }

    #[test]
    fn list_dirs_empty_when_new() {
        let (db, _dir) = open_temp_db();
        assert!(db.list_dirs().unwrap().is_empty());
    }

    #[test]
    fn insert_and_list() {
        let (mut db, _dir) = open_temp_db();
        db.insert_dir("/tmp/a", Some("alpha"), false).unwrap();
        let dirs = db.list_dirs().unwrap();
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].path, "/tmp/a");
        assert_eq!(dirs[0].label, Some("alpha".to_string()));
        assert!(!dirs[0].default);
    }

    #[test]
    fn insert_with_default_clears_others() {
        let (mut db, _dir) = open_temp_db();
        db.insert_dir("/tmp/a", None, true).unwrap();
        db.insert_dir("/tmp/b", None, true).unwrap();
        let dirs = db.list_dirs().unwrap();
        assert!(!dirs[0].default);
        assert!(dirs[1].default);
    }

    #[test]
    fn insert_fails_on_duplicate() {
        let (mut db, _dir) = open_temp_db();
        db.insert_dir("/tmp/a", None, false).unwrap();
        assert!(db.insert_dir("/tmp/a", None, false).is_err());
    }

    #[test]
    fn remove_deletes_entry() {
        let (mut db, _dir) = open_temp_db();
        db.insert_dir("/tmp/a", None, false).unwrap();
        db.insert_dir("/tmp/b", None, false).unwrap();
        db.remove_dir("/tmp/a").unwrap();
        let dirs = db.list_dirs().unwrap();
        assert_eq!(dirs.len(), 1);
        assert_eq!(dirs[0].path, "/tmp/b");
    }

    #[test]
    fn remove_fails_when_not_found() {
        let (mut db, _dir) = open_temp_db();
        let err = db.remove_dir("/tmp/nonexistent").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn remove_hints_trailing_slash() {
        let (mut db, _dir) = open_temp_db();
        db.insert_dir("/tmp/a", None, false).unwrap();
        let err = db.remove_dir("/tmp/a/").unwrap_err();
        assert!(err.to_string().contains("hint:"));
    }

    #[test]
    fn set_default_updates_correctly() {
        let (mut db, _dir) = open_temp_db();
        db.insert_dir("/tmp/a", None, true).unwrap();
        db.insert_dir("/tmp/b", None, false).unwrap();
        db.set_default("/tmp/b").unwrap();
        let dirs = db.list_dirs().unwrap();
        assert!(!dirs[0].default);
        assert!(dirs[1].default);
    }

    #[test]
    fn set_default_fails_when_not_found() {
        let (mut db, _dir) = open_temp_db();
        assert!(db.set_default("/tmp/nonexistent").is_err());
    }
}
