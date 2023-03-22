use rusqlite::{Connection, Error};

use crate::utils::directory::Directory;

pub struct Db(pub Connection);

impl Db {
    pub fn open() -> Result<Db, Error> {
        let mut local_db = Directory::get_local_dir("db").unwrap();
        local_db.push("flow.sqlite");
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
