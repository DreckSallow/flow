use rusqlite::{Connection, Error};
// TODO: to do..

pub struct Db(pub Connection);

impl Db {
    pub fn open() -> Result<Db, Error> {
        let conn = Connection::open("projects.sqlite")?;
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS projects (
              id INTEGER PRIMARY KEY,
              path TEXT NOT NULL UNIQUE,
              date TEXT NOT NULL,
            )
          ",
            (),
        )?;
        Ok(Db(conn))
    }
}
