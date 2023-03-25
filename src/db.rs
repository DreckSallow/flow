use std::path::{Path, PathBuf};

use rusqlite::{Connection, Error};

use crate::utils::directory::Directory;

pub struct Db(pub Connection);

impl Db {
    pub fn open() -> Result<Db, Error> {
        let mut local_db = Directory::get_local_dir("db").unwrap();
        local_db.push("flow.sqlite");
        Self::open_db(local_db)
    }

    #[allow(dead_code)]
    pub fn open_test_db() -> Result<Db, Error> {
        Self::open_db(Self::get_path_test_db())
    }

    /// Get the test-db path
    #[allow(dead_code)]
    pub fn get_path_test_db() -> PathBuf {
        let mut local_db = Directory::get_local_dir("test_db").unwrap();
        local_db.push("flow-test-db.sqlite");
        local_db
    }

    pub fn open_db<P>(local_db: P) -> Result<Db, Error>
    where
        P: AsRef<Path>,
    {
        let conn = Connection::open(local_db)?;
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS projects (
              id INTEGER PRIMARY KEY,
              path TEXT NOT NULL UNIQUE
            )
          ",
            (),
        )?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS tasks (
              id INTEGER PRIMARY KEY,
              description TEXT NOT NULL UNIQUE,
              date TEXT NOT NULL,
              project_id INTEGER NOT NULL references projects(id)
            )
          ",
            (),
        )?;

        Ok(Db(conn))
    }
}
